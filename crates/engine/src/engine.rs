//! The engine: commands in, events out (plan section 4).
//!
//! [`Engine::apply`] takes the game state, a [`Command`] and a dice source.
//! It changes the state and returns what happened as [`Event`]s. Where the
//! rules ask a person to decide (push or not, Overkill, who heard the
//! scream), the engine stores a [`Pending`] decision and waits for a
//! [`Command::Decide`]. A failed command leaves the state untouched.

use std::collections::VecDeque;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::active::{self, ActiveEffect, Modification, RollMatch};
use crate::data::effects::{
    Blocked, Cold, Effect, EndEvent, Lasts, Op, ReplaceWith, Replaceable, RollSpec, TriggerEvent,
    Unit, UsePeriod, Who,
};
use crate::data::model::{
    Condition, CriticalInjury, Healing, PanicResult, RollRange, RulingSwitch, Skill, Talent, Trauma,
};
use crate::data::{Id, RuleData};
use crate::dice::{Dice, Die, throw};
use crate::panic::{PanicCause, StopReason};
use crate::roll::{Outcome, RollRequest};
use crate::state::{
    ActiveSource, Actor, ActorId, Ending, GameState, Mode, Origin, Pending, PendingId, PlayerId,
    Supply,
};

/// The rule engine. It owns the rule data and holds no game state.
#[derive(Debug, Clone)]
pub struct Engine {
    pub(crate) rules: RuleData,
}

/// Something the game master or a player wants to happen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Command {
    /// Any roll except the Panic Roll [p. 58–64].
    Roll { request: RollRequest },
    /// An opposed roll; only the attacker may push [p. 64; F-22].
    Opposed {
        attacker: RollRequest,
        defender: RollRequest,
    },
    /// Answers a pending decision.
    Decide { pending: PendingId, choice: Choice },
    /// A Panic Roll called for by the game master, e.g. for a critical
    /// injury or a horrible sight [p. 104].
    PanicRoll { actor: ActorId },
    /// Stress ±N for chosen characters, e.g. for a horrible sight [p. 211].
    ChangeStress { actors: Vec<ActorId>, delta: i8 },
    /// Damage that is suffered, after armor (F-33).
    Damage { actor: ActorId, amount: u8 },
    /// Puts a trauma, condition or critical injury on an actor.
    Apply { actor: ActorId, source: NewSource },
    /// Something happened that ends sources, e.g. MEDICAL AID or warmth.
    End { actor: ActorId, event: EndEvent },
    /// Removes a source by hand (correction by the game master).
    Remove { actor: ActorId, origin: Origin },
    /// Ends a panic by hand, e.g. Berserk when the target is Broken (F-29).
    StopPanic { actor: ActorId },
    /// A step of game time (plan section 9.1).
    Step { step: Step },
    /// These characters sleep (for Nightmares, p. 101).
    Sleep { actors: Vec<ActorId> },
    /// An actor performs an action; checks blocks and fires triggers.
    PerformAction { actor: ActorId, action: Id },
    /// Stress −1 once per Act or session [p. 104].
    UseSignatureItem { actor: ActorId },
    /// End of an Act in Cinematic play.
    EndAct,
    /// End of a game session: resets uses, trauma checks [p. 104].
    EndSession,
}

/// A source to put on an actor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NewSource {
    Trauma(Id),
    Condition { id: Id, cold: Option<Cold> },
    Injury(Id),
}

/// A step of game time. A Shift step can start a new day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Step {
    Round,
    Turn,
    Shift { new_day: bool },
}

/// An answer to a pending decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Choice {
    /// Go on without using any option.
    Accept,
    Push,
    /// Use The Long Haul [p. 76].
    IgnoreFacehuggers,
    StoryPoint,
    Overkill,
    /// Use this optional talent for the roll, e.g. Stoic.
    UseTalent(Id),
    Answer(bool),
    Targets(Vec<ActorId>),
}

/// What happened. The server stores these in the log and shows them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum Event {
    RollStarted {
        roll: PendingId,
        actor: Option<ActorId>,
        skill: Option<Id>,
        attribute: Option<Id>,
        base_dice: u8,
        stress_dice: u8,
        /// Stress Dice removed by a negative modification [p. 62].
        stress_dice_removed: u8,
        modifications: Vec<Modification>,
        max_pushes: u8,
    },
    DiceThrown {
        roll: PendingId,
        dice: Vec<Die>,
        facehuggers: u8,
    },
    Pushed {
        roll: PendingId,
        push: u8,
    },
    TalentUsed {
        actor: ActorId,
        talent: Id,
    },
    StoryPointSpent {
        roll: PendingId,
        player: PlayerId,
    },
    /// A roll an operation asked for did not happen: a Broken character
    /// cannot roll for skills [p. 98].
    RollSkipped {
        actor: ActorId,
        skill: Id,
    },
    RollFinished {
        roll: PendingId,
        outcome: Outcome,
        successes: u8,
        /// Successes of the defender in an opposed roll.
        against: Option<u8>,
        stunts: u8,
    },
    Decision {
        pending: PendingId,
        actor: Option<ActorId>,
        options: Vec<Choice>,
    },
    Question {
        pending: PendingId,
        actor: ActorId,
        question: Id,
    },
    ChooseTargets {
        pending: PendingId,
        actor: ActorId,
        who: Who,
        candidates: Vec<ActorId>,
    },
    StressChanged {
        actor: ActorId,
        from: u8,
        to: u8,
        source: String,
    },
    Damaged {
        actor: ActorId,
        amount: u8,
        health: u8,
    },
    Broken {
        actor: ActorId,
    },
    PanicRolled {
        actor: ActorId,
        d6: u8,
        stress: u8,
        modifications: Vec<Modification>,
        value: u8,
        /// The value before More Panic raised it [p. 104].
        raised_from: Option<u8>,
    },
    PanicEffect {
        actor: ActorId,
        effect: Id,
        value: u8,
    },
    PanicStopped {
        actor: ActorId,
        reason: StopReason,
    },
    SourceApplied {
        actor: ActorId,
        origin: Origin,
        ends: Ending,
    },
    SourceEnded {
        actor: ActorId,
        origin: Origin,
    },
    NextSlowActionLost {
        actor: ActorId,
        /// The cancelled action already was that slow action (F-28).
        already_lost: bool,
    },
    ItemDropped {
        actor: ActorId,
    },
    Hint {
        actor: ActorId,
        text: Id,
        gm_only: bool,
    },
    SupplyRolled {
        actor: ActorId,
        supply: Supply,
        dice: Vec<Die>,
        from: u8,
        to: u8,
    },
    TraumaCheck {
        actor: ActorId,
        dice: Vec<Die>,
        trauma: Option<Id>,
    },
    SignatureItemUsed {
        actor: ActorId,
    },
    ActionPerformed {
        actor: ActorId,
        action: Id,
    },
}

/// A roll as it would be built, see [`Engine::preview`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Preview {
    pub skill: Option<Id>,
    pub attribute: Option<Id>,
    pub base_dice: u8,
    pub stress_dice: u8,
    pub stress_dice_removed: u8,
    pub modifications: Vec<Modification>,
    pub max_pushes: u8,
    /// Optional talents that fit this roll.
    pub talents: Vec<Id>,
}

/// Dice for previews: building a pool never throws.
struct NoDice;

impl Dice for NoDice {
    fn d6(&mut self) -> u8 {
        unreachable!("a preview throws no dice")
    }
}

/// Why a command was refused. The state is unchanged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleError {
    UnknownActor(ActorId),
    UnknownEntry(Id),
    UnknownPending(PendingId),
    InvalidChoice(String),
    NotAllowed(String),
}

impl fmt::Display for RuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleError::UnknownActor(id) => write!(f, "unknown actor \"{id}\""),
            RuleError::UnknownEntry(id) => write!(f, "unknown rule entry \"{id}\""),
            RuleError::UnknownPending(id) => write!(f, "no pending decision {id}"),
            RuleError::InvalidChoice(why) => write!(f, "invalid choice: {why}"),
            RuleError::NotAllowed(why) => write!(f, "not allowed: {why}"),
        }
    }
}

impl std::error::Error for RuleError {}

pub type Result<T> = std::result::Result<T, RuleError>;

impl Engine {
    pub fn new(rules: RuleData) -> Self {
        Self { rules }
    }

    pub fn rules(&self) -> &RuleData {
        &self.rules
    }

    /// Runs a command. On error the state stays as it was.
    pub fn apply(
        &self,
        state: &mut GameState,
        command: Command,
        dice: &mut dyn Dice,
    ) -> Result<Vec<Event>> {
        let mut ctx = Ctx {
            engine: self,
            state: state.clone(),
            dice,
            events: Vec::new(),
            queue: VecDeque::new(),
        };
        ctx.command(command)?;
        ctx.run_queue()?;
        *state = ctx.state;
        Ok(ctx.events)
    }

    /// What a roll would look like, without throwing dice: for the roll
    /// dialog, so the interface never computes rules itself.
    pub fn preview(&self, state: &GameState, request: RollRequest) -> Result<Preview> {
        let mut dice = NoDice;
        let mut ctx = Ctx {
            engine: self,
            state: state.clone(),
            dice: &mut dice,
            events: Vec::new(),
            queue: VecDeque::new(),
        };
        let talents = match (&request.actor, &request.kind) {
            (Some(actor), crate::roll::RollKind::Skill { skill }) => {
                ctx.fitting_talents(actor, skill)?
            }
            (Some(actor), crate::roll::RollKind::Death) => ctx.fitting_talents(actor, "stamina")?,
            (
                _,
                crate::roll::RollKind::Supply { .. } | crate::roll::RollKind::StressDice { .. },
            ) => {
                return Err(RuleError::NotAllowed(
                    "no preview for this roll kind".into(),
                ));
            }
            _ => Vec::new(),
        };
        ctx.build(request, crate::roll::Role::Single, None)?;
        match ctx.events.into_iter().next() {
            Some(Event::RollStarted {
                skill,
                attribute,
                base_dice,
                stress_dice,
                stress_dice_removed,
                modifications,
                max_pushes,
                ..
            }) => Ok(Preview {
                skill,
                attribute,
                base_dice,
                stress_dice,
                stress_dice_removed,
                modifications,
                max_pushes,
                talents,
            }),
            _ => Err(RuleError::NotAllowed("no preview for this roll".into())),
        }
    }

    /// Hints working on an actor right now, for the overview: label, text
    /// key and visibility.
    pub fn hints(&self, state: &GameState, actor: &str) -> Vec<(String, Id, bool)> {
        let Some(actor) = state.actor(actor) else {
            return Vec::new();
        };
        self.active_effects(state, actor)
            .into_iter()
            .filter_map(|a| match a.effect.as_ref() {
                Effect::Hint { text, gm_only, .. } => {
                    Some((a.label.clone(), text.clone(), *gm_only))
                }
                _ => None,
            })
            .collect()
    }

    // ---- lookups ---------------------------------------------------------

    pub(crate) fn talent(&self, id: &str) -> Option<&Talent> {
        self.rules.talents.iter().find(|e| e.id == id)
    }

    pub(crate) fn trauma(&self, id: &str) -> Option<&Trauma> {
        self.rules.traumas.iter().find(|e| e.id == id)
    }

    pub(crate) fn condition(&self, id: &str) -> Option<&Condition> {
        self.rules.conditions.iter().find(|e| e.id == id)
    }

    pub(crate) fn injury(&self, id: &str) -> Option<&CriticalInjury> {
        self.rules.critical_injuries.iter().find(|e| e.id == id)
    }

    pub(crate) fn panic_entry(&self, id: &str) -> Option<&PanicResult> {
        self.rules.panic.iter().find(|e| e.id == id)
    }

    pub(crate) fn skill(&self, id: &str) -> Option<&Skill> {
        self.rules.skills.iter().find(|e| e.id == id)
    }

    /// Row of the Panic table for a value; values below the table count
    /// as its first row (docs/datenformat.md).
    pub(crate) fn panic_row(&self, value: u8) -> Option<&PanicResult> {
        let rows = self
            .rules
            .panic
            .iter()
            .filter_map(|r| r.roll.map(|roll| (r, roll)));
        let mut first: Option<(&PanicResult, RollRange)> = None;
        for (row, roll) in rows {
            if roll.from <= value && roll.to.is_none_or(|to| value <= to) {
                return Some(row);
            }
            if first.is_none_or(|(_, f)| roll.from < f.from) {
                first = Some((row, roll));
            }
        }
        first
            .filter(|(_, roll)| value < roll.from)
            .map(|(row, _)| row)
    }

    /// Option chosen for a ruling; its default if the team chose none.
    pub(crate) fn ruling_option<'s>(&'s self, state: &'s GameState, id: &str) -> &'s str {
        if let Some(option) = state.rulings.get(id) {
            return option;
        }
        self.rules
            .rulings
            .iter()
            .find(|r| r.id == id)
            .map(|r| r.default.as_str())
            .unwrap_or("")
    }

    /// The effects of an entry under the team's ruling option.
    pub(crate) fn with_ruling<'s>(
        &'s self,
        state: &GameState,
        effects: &'s [Effect],
        ruling: Option<&'s RulingSwitch>,
    ) -> Vec<&'s Effect> {
        if let Some(switch) = ruling {
            let option = self.ruling_option(state, &switch.id);
            if let Some(alternative) = switch.alternatives.get(option) {
                return alternative.iter().collect();
            }
        }
        effects.iter().collect()
    }

    /// The period a talent's limited use refers to.
    fn use_period(&self, talent: &str) -> Option<UsePeriod> {
        self.talent(talent)?.effects.iter().find_map(|e| match e {
            Effect::Replacement { uses: Some(u), .. } => Some(u.per),
            _ => None,
        })
    }

    /// Is the action granted by some capability, so that it needs one?
    fn needs_capability(&self, action: &str) -> bool {
        let mut all = Vec::new();
        self.rules
            .for_each_entry(|_, entry| all.extend(entry.effects()));
        all.iter()
            .any(|e| matches!(e, Effect::Capability { grants, .. } if grants == action))
    }
}

impl Engine {
    /// A value from `constants.json`. The data validation makes sure the
    /// engine's constants exist.
    pub fn constant(&self, id: &str) -> i16 {
        self.rules
            .constants
            .iter()
            .find(|c| c.id == id)
            .map(|c| c.value)
            .unwrap_or_else(|| panic!("constant \"{id}\" is missing in constants.json"))
    }

    pub(crate) fn constant_u8(&self, id: &str) -> u8 {
        u8::try_from(self.constant(id)).unwrap_or(0)
    }

    /// Duration of a source or effect from its data. Days count as Shift
    /// steps (plan section 9.1).
    pub(crate) fn ending(&self, lasts: Lasts) -> Option<Ending> {
        match lasts {
            Lasts::Instant => None,
            Lasts::Steps {
                unit: Unit::Day,
                count,
            } => Some(Ending::After {
                unit: Unit::Shift,
                left: u16::from(count) * u16::from(self.constant_u8("shifts_per_day")),
            }),
            Lasts::Steps { unit, count } => Some(Ending::After {
                unit,
                left: count.into(),
            }),
            Lasts::Until(event) => Some(Ending::On(event)),
            Lasts::Permanent | Lasts::WhileSource => Some(Ending::Never),
        }
    }
}

/// Work waiting in the queue.
#[derive(Debug, Clone)]
pub(crate) enum Task {
    Ops {
        actor: ActorId,
        owner: Id,
        ops: Vec<Op>,
        flags: OpFlags,
    },
    Panic {
        actor: ActorId,
        cause: PanicCause,
    },
}

/// Circumstances of a list of operations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpFlags {
    /// From a condition: its damage gives no extra stress (F-32).
    pub from_condition: bool,
    /// A cancelled slow action counts as the lost one (F-28).
    pub cancelled_slow: bool,
}

/// Everything one command works with.
pub(crate) struct Ctx<'a> {
    pub engine: &'a Engine,
    pub state: GameState,
    pub dice: &'a mut dyn Dice,
    pub events: Vec<Event>,
    pub queue: VecDeque<Task>,
}

impl<'a> Ctx<'a> {
    pub fn actor(&self, id: &str) -> Result<&Actor> {
        self.state
            .actors
            .get(id)
            .ok_or_else(|| RuleError::UnknownActor(id.to_owned()))
    }

    pub fn actor_mut(&mut self, id: &str) -> Result<&mut Actor> {
        self.state
            .actors
            .get_mut(id)
            .ok_or_else(|| RuleError::UnknownActor(id.to_owned()))
    }

    /// Effects working on an actor right now.
    pub fn effects(&self, id: &str) -> Result<Vec<ActiveEffect<'a>>> {
        let engine = self.engine;
        let actor = self.actor(id)?;
        Ok(engine.active_effects(&self.state, actor))
    }

    pub fn emit(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn next_pending(&mut self) -> PendingId {
        self.state.next_pending += 1;
        self.state.next_pending
    }

    pub fn throw(&mut self, base: u8, stress: u8) -> Vec<Die> {
        throw(self.dice, base, stress)
    }

    fn command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Roll { request } => self.start_roll(request),
            Command::Opposed { attacker, defender } => self.start_opposed(attacker, defender),
            Command::Decide { pending, choice } => self.decide(pending, choice),
            Command::PanicRoll { actor } => {
                if !self.actor(&actor)?.human_pc() {
                    return Err(RuleError::NotAllowed(
                        "only human player characters make Panic Rolls (F-43)".into(),
                    ));
                }
                self.queue.push_back(Task::Panic {
                    actor,
                    cause: PanicCause::Other,
                });
                Ok(())
            }
            Command::ChangeStress { actors, delta } => {
                for actor in actors {
                    if !self.actor(&actor)?.human_pc() {
                        return Err(RuleError::NotAllowed(format!(
                            "\"{actor}\" has no stress level"
                        )));
                    }
                    self.change_stress(&actor, delta, "gm")?;
                }
                Ok(())
            }
            Command::Damage { actor, amount } => self.damage(&actor, amount, false),
            Command::Apply { actor, source } => self.apply_source(&actor, source),
            Command::End { actor, event } => self.end_event(&actor, event),
            Command::Remove { actor, origin } => self.remove_source(&actor, &origin),
            Command::StopPanic { actor } => self.stop_panic(&actor, StopReason::GameMaster),
            Command::Step { step } => self.step(step),
            Command::Sleep { actors } => {
                for actor in actors {
                    self.fire_triggers(&actor, |on| matches!(on, TriggerEvent::Sleep))?;
                }
                Ok(())
            }
            Command::PerformAction { actor, action } => self.perform_action(&actor, action),
            Command::UseSignatureItem { actor } => self.use_signature_item(&actor),
            Command::EndAct => {
                if self.state.mode != Mode::Cinematic {
                    return Err(RuleError::NotAllowed(
                        "Acts exist only in Cinematic play".into(),
                    ));
                }
                self.reset_uses(false);
                Ok(())
            }
            Command::EndSession => self.end_session(),
        }
    }

    fn decide(&mut self, id: PendingId, choice: Choice) -> Result<()> {
        let pending = self
            .state
            .pending
            .remove(&id)
            .ok_or(RuleError::UnknownPending(id))?;
        match pending {
            Pending::Roll(roll) => self.decide_roll(*roll, choice),
            Pending::StartRoll {
                mut request,
                then,
                options,
            } => {
                if !options.contains(&choice) {
                    return Err(RuleError::InvalidChoice(format!("{choice:?}")));
                }
                if let Choice::UseTalent(talent) = choice {
                    request.talents.push(talent);
                }
                self.start_roll_with(*request, crate::roll::Role::Single, then)
            }
            Pending::Overkill {
                actor,
                value,
                talent,
                effect,
                cancelled_slow,
            } => {
                let chosen = match choice {
                    Choice::Overkill => {
                        let overkill = self
                            .engine
                            .talent(&talent)
                            .and_then(|t| {
                                t.effects.iter().find_map(|e| match e {
                                    Effect::Replacement {
                                        with: ReplaceWith::PanicEffect(id),
                                        ..
                                    } => Some(id.clone()),
                                    _ => None,
                                })
                            })
                            .ok_or_else(|| RuleError::UnknownEntry(talent.clone()))?;
                        self.emit(Event::TalentUsed {
                            actor: actor.clone(),
                            talent,
                        });
                        overkill
                    }
                    Choice::Accept => effect,
                    other => return Err(RuleError::InvalidChoice(format!("{other:?}"))),
                };
                self.start_panic_effect(&actor, &chosen, value, cancelled_slow)
            }
            Pending::Question {
                actor,
                owner,
                on_no,
                ..
            } => match choice {
                Choice::Answer(true) => Ok(()),
                Choice::Answer(false) => {
                    self.queue.push_back(Task::Ops {
                        actor,
                        owner,
                        ops: on_no,
                        flags: OpFlags::default(),
                    });
                    Ok(())
                }
                other => Err(RuleError::InvalidChoice(format!("{other:?}"))),
            },
            Pending::Targets {
                owner,
                candidates,
                ops,
                ..
            } => match choice {
                Choice::Targets(chosen) => {
                    if let Some(bad) = chosen.iter().find(|c| !candidates.contains(c)) {
                        return Err(RuleError::InvalidChoice(format!(
                            "\"{bad}\" is not a candidate"
                        )));
                    }
                    for target in chosen {
                        self.queue.push_back(Task::Ops {
                            actor: target,
                            owner: owner.clone(),
                            ops: ops.clone(),
                            flags: OpFlags::default(),
                        });
                    }
                    Ok(())
                }
                other => Err(RuleError::InvalidChoice(format!("{other:?}"))),
            },
        }
    }

    /// Works off the queue, with state checks after every task.
    fn run_queue(&mut self) -> Result<()> {
        self.state_checks()?;
        while let Some(task) = self.queue.pop_front() {
            match task {
                Task::Ops {
                    actor,
                    owner,
                    ops,
                    flags,
                } => self.run_ops(&actor, &owner, &ops, flags)?,
                Task::Panic { actor, cause } => {
                    if self.actor(&actor)?.human_pc() {
                        self.panic_roll(&actor, cause)?;
                    }
                }
            }
            self.state_checks()?;
        }
        Ok(())
    }

    /// Checks after every step: Broken ends panic [p. 104]. A panic that
    /// began while Broken ends at once or runs on (F-48).
    fn state_checks(&mut self) -> Result<()> {
        let runs_on = self.engine.ruling_option(&self.state, "F-48") == "runs_on";
        let broken: Vec<ActorId> = self
            .state
            .actors
            .iter()
            .filter(|(_, a)| {
                a.broken()
                    && a.panic
                        .as_ref()
                        .is_some_and(|p| !(runs_on && p.started_broken))
            })
            .map(|(id, _)| id.clone())
            .collect();
        for actor in broken {
            self.stop_panic(&actor, StopReason::Broken)?;
        }
        Ok(())
    }

    // ---- operations --------------------------------------------------------

    pub fn run_ops(&mut self, actor: &str, owner: &str, ops: &[Op], flags: OpFlags) -> Result<()> {
        // Operations for a group are collected per group, so the game
        // master confirms each group once.
        let mut groups: Vec<(Who, Vec<Op>)> = Vec::new();
        for op in ops {
            let who = match op {
                Op::Stress { who, .. } | Op::Damage { who, .. } | Op::PanicRoll { who } => *who,
                _ => Who::Bearer,
            };
            if who != Who::Bearer {
                let for_each = match op.clone() {
                    Op::Stress { delta, .. } => Op::Stress {
                        delta,
                        who: Who::Bearer,
                    },
                    Op::Damage { amount, .. } => Op::Damage {
                        amount,
                        who: Who::Bearer,
                    },
                    _ => Op::PanicRoll { who: Who::Bearer },
                };
                match groups.iter_mut().find(|(w, _)| *w == who) {
                    Some((_, list)) => list.push(for_each),
                    None => groups.push((who, vec![for_each])),
                }
                continue;
            }
            self.run_op(actor, owner, op, flags)?;
        }
        for (who, list) in groups {
            self.ask_targets(actor, owner, who, list)?;
        }
        Ok(())
    }

    fn run_op(&mut self, actor: &str, owner: &str, op: &Op, flags: OpFlags) -> Result<()> {
        match op {
            Op::Stress { delta, .. } => self.change_stress(actor, *delta, owner),
            Op::Damage { amount, .. } => self.damage(actor, *amount, flags.from_condition),
            Op::Roll {
                roll,
                on_success,
                on_fail,
            } => {
                let continuation = crate::roll::Continuation {
                    actor: actor.to_owned(),
                    owner: owner.to_owned(),
                    on_success: on_success.clone(),
                    on_fail: on_fail.clone(),
                    from_condition: flags.from_condition,
                };
                let kind = match roll {
                    RollSpec::Skill(skill) => crate::roll::RollKind::Skill {
                        skill: skill.clone(),
                    },
                    RollSpec::Attribute(attribute) => crate::roll::RollKind::Attribute {
                        attribute: attribute.clone(),
                    },
                };
                let request = RollRequest::new(actor, kind);
                self.roll_for_op(request, continuation)
            }
            Op::PanicRoll { .. } => {
                self.queue.push_back(Task::Panic {
                    actor: actor.to_owned(),
                    cause: PanicCause::Other,
                });
                Ok(())
            }
            Op::LoseNextAction { .. } => {
                if !flags.cancelled_slow {
                    self.actor_mut(actor)?.loses_next_slow_action = true;
                }
                self.emit(Event::NextSlowActionLost {
                    actor: actor.to_owned(),
                    already_lost: flags.cancelled_slow,
                });
                Ok(())
            }
            Op::DropItem { .. } => {
                self.emit(Event::ItemDropped {
                    actor: actor.to_owned(),
                });
                Ok(())
            }
            Op::Block { blocks, lasts } => {
                let Some(ends) = self.engine.ending(*lasts) else {
                    return Ok(());
                };
                self.add_source(
                    actor,
                    Origin::Block {
                        blocks: blocks.clone(),
                        by: owner.to_owned(),
                    },
                    ends,
                )
            }
            Op::Ask { question, on_no } => {
                let pending = self.next_pending();
                self.state.pending.insert(
                    pending,
                    Pending::Question {
                        actor: actor.to_owned(),
                        owner: owner.to_owned(),
                        question: question.clone(),
                        on_no: on_no.clone(),
                    },
                );
                self.emit(Event::Question {
                    pending,
                    actor: actor.to_owned(),
                    question: question.clone(),
                });
                Ok(())
            }
            Op::Hint { text } => {
                self.emit(Event::Hint {
                    actor: actor.to_owned(),
                    text: text.clone(),
                    gm_only: false,
                });
                Ok(())
            }
            Op::Special { handler } => Err(RuleError::NotAllowed(format!(
                "no special handler \"{handler}\""
            ))),
        }
    }

    /// Proposes the friendly human player characters (F-39); the game
    /// master picks who is affected.
    fn ask_targets(&mut self, actor: &str, owner: &str, who: Who, ops: Vec<Op>) -> Result<()> {
        let candidates: Vec<ActorId> = self
            .state
            .actors
            .iter()
            .filter(|(id, a)| id.as_str() != actor && a.human_pc())
            .map(|(id, _)| id.clone())
            .collect();
        if candidates.is_empty() {
            return Ok(());
        }
        let pending = self.next_pending();
        self.state.pending.insert(
            pending,
            Pending::Targets {
                actor: actor.to_owned(),
                owner: owner.to_owned(),
                who,
                candidates: candidates.clone(),
                ops,
            },
        );
        self.emit(Event::ChooseTargets {
            pending,
            actor: actor.to_owned(),
            who,
            candidates,
        });
        Ok(())
    }

    /// Changes the STRESS LEVEL of a human PC; never below 0, no maximum
    /// (F-19). Others have no stress level and are left alone.
    pub fn change_stress(&mut self, actor: &str, delta: i8, source: &str) -> Result<()> {
        let a = self.actor_mut(actor)?;
        if !a.human_pc() || delta == 0 {
            return Ok(());
        }
        let from = a.stress;
        let to = u8::try_from((i16::from(from) + i16::from(delta)).max(0)).unwrap_or(u8::MAX);
        a.stress = to;
        if from != to {
            self.emit(Event::StressChanged {
                actor: actor.to_owned(),
                from,
                to,
                source: source.to_owned(),
            });
        }
        Ok(())
    }

    /// Suffered damage. Gives stress +1 [p. 103], except damage from a
    /// condition, whose stress comes with it (F-32, Hayes p. 108).
    pub fn damage(&mut self, actor: &str, amount: u8, from_condition: bool) -> Result<()> {
        if amount == 0 {
            return Ok(());
        }
        let a = self.actor_mut(actor)?;
        let before = a.health;
        a.health = a.health.saturating_sub(amount);
        let health = a.health;
        self.emit(Event::Damaged {
            actor: actor.to_owned(),
            amount,
            health,
        });
        if before > 0 && health == 0 {
            self.emit(Event::Broken {
                actor: actor.to_owned(),
            });
        }
        if !from_condition {
            self.change_stress(actor, 1, "damage")?;
        }
        Ok(())
    }

    // ---- sources -----------------------------------------------------------

    fn apply_source(&mut self, actor: &str, source: NewSource) -> Result<()> {
        let engine = self.engine;
        self.actor(actor)?;
        let (origin, ends, on_apply) = match source {
            NewSource::Trauma(id) => {
                engine
                    .trauma(&id)
                    .ok_or_else(|| RuleError::UnknownEntry(id.clone()))?;
                (Origin::Trauma(id), Ending::Never, Vec::new())
            }
            NewSource::Condition { id, cold } => {
                let condition = engine
                    .condition(&id)
                    .ok_or_else(|| RuleError::UnknownEntry(id.clone()))?;
                let needs_cold = condition.effects.iter().any(|e| {
                    matches!(
                        e,
                        Effect::Periodic {
                            every: crate::data::effects::Interval::ByCold(_),
                            ..
                        }
                    )
                });
                if needs_cold && cold.is_none() {
                    return Err(RuleError::NotAllowed(format!(
                        "condition \"{id}\" needs a cold level [p. 107]"
                    )));
                }
                let ends = Ending::On(condition.ends);
                (Origin::Condition { id, cold }, ends, Vec::new())
            }
            NewSource::Injury(id) => {
                let injury = engine
                    .injury(&id)
                    .ok_or_else(|| RuleError::UnknownEntry(id.clone()))?;
                let ends = match injury.healing {
                    Healing::None => None,
                    Healing::DaysD6(n) => {
                        let days: u16 = (0..n).map(|_| u16::from(self.dice.d6())).sum();
                        Some(Ending::After {
                            unit: Unit::Shift,
                            left: days * u16::from(engine.constant_u8("shifts_per_day")),
                        })
                    }
                    Healing::UntilMedicalAid => Some(Ending::On(EndEvent::MedicalAid)),
                    Healing::Permanent => Some(Ending::Never),
                };
                let on_apply = injury.on_apply.clone();
                match ends {
                    Some(ends) => (Origin::Injury(id), ends, on_apply),
                    None => {
                        self.queue.push_back(Task::Ops {
                            actor: actor.to_owned(),
                            owner: id,
                            ops: on_apply,
                            flags: OpFlags::default(),
                        });
                        return Ok(());
                    }
                }
            }
        };
        let owner = origin.entry().to_owned();
        self.add_source(actor, origin, ends)?;
        if !on_apply.is_empty() {
            self.queue.push_back(Task::Ops {
                actor: actor.to_owned(),
                owner,
                ops: on_apply,
                flags: OpFlags::default(),
            });
        }
        Ok(())
    }

    /// Adds a source; effects with their own duration become parts.
    pub fn add_source(&mut self, actor: &str, origin: Origin, ends: Ending) -> Result<()> {
        let engine = self.engine;
        let parts: Vec<(usize, Ending)> = engine
            .origin_effects(&self.state, &origin)
            .iter()
            .enumerate()
            .filter_map(|(i, e)| {
                e.common()
                    .1
                    .and_then(|l| engine.ending(l))
                    .map(|end| (i, end))
            })
            .collect();
        let a = self.actor_mut(actor)?;
        a.sources.push(ActiveSource {
            origin: origin.clone(),
            ends,
        });
        for (index, end) in &parts {
            a.sources.push(ActiveSource {
                origin: Origin::Part {
                    of: Box::new(origin.clone()),
                    index: *index,
                },
                ends: *end,
            });
        }
        self.emit(Event::SourceApplied {
            actor: actor.to_owned(),
            origin,
            ends,
        });
        Ok(())
    }

    /// Removes sources (and their parts) for which `ended` is true.
    fn remove_sources(&mut self, actor: &str, ended: impl Fn(&ActiveSource) -> bool) -> Result<()> {
        let a = self.actor_mut(actor)?;
        let gone: Vec<Origin> = a
            .sources
            .iter()
            .filter(|s| ended(s))
            .map(|s| s.origin.clone())
            .collect();
        a.sources.retain(|s| {
            !ended(s) && !matches!(&s.origin, Origin::Part { of, .. } if gone.contains(of.as_ref()))
        });
        for origin in gone {
            if !matches!(origin, Origin::Part { .. }) {
                self.emit(Event::SourceEnded {
                    actor: actor.to_owned(),
                    origin,
                });
            }
        }
        Ok(())
    }

    pub fn end_event(&mut self, actor: &str, event: EndEvent) -> Result<()> {
        self.remove_sources(actor, |s| s.ends == Ending::On(event))
    }

    fn remove_source(&mut self, actor: &str, origin: &Origin) -> Result<()> {
        if !self
            .actor(actor)?
            .sources
            .iter()
            .any(|s| &s.origin == origin)
        {
            return Err(RuleError::NotAllowed(format!(
                "\"{actor}\" has no {}",
                origin.label()
            )));
        }
        self.remove_sources(actor, |s| &s.origin == origin)
    }

    // ---- time ------------------------------------------------------------

    fn step(&mut self, step: Step) -> Result<()> {
        let (unit, new_day) = match step {
            Step::Round => (Unit::Round, false),
            Step::Turn => (Unit::Turn, false),
            Step::Shift { new_day } => (Unit::Shift, new_day),
        };
        let ids: Vec<ActorId> = self.state.actors.keys().cloned().collect();
        for id in &ids {
            let a = self.actor_mut(id)?;
            for source in &mut a.sources {
                tick(&mut source.ends, unit);
            }
            if let Some(panic) = &mut a.panic {
                if let Some(end) = &mut panic.effects_end {
                    tick(end, unit);
                    if matches!(end, Ending::After { left: 0, .. }) {
                        panic.effects_end = None;
                    }
                }
            }
            self.remove_sources(id, |s| matches!(s.ends, Ending::After { left: 0, .. }))?;
            if new_day {
                self.end_event(id, EndEvent::DayChange)?;
            }
            // After a Turn, and after a Shift, at least one Turn has passed
            // (plan section 9.1): panic stops [p. 104].
            if matches!(unit, Unit::Turn | Unit::Shift) && self.actor(id)?.panic.is_some() {
                self.stop_panic(id, StopReason::TurnPassed)?;
            }
        }
        for id in &ids {
            for active in self.effects(id)? {
                if let Effect::Periodic { every, then, .. } = active.effect.as_ref() {
                    if active::fires(every, active.cold, unit, new_day) {
                        self.queue.push_back(Task::Ops {
                            actor: id.clone(),
                            owner: active.owner.clone(),
                            ops: then.clone(),
                            flags: OpFlags {
                                from_condition: active.from_condition,
                                cancelled_slow: false,
                            },
                        });
                    }
                }
            }
        }
        Ok(())
    }

    /// Queues the operations of matching trigger effects.
    pub fn fire_triggers(
        &mut self,
        actor: &str,
        matches: impl Fn(&TriggerEvent) -> bool,
    ) -> Result<()> {
        for active in self.effects(actor)? {
            if let Effect::Trigger { on, then, .. } = active.effect.as_ref() {
                if matches(on) {
                    self.queue.push_back(Task::Ops {
                        actor: actor.to_owned(),
                        owner: active.owner.clone(),
                        ops: then.clone(),
                        flags: OpFlags {
                            from_condition: active.from_condition,
                            cancelled_slow: false,
                        },
                    });
                }
            }
        }
        Ok(())
    }

    // ---- actions, signature item, sessions -----------------------------------

    fn perform_action(&mut self, actor: &str, action: Id) -> Result<()> {
        if !self.engine.rules.actions.iter().any(|a| a.id == action) {
            return Err(RuleError::UnknownEntry(action));
        }
        let effects = self.effects(actor)?;
        if let Some(by) = active::blocking(&effects, &Blocked::Action(action.clone())) {
            return Err(RuleError::NotAllowed(format!(
                "\"{action}\" is blocked by {by}"
            )));
        }
        let granted = effects.iter().any(
            |a| matches!(a.effect.as_ref(), Effect::Capability { grants, .. } if *grants == action),
        );
        if self.engine.needs_capability(&action) && !granted {
            return Err(RuleError::NotAllowed(format!(
                "nothing grants \"{action}\""
            )));
        }
        self.emit(Event::ActionPerformed {
            actor: actor.to_owned(),
            action: action.clone(),
        });
        self.fire_triggers(
            actor,
            |on| matches!(on, TriggerEvent::Action(a) if *a == action),
        )
    }

    /// Stress −1 once per Act (Cinematic) or session (Campaign); blocked
    /// like other stress relief (F-31) [p. 104].
    fn use_signature_item(&mut self, actor: &str) -> Result<()> {
        let a = self.actor(actor)?;
        if !a.human_pc() {
            return Err(RuleError::NotAllowed(format!(
                "\"{actor}\" has no stress level"
            )));
        }
        if a.signature_item_used {
            return Err(RuleError::NotAllowed(
                "the signature item was already used this Act or session".into(),
            ));
        }
        let effects = self.effects(actor)?;
        if let Some(by) = active::blocking(&effects, &Blocked::StressRelief) {
            return Err(RuleError::NotAllowed(format!(
                "stress relief is blocked by {by}"
            )));
        }
        self.actor_mut(actor)?.signature_item_used = true;
        self.emit(Event::SignatureItemUsed {
            actor: actor.to_owned(),
        });
        self.change_stress(actor, -1, "signature_item")
    }

    /// Resets limited uses. At the end of an Act only the "per Act"
    /// things reset; at the end of a session everything does.
    fn reset_uses(&mut self, session: bool) {
        let engine = self.engine;
        for actor in self.state.actors.values_mut() {
            actor.signature_item_used = false;
            actor.uses.retain(|talent, _| {
                let period = engine.use_period(talent);
                !(session || period == Some(UsePeriod::ActOrSession))
            });
        }
    }

    /// End of a game session: trauma checks [p. 104; F-01, F-26, F-27,
    /// F-42], then all uses reset.
    fn end_session(&mut self) -> Result<()> {
        let checks: Vec<ActorId> = self
            .state
            .actors
            .iter()
            .filter(|(_, a)| a.panic_13_plus && a.human_pc())
            .map(|(id, _)| id.clone())
            .collect();
        for actor in checks {
            let empathy = self.actor(&actor)?.attribute("empathy");
            let dice = self.throw(empathy, 0);
            let failed = !dice.iter().any(Die::success);
            let trauma = if failed {
                let d6 = self.dice.d6();
                let trauma = self
                    .engine
                    .rules
                    .traumas
                    .iter()
                    .find(|t| t.d6 == d6)
                    .map(|t| t.id.clone())
                    .ok_or_else(|| RuleError::UnknownEntry(format!("trauma d6 {d6}")))?;
                Some(trauma)
            } else {
                None
            };
            self.emit(Event::TraumaCheck {
                actor: actor.clone(),
                dice,
                trauma: trauma.clone(),
            });
            self.actor_mut(&actor)?.panic_13_plus = false;
            if let Some(trauma) = trauma {
                self.apply_source(&actor, NewSource::Trauma(trauma))?;
            }
        }
        self.reset_uses(true);
        Ok(())
    }

    /// Uses left of a talent with a limited use.
    pub fn uses_left(&self, actor: &str, talent: &str) -> Result<u8> {
        let max = self
            .engine
            .talent(talent)
            .and_then(|t| {
                t.effects.iter().find_map(|e| match e {
                    Effect::Replacement { uses, .. } => Some(uses.map_or(u8::MAX, |u| u.count)),
                    _ => None,
                })
            })
            .unwrap_or(u8::MAX);
        let used = self.actor(actor)?.uses.get(talent).copied().unwrap_or(0);
        Ok(max.saturating_sub(used))
    }

    pub fn spend_use(&mut self, actor: &str, talent: &str) -> Result<()> {
        *self
            .actor_mut(actor)?
            .uses
            .entry(talent.to_owned())
            .or_default() += 1;
        self.emit(Event::TalentUsed {
            actor: actor.to_owned(),
            talent: talent.to_owned(),
        });
        Ok(())
    }

    /// Talent that offers a replacement of this kind, if the actor has
    /// one with uses left.
    pub fn usable_talent(
        &self,
        actor: &str,
        fits: impl Fn(&Replaceable, &ReplaceWith) -> bool,
    ) -> Result<Option<Id>> {
        let effects = self.effects(actor)?;
        for usable in active::replacements(&effects) {
            if fits(usable.replaces, usable.with) && self.uses_left(actor, &usable.owner)? > 0 {
                return Ok(Some(usable.owner));
            }
        }
        Ok(None)
    }

    /// Modifications from effects for a roll.
    pub fn effect_modifiers(&self, actor: &str, roll: RollMatch) -> Result<Vec<Modification>> {
        Ok(active::roll_modifiers(&self.effects(actor)?, roll))
    }
}

/// Counts down a step-based ending of the matching unit.
fn tick(end: &mut Ending, unit: Unit) {
    if let Ending::After { unit: u, left } = end {
        if *u == unit && *left > 0 {
            *left -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_count_as_four_shifts() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/rules");
        let engine = Engine::new(RuleData::load(&dir).unwrap());
        let lasts = Lasts::Steps {
            unit: Unit::Day,
            count: 1,
        };
        assert_eq!(
            engine.ending(lasts),
            Some(Ending::After {
                unit: Unit::Shift,
                left: 4
            })
        );
    }

    #[test]
    fn dice_kinds_are_serialized_in_snake_case() {
        let die = Die {
            kind: crate::dice::DieKind::Stress,
            face: 1,
            locked: false,
        };
        let json = serde_json::to_string(&die).unwrap();
        assert!(json.contains("\"stress\""), "{json}");
    }
}

//! All rolls except the Panic Roll [p. 58–64, 99, 34; plan section 4.1].
//!
//! One dice pool serves every roll kind; the kinds differ in their rules:
//! Stress Dice or not, push or not, what a facehugger does. The skill roll
//! stops for decisions: The Long Haul, push, Story Point.

use serde::{Deserialize, Serialize};

use crate::active::{self, Modification, RollMatch};
use crate::data::Id;
use crate::data::effects::{Op, ReplaceWith, Replaceable, RollKind as DataRollKind, TriggerEvent};
use crate::dice::{Die, DieKind};
use crate::engine::{Choice, Ctx, Event, OpFlags, Result, RuleError, Task};
use crate::panic::{PanicCause, StopReason};
use crate::state::{ActorId, Mode, Pending, PendingId, Supply};

/// What to roll.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollKind {
    /// Attribute + skill + modifications, with Stress Dice [p. 58].
    Skill { skill: Id },
    /// Attribute only: no Stress Dice, no push (F-27).
    Attribute { attribute: Id },
    /// STAMINA roll without Stress Dice and without push [p. 99].
    Death,
    /// A fixed number of Base Dice: NPC pool, armor, machines [p. 64, 96].
    FixedPool { dice: u8 },
    /// Stress Dice equal to the Supply rating, at most six [p. 34].
    Supply { supply: Supply },
    /// Single Stress Dice that only look for facehuggers [p. 110].
    StressDice { count: u8 },
}

/// Is the roll a combat action, and of which speed (F-28)?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Speed {
    Slow,
    Fast,
}

/// What a roll is for, if it has a rule consequence.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Purpose {
    #[default]
    Plain,
    /// A COMMAND roll to stop someone else's panic (F-36) [p. 71, 104].
    StopPanic { target: ActorId },
}

/// A roll as asked for by a player or the game master.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollRequest {
    /// Who rolls; may be missing for a fixed pool such as Mother's.
    pub actor: Option<ActorId>,
    pub kind: RollKind,
    /// Gear, difficulty and other modifications [p. 62–63].
    #[serde(default)]
    pub modifiers: Vec<Modification>,
    /// Declared helpers; at most three count [p. 63].
    #[serde(default)]
    pub helpers: u8,
    #[serde(default)]
    pub action: Option<Speed>,
    #[serde(default)]
    pub purpose: Purpose,
    /// Optional talents chosen before rolling: another attribute or skill,
    /// or Breakthrough.
    #[serde(default)]
    pub talents: Vec<Id>,
}

impl RollRequest {
    pub fn new(actor: &str, kind: RollKind) -> Self {
        Self {
            actor: Some(actor.to_owned()),
            kind,
            modifiers: Vec::new(),
            helpers: 0,
            action: None,
            purpose: Purpose::Plain,
            talents: Vec::new(),
        }
    }

    pub fn skill(actor: &str, skill: &str) -> Self {
        Self::new(
            actor,
            RollKind::Skill {
                skill: skill.to_owned(),
            },
        )
    }

    /// A fixed pool without an actor, e.g. Mother's 8 dice [p. 64].
    pub fn pool(dice: u8) -> Self {
        Self {
            actor: None,
            kind: RollKind::FixedPool { dice },
            modifiers: Vec::new(),
            helpers: 0,
            action: None,
            purpose: Purpose::Plain,
            talents: Vec::new(),
        }
    }

    pub fn modifier(mut self, source: &str, value: i8) -> Self {
        self.modifiers.push(Modification {
            source: source.to_owned(),
            value,
        });
        self
    }

    pub fn helpers(mut self, helpers: u8) -> Self {
        self.helpers = helpers;
        self
    }

    pub fn action(mut self, speed: Speed) -> Self {
        self.action = Some(speed);
        self
    }

    pub fn purpose(mut self, purpose: Purpose) -> Self {
        self.purpose = purpose;
        self
    }

    pub fn talent(mut self, talent: &str) -> Self {
        self.talents.push(talent.to_owned());
        self
    }
}

/// Result of a roll.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Success,
    Failure,
    /// A panic effect of 10+ replaced the action, successes or not [p. 104].
    Cancelled,
}

/// Role in an opposed roll.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Single,
    /// Waits for the defender's successes before it is thrown.
    Attacker {
        against: Option<u8>,
    },
    /// Carries the attacker's roll, which is thrown once the defence is
    /// known.
    Defender {
        attacker: Box<RollState>,
    },
}

/// Operations that wait for the result of a roll started by an operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Continuation {
    pub actor: ActorId,
    pub owner: Id,
    pub on_success: Vec<Op>,
    pub on_fail: Vec<Op>,
    pub from_condition: bool,
}

/// A roll in progress.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollState {
    pub id: PendingId,
    pub request: RollRequest,
    /// Skill and attribute actually rolled, after talents.
    pub skill: Option<Id>,
    pub attribute: Option<Id>,
    pub base_dice: u8,
    pub stress_dice: u8,
    pub dice: Vec<Die>,
    pub pushes: u8,
    pub max_pushes: u8,
    /// Facehuggers in the latest throw.
    pub facehuggers: u8,
    /// The Long Haul was used on the latest throw (F-38).
    pub facehuggers_ignored: bool,
    pub panicked: bool,
    pub cancelled: bool,
    pub story_point: bool,
    /// Passed without rolling (Breakthrough).
    pub automatic: bool,
    pub role: Role,
    pub then: Option<Continuation>,
    /// Choices offered while the roll waits.
    #[serde(default)]
    pub offered: Vec<Choice>,
}

impl RollState {
    pub fn successes(&self) -> u8 {
        let rolled = self.dice.iter().filter(|d| d.success()).count();
        u8::try_from(rolled).unwrap_or(u8::MAX)
            + u8::from(self.story_point)
            + u8::from(self.automatic)
    }

    fn is_skill(&self) -> bool {
        matches!(self.request.kind, RollKind::Skill { .. })
    }
}

impl Ctx<'_> {
    pub(crate) fn start_roll(&mut self, request: RollRequest) -> Result<()> {
        self.start_roll_with(request, Role::Single, None)
    }

    pub(crate) fn start_opposed(
        &mut self,
        attacker: RollRequest,
        defender: RollRequest,
    ) -> Result<()> {
        for side in [&attacker, &defender] {
            if matches!(
                side.kind,
                RollKind::Supply { .. } | RollKind::StressDice { .. }
            ) {
                return Err(RuleError::NotAllowed(
                    "this roll kind cannot be opposed".into(),
                ));
            }
        }
        let attacker = self.build(attacker, Role::Attacker { against: None }, None)?;
        let defender = self.build(
            defender,
            Role::Defender {
                attacker: Box::new(attacker),
            },
            None,
        )?;
        self.throw_and_go(defender)
    }

    /// A roll asked for by an operation, e.g. Freezing's STAMINA roll. A
    /// Broken character cannot roll for skills [p. 98]; optional talents
    /// that fit the roll are offered first.
    pub(crate) fn roll_for_op(&mut self, request: RollRequest, then: Continuation) -> Result<()> {
        let actor = request.actor.clone().unwrap_or_default();
        if let RollKind::Skill { skill } = &request.kind {
            if self.actor(&actor)?.broken() {
                self.emit(Event::RollSkipped {
                    actor,
                    skill: skill.clone(),
                });
                return Ok(());
            }
            // Situational talents are offered here as in the roll dialog
            // (A-10).
            let mut options: Vec<Choice> = self
                .fitting_talents(&actor, skill)?
                .into_iter()
                .chain(self.situational_talents(&actor, skill)?)
                .map(Choice::UseTalent)
                .collect();
            if !options.is_empty() {
                options.push(Choice::Accept);
                let pending = self.next_pending();
                self.state.pending.insert(
                    pending,
                    Pending::StartRoll {
                        request: Box::new(request),
                        then: Some(then),
                        options: options.clone(),
                    },
                );
                self.emit(Event::Decision {
                    pending,
                    actor: Some(actor),
                    options,
                });
                return Ok(());
            }
        }
        self.start_roll_with(request, Role::Single, Some(then))
    }

    /// Optional talents the actor could use for a roll with this skill:
    /// another attribute or skill, or Breakthrough.
    pub(crate) fn fitting_talents(&self, actor: &str, skill: &str) -> Result<Vec<Id>> {
        let effects = self.effects(actor)?;
        let mut talents = Vec::new();
        for usable in active::replacements(&effects) {
            let fits = match usable.replaces {
                Replaceable::AttributeFor(s) | Replaceable::Skill(s) | Replaceable::Roll(s) => {
                    s == skill
                }
                _ => false,
            };
            let held = self.actor(actor)?.talents.contains(&usable.owner);
            if fits && held && self.uses_left(actor, &usable.owner)? > 0 {
                talents.push(usable.owner);
            }
        }
        Ok(talents)
    }

    /// Talents with a situational bonus that fits a skill roll (A-10).
    pub(crate) fn situational_talents(&self, actor: &str, skill: &str) -> Result<Vec<Id>> {
        let attribute = self
            .engine
            .skill(skill)
            .map(|s| s.attribute.clone())
            .ok_or_else(|| RuleError::UnknownEntry(skill.to_owned()))?;
        let matcher = RollMatch {
            skill: Some(skill),
            attribute: Some(&attribute),
            kind: None,
        };
        let mut talents: Vec<Id> = active::situational(&self.effects(actor)?, matcher)
            .into_iter()
            .map(|(owner, _)| owner)
            .collect();
        talents.dedup();
        Ok(talents)
    }

    pub(crate) fn start_roll_with(
        &mut self,
        request: RollRequest,
        role: Role,
        then: Option<Continuation>,
    ) -> Result<()> {
        match &request.kind {
            RollKind::Supply { supply } => self.supply_roll(&request, *supply),
            RollKind::StressDice { count } => {
                let dice = self.throw(0, *count);
                let id = self.next_pending();
                let facehuggers = count_facehuggers(&dice);
                // No Panic Roll outside skill rolls (F-40).
                self.emit(Event::DiceThrown {
                    roll: id,
                    dice,
                    facehuggers,
                });
                Ok(())
            }
            _ => {
                let roll = self.build(request, role, then)?;
                self.throw_and_go(roll)
            }
        }
    }

    /// Builds the pool [p. 58–63]: attribute + skill + modifications, and
    /// Stress Dice equal to the STRESS LEVEL for human PCs' skill rolls.
    pub(crate) fn build(
        &mut self,
        request: RollRequest,
        role: Role,
        then: Option<Continuation>,
    ) -> Result<RollState> {
        let engine = self.engine;
        if let Purpose::StopPanic { target } = &request.purpose {
            // Someone else's successful COMMAND roll (F-36) [p. 71, 104].
            let is_command =
                matches!(&request.kind, RollKind::Skill { skill } if skill == "command");
            if !is_command || request.actor.as_ref() == Some(target) {
                return Err(RuleError::NotAllowed(
                    "panic is stopped by another character's COMMAND roll (F-36)".into(),
                ));
            }
            if self.actor(target)?.panic.is_none() {
                return Err(RuleError::NotAllowed(format!(
                    "\"{target}\" is not panicking"
                )));
            }
        }
        let actor_id = request.actor.clone();
        let actor = match &actor_id {
            Some(id) => Some(self.actor(id)?.clone()),
            None if matches!(request.kind, RollKind::FixedPool { .. }) => None,
            None => return Err(RuleError::NotAllowed("this roll needs an actor".into())),
        };

        if let (RollKind::Skill { .. }, Some(a)) = (&request.kind, &actor) {
            if a.broken() {
                return Err(RuleError::NotAllowed(
                    "a Broken character cannot roll for skills [p. 98]".into(),
                ));
            }
        }

        let (mut skill, mut attribute) = match &request.kind {
            RollKind::Skill { skill } => {
                let entry = engine
                    .skill(skill)
                    .ok_or_else(|| RuleError::UnknownEntry(skill.clone()))?;
                (Some(skill.clone()), Some(entry.attribute.clone()))
            }
            RollKind::Death => (Some("stamina".to_owned()), Some("strength".to_owned())),
            RollKind::Attribute { attribute } => (None, Some(attribute.clone())),
            _ => (None, None),
        };

        // Talents chosen before the roll. Those with a situational bonus
        // (A-10) count once skill and attribute are settled.
        let mut automatic = false;
        let mut situational = Vec::new();
        // Each talent counts once.
        if let Some(twice) = request
            .talents
            .iter()
            .enumerate()
            .find_map(|(i, t)| request.talents[..i].contains(t).then_some(t))
        {
            return Err(RuleError::NotAllowed(format!(
                "\"{twice}\" is chosen twice"
            )));
        }
        for talent in &request.talents {
            let actor_id = actor_id.as_deref().unwrap_or_default();
            let Some(a) = &actor else {
                return Err(RuleError::NotAllowed("talents need an actor".into()));
            };
            if !a.talents.contains(talent) {
                return Err(RuleError::NotAllowed(format!(
                    "\"{actor_id}\" lacks \"{talent}\""
                )));
            }
            let effects = self.effects(actor_id)?;
            let fitting = active::replacements(&effects).into_iter().find_map(|u| {
                if u.owner != *talent {
                    return None;
                }
                match (u.replaces, u.with) {
                    (Replaceable::Skill(s), ReplaceWith::Skill(other))
                        if skill.as_ref() == Some(s) =>
                    {
                        Some((Some(other.clone()), None, false))
                    }
                    (Replaceable::AttributeFor(s), ReplaceWith::Attribute(a))
                        if skill.as_ref() == Some(s) =>
                    {
                        Some((None, Some(a.clone()), false))
                    }
                    (Replaceable::Roll(s), ReplaceWith::Success)
                        if skill.as_ref() == Some(s)
                            && matches!(request.kind, RollKind::Skill { .. }) =>
                    {
                        Some((None, None, true))
                    }
                    _ => None,
                }
            });
            match fitting {
                Some((Some(new_skill), _, _)) => {
                    attribute = engine.skill(&new_skill).map(|s| s.attribute.clone());
                    skill = Some(new_skill);
                }
                Some((None, Some(new_attribute), _)) => attribute = Some(new_attribute),
                Some((None, None, true)) => {
                    if self.uses_left(actor_id, talent)? == 0 {
                        return Err(RuleError::NotAllowed(format!("\"{talent}\" is used up")));
                    }
                    self.spend_use(actor_id, talent)?;
                    automatic = true;
                }
                _ if engine.has_situation(talent) => situational.push(talent.clone()),
                _ => {
                    return Err(RuleError::NotAllowed(format!(
                        "\"{talent}\" does not apply to this roll"
                    )));
                }
            }
        }

        let human = actor.as_ref().is_some_and(|a| a.human_pc());
        let is_skill = matches!(request.kind, RollKind::Skill { .. });
        let mut modifications = request.modifiers.clone();
        let (mut base, stress) = match (&request.kind, &actor) {
            (RollKind::FixedPool { dice }, _) => (i16::from(*dice), 0),
            (RollKind::Attribute { .. }, Some(a)) => (
                i16::from(a.attribute(attribute.as_deref().unwrap_or_default())),
                0,
            ),
            (RollKind::Skill { .. } | RollKind::Death, Some(a)) => {
                let value = a
                    .attribute(attribute.as_deref().unwrap_or_default())
                    .saturating_add(a.skill_level(skill.as_deref().unwrap_or_default()));
                let stress = if is_skill && human { a.stress } else { 0 };
                (i16::from(value), stress)
            }
            _ => (0, 0),
        };
        if request.helpers > 0 && matches!(request.kind, RollKind::Skill { .. }) {
            modifications.push(Modification {
                source: "help".into(),
                value: i8::try_from(request.helpers.min(engine.constant_u8("help_max")))
                    .unwrap_or(0),
            });
        }
        let matcher = RollMatch {
            skill: skill.as_deref(),
            attribute: attribute.as_deref(),
            kind: None,
        };
        let mut max_pushes = 0;
        if let (Some(id), true) = (&actor_id, skill.is_some()) {
            modifications.extend(self.effect_modifiers(id, matcher)?);
            let offered = active::situational(&self.effects(id)?, matcher);
            for talent in &situational {
                let mut fits = offered
                    .iter()
                    .filter(|(owner, _)| owner == talent)
                    .peekable();
                if fits.peek().is_none() {
                    return Err(RuleError::NotAllowed(format!(
                        "\"{talent}\" does not apply to this roll"
                    )));
                }
                modifications.extend(fits.map(|(_, m)| m.clone()));
            }
            if is_skill && human && !matches!(role, Role::Defender { .. }) {
                max_pushes = engine.constant_u8("base_pushes")
                    + active::extra_pushes(&self.effects(id)?, matcher);
            }
        }
        // Saturating: absurd inputs must not overflow.
        base = modifications
            .iter()
            .fold(base, |sum, m| sum.saturating_add(i16::from(m.value)));
        // A negative overhang removes Stress Dice [p. 62].
        let mut stress_dice = stress;
        let mut removed = 0;
        if base < 0 {
            removed = u8::try_from(-base).unwrap_or(u8::MAX).min(stress);
            stress_dice -= removed;
            base = 0;
        }
        let base_dice = u8::try_from(base).unwrap_or(u8::MAX);

        let id = self.next_pending();
        self.emit(Event::RollStarted {
            roll: id,
            actor: actor_id,
            skill: skill.clone(),
            attribute: attribute.clone(),
            base_dice,
            stress_dice,
            stress_dice_removed: removed,
            modifications,
            max_pushes,
        });
        Ok(RollState {
            id,
            request,
            skill,
            attribute,
            base_dice,
            stress_dice,
            dice: Vec::new(),
            pushes: 0,
            max_pushes,
            facehuggers: 0,
            facehuggers_ignored: false,
            panicked: false,
            cancelled: false,
            story_point: false,
            automatic,
            role,
            then,
            offered: Vec::new(),
        })
    }

    fn throw_and_go(&mut self, mut roll: RollState) -> Result<()> {
        if roll.automatic || (roll.base_dice == 0 && roll.stress_dice == 0) {
            // No dice: the roll cannot succeed (F-21) [p. 62].
            return self.finish(roll);
        }
        roll.dice = self.throw(roll.base_dice, roll.stress_dice);
        roll.facehuggers = count_facehuggers(&roll.dice);
        self.emit(Event::DiceThrown {
            roll: roll.id,
            dice: roll.dice.clone(),
            facehuggers: roll.facehuggers,
        });
        self.after_throw(roll)
    }

    /// Facehuggers first: The Long Haul or panic [p. 61, 76]. Otherwise
    /// offer push and Story Point.
    fn after_throw(&mut self, roll: RollState) -> Result<()> {
        if self.facehuggers_count(&roll) {
            let actor = roll.request.actor.clone().unwrap_or_default();
            let long_haul =
                self.usable_talent(&actor, |r, _| *r == Replaceable::FacehuggersInRoll)?;
            if long_haul.is_some() {
                return self.wait(roll, vec![Choice::IgnoreFacehuggers, Choice::Accept]);
            }
            return self.panic_from_roll(roll);
        }
        self.offer_or_finish(roll)
    }

    /// Facehuggers that matter: on a human PC's skill roll, not ignored.
    fn facehuggers_count(&self, roll: &RollState) -> bool {
        let human = roll
            .request
            .actor
            .as_deref()
            .and_then(|a| self.state.actor(a))
            .is_some_and(|a| a.human_pc());
        human
            && roll.is_skill()
            && roll.facehuggers > 0
            && !roll.facehuggers_ignored
            && !roll.panicked
    }

    fn can_push(&self, roll: &RollState) -> bool {
        roll.is_skill()
            && !roll.panicked
            && !roll.cancelled
            && roll.pushes < roll.max_pushes
            && (roll.facehuggers == 0 || roll.facehuggers_ignored)
    }

    /// Story Point: Cinematic only, one per skill roll (F-06), not without
    /// dice (F-21), not against a cancellation by panic (F-37) [p. 61].
    fn can_spend_story_point(&self, roll: &RollState) -> bool {
        let player = roll
            .request
            .actor
            .as_deref()
            .and_then(|a| self.state.actor(a))
            .and_then(|a| a.player.as_ref());
        self.state.mode == Mode::Cinematic
            && roll.is_skill()
            && !roll.story_point
            && !roll.automatic
            && !roll.cancelled
            && !roll.dice.is_empty()
            && player.is_some_and(|p| self.state.story_points.get(p).copied().unwrap_or(0) > 0)
    }

    fn offer_or_finish(&mut self, roll: RollState) -> Result<()> {
        let mut options = Vec::new();
        if self.can_push(&roll) {
            options.push(Choice::Push);
        }
        if self.can_spend_story_point(&roll) {
            options.push(Choice::StoryPoint);
        }
        if options.is_empty() {
            return self.finish(roll);
        }
        options.push(Choice::Accept);
        self.wait(roll, options)
    }

    fn wait(&mut self, mut roll: RollState, options: Vec<Choice>) -> Result<()> {
        roll.offered = options.clone();
        self.emit(Event::Decision {
            pending: roll.id,
            actor: roll.request.actor.clone(),
            options,
        });
        self.state
            .pending
            .insert(roll.id, Pending::Roll(Box::new(roll)));
        Ok(())
    }

    pub(crate) fn decide_roll(&mut self, mut roll: RollState, choice: Choice) -> Result<()> {
        let actor = roll.request.actor.clone().unwrap_or_default();
        let invalid = |c: &Choice| {
            Err(RuleError::InvalidChoice(format!(
                "{c:?} is not possible now"
            )))
        };
        if !roll.offered.contains(&choice) {
            return invalid(&choice);
        }
        roll.offered.clear();
        match choice {
            Choice::IgnoreFacehuggers => {
                if !self.facehuggers_count(&roll) {
                    return invalid(&choice);
                }
                let Some(talent) =
                    self.usable_talent(&actor, |r, _| *r == Replaceable::FacehuggersInRoll)?
                else {
                    return invalid(&choice);
                };
                self.spend_use(&actor, &talent)?;
                roll.facehuggers_ignored = true;
                self.offer_or_finish(roll)
            }
            Choice::Push => {
                if self.facehuggers_count(&roll) || !self.can_push(&roll) {
                    return invalid(&choice);
                }
                // Stress +1 before re-rolling [p. 61]; successes stay, the
                // rest is thrown again with one new Stress Die (F-20).
                self.change_stress(&actor, 1, "push")?;
                roll.pushes += 1;
                self.emit(Event::Pushed {
                    roll: roll.id,
                    push: roll.pushes,
                });
                for die in &mut roll.dice {
                    die.locked = die.success();
                }
                roll.dice.push(Die {
                    kind: DieKind::Stress,
                    face: 1,
                    locked: false,
                });
                roll.stress_dice = roll.stress_dice.saturating_add(1);
                let mut thrown = Vec::new();
                for die in roll.dice.iter_mut().filter(|d| !d.locked) {
                    die.face = self.dice.d6();
                    thrown.push(*die);
                }
                roll.facehuggers = count_facehuggers(&thrown);
                // The Long Haul covers the throw it was used on (F-47).
                let whole_roll = self.engine.ruling_option(&self.state, "F-47") == "including_push";
                roll.facehuggers_ignored = roll.facehuggers_ignored && whole_roll;
                self.emit(Event::DiceThrown {
                    roll: roll.id,
                    dice: roll.dice.clone(),
                    facehuggers: roll.facehuggers,
                });
                self.after_throw(roll)
            }
            Choice::StoryPoint => {
                if self.facehuggers_count(&roll) || !self.can_spend_story_point(&roll) {
                    return invalid(&choice);
                }
                let player = self.actor(&actor)?.player.clone().unwrap_or_default();
                if let Some(points) = self.state.story_points.get_mut(&player) {
                    *points -= 1;
                }
                roll.story_point = true;
                self.emit(Event::StoryPointSpent {
                    roll: roll.id,
                    player,
                });
                self.finish(roll)
            }
            Choice::Accept => {
                if self.facehuggers_count(&roll) {
                    self.panic_from_roll(roll)
                } else {
                    self.finish(roll)
                }
            }
            other => invalid(&other),
        }
    }

    /// Facehuggers: no push, immediate Panic Roll; 10+ cancels the action
    /// (F-44 for 1–9) [p. 61, 104].
    fn panic_from_roll(&mut self, mut roll: RollState) -> Result<()> {
        roll.panicked = true;
        let actor = roll.request.actor.clone().unwrap_or_default();
        let cause = PanicCause::SkillRoll {
            slow_action: roll.request.action == Some(Speed::Slow),
        };
        roll.cancelled = self.panic_roll(&actor, cause)?;
        if self.can_spend_story_point(&roll) {
            return self.wait(roll, vec![Choice::StoryPoint, Choice::Accept]);
        }
        self.finish(roll)
    }

    fn finish(&mut self, roll: RollState) -> Result<()> {
        let successes = roll.successes();
        let (outcome, against, stunts) = if roll.cancelled {
            (Outcome::Cancelled, None, 0)
        } else {
            match roll.role {
                // Each defender success cancels one; a tie fails [p. 64, 70].
                Role::Attacker {
                    against: Some(against),
                } => {
                    let net = successes.saturating_sub(against);
                    let outcome = if successes >= 1 && net >= 1 {
                        Outcome::Success
                    } else {
                        Outcome::Failure
                    };
                    (outcome, Some(against), net.saturating_sub(1))
                }
                _ => {
                    let outcome = if successes >= 1 {
                        Outcome::Success
                    } else {
                        Outcome::Failure
                    };
                    (outcome, None, successes.saturating_sub(1))
                }
            }
        };
        self.emit(Event::RollFinished {
            roll: roll.id,
            outcome,
            successes,
            against,
            stunts,
        });

        if let Some(actor) = roll.request.actor.as_deref() {
            if let Some(skill) = roll.skill.as_deref() {
                // Triggers on skill rolls, e.g. Crotch hit [p. 100].
                self.fire_triggers(actor, |on| {
                    matches!(on, TriggerEvent::SkillRoll(skills) if skills.iter().any(|s| s == skill))
                })?;
            }
            if let Purpose::StopPanic { target } = &roll.request.purpose {
                if outcome == Outcome::Success {
                    self.stop_panic(
                        target,
                        StopReason::Command {
                            by: actor.to_owned(),
                        },
                    )?;
                }
            }
        }
        if let Some(then) = roll.then {
            let ops = if outcome == Outcome::Success {
                then.on_success
            } else {
                then.on_fail
            };
            if !ops.is_empty() {
                self.queue.push_back(Task::Ops {
                    actor: then.actor,
                    owner: then.owner,
                    ops,
                    flags: OpFlags {
                        from_condition: then.from_condition,
                        cancelled_slow: false,
                    },
                });
            }
        }
        if let Role::Defender { mut attacker } = roll.role {
            // A cancelled defence counts as no successes (F-22).
            let against = if outcome == Outcome::Cancelled {
                0
            } else {
                successes
            };
            attacker.role = Role::Attacker {
                against: Some(against),
            };
            return self.throw_and_go(*attacker);
        }
        Ok(())
    }

    /// Supply roll [p. 34]: Stress Dice equal to the rating, at most six;
    /// each facehugger lowers the rating by one; no panic (F-40). Talents
    /// such as Calm Breather take dice away, but leave at least one (F-09).
    fn supply_roll(&mut self, request: &RollRequest, supply: Supply) -> Result<()> {
        let actor = request
            .actor
            .clone()
            .ok_or_else(|| RuleError::NotAllowed("a supply roll needs an actor".into()))?;
        let rating = self
            .actor(&actor)?
            .supplies
            .get(&supply)
            .copied()
            .unwrap_or(0);
        if rating == 0 {
            return Err(RuleError::NotAllowed(format!(
                "\"{actor}\" is out of {supply:?}"
            )));
        }
        let kind = match supply {
            Supply::Air => DataRollKind::AirSupply,
            Supply::Food => DataRollKind::FoodSupply,
            Supply::Water => DataRollKind::WaterSupply,
            Supply::Power => DataRollKind::PowerSupply,
        };
        let matcher = RollMatch {
            skill: None,
            attribute: None,
            kind: Some(kind),
        };
        let effects = self.effects(&actor)?;
        let change: i16 = active::roll_modifiers(&effects, matcher)
            .iter()
            .map(|m| i16::from(m.value))
            .sum();
        let floor = i16::from(active::floor(&effects, matcher).unwrap_or(0));
        let rating16 = i16::from(rating);
        let cap = self.engine.constant("supply_dice_max");
        // Order of cap and talent: F-09.
        let count = if self.engine.ruling_option(&self.state, "F-09") == "minus_then_cap" {
            (rating16 + change).min(cap).max(floor)
        } else {
            (rating16.min(cap) + change).max(floor)
        };
        let dice = self.throw(0, u8::try_from(count).unwrap_or(0));
        let lost = count_facehuggers(&dice);
        let to = rating.saturating_sub(lost);
        self.actor_mut(&actor)?.supplies.insert(supply, to);
        self.emit(Event::SupplyRolled {
            actor,
            supply,
            dice,
            from: rating,
            to,
        });
        Ok(())
    }
}

fn count_facehuggers(dice: &[Die]) -> u8 {
    u8::try_from(dice.iter().filter(|d| d.facehugger()).count()).unwrap_or(u8::MAX)
}

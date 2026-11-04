//! Helpers for the engine tests: real rule data, scripted dice.
#![allow(dead_code)]

use std::path::PathBuf;

use xenoforge_engine::data::RuleData;
use xenoforge_engine::dice::ScriptedDice;
use xenoforge_engine::roll::{Outcome, RollRequest};
use xenoforge_engine::state::{Actor, GameState, Mode, PendingId};
use xenoforge_engine::{Choice, Command, Engine, Event, Modification, RuleError};

pub fn rules() -> RuleData {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/rules");
    RuleData::load(&dir).unwrap_or_else(|e| panic!("{e}"))
}

/// A table: engine plus game state.
pub struct Table {
    pub engine: Engine,
    pub state: GameState,
}

impl Table {
    pub fn new(mode: Mode) -> Self {
        Self {
            engine: Engine::new(rules()),
            state: GameState::new(mode),
        }
    }

    pub fn with(mut self, id: &str, actor: Actor) -> Self {
        self.state.actors.insert(id.to_owned(), actor);
        self
    }

    pub fn actor(&self, id: &str) -> &Actor {
        self.state.actor(id).expect("actor exists")
    }

    /// Runs a command with exactly these die results.
    pub fn try_run(&mut self, command: Command, faces: &[u8]) -> Result<Vec<Event>, RuleError> {
        let mut dice = ScriptedDice::new(faces.iter().copied());
        let events = self.engine.apply(&mut self.state, command, &mut dice)?;
        assert_eq!(dice.remaining(), 0, "unused dice; events: {events:#?}");
        Ok(events)
    }

    pub fn run(&mut self, command: Command, faces: &[u8]) -> Vec<Event> {
        self.try_run(command, faces)
            .unwrap_or_else(|e| panic!("command failed: {e}"))
    }

    pub fn roll(&mut self, request: RollRequest, faces: &[u8]) -> Vec<Event> {
        self.run(Command::Roll { request }, faces)
    }

    /// Answers the last open decision in `events`.
    pub fn decide(&mut self, events: &[Event], choice: Choice, faces: &[u8]) -> Vec<Event> {
        let pending = pending(events);
        self.run(Command::Decide { pending, choice }, faces)
    }

    pub fn try_decide(
        &mut self,
        events: &[Event],
        choice: Choice,
        faces: &[u8],
    ) -> Result<Vec<Event>, RuleError> {
        let pending = pending(events);
        self.try_run(Command::Decide { pending, choice }, faces)
    }
}

/// The last decision, question or target choice in the events.
pub fn pending(events: &[Event]) -> PendingId {
    events
        .iter()
        .rev()
        .find_map(|e| match e {
            Event::Decision { pending, .. }
            | Event::Question { pending, .. }
            | Event::ChooseTargets { pending, .. } => Some(*pending),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no open decision in {events:#?}"))
}

/// Options of the last decision, or none.
pub fn options(events: &[Event]) -> Vec<Choice> {
    events
        .iter()
        .rev()
        .find_map(|e| match e {
            Event::Decision { options, .. } => Some(options.clone()),
            _ => None,
        })
        .unwrap_or_default()
}

/// Outcome, successes and stunts of the last finished roll.
pub fn finished(events: &[Event]) -> (Outcome, u8, u8) {
    events
        .iter()
        .rev()
        .find_map(|e| match e {
            Event::RollFinished {
                outcome,
                successes,
                stunts,
                ..
            } => Some((*outcome, *successes, *stunts)),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no finished roll in {events:#?}"))
}

/// Base dice, Stress Dice, removed Stress Dice and modifications of the
/// first roll started in the events.
pub fn started(events: &[Event]) -> (u8, u8, u8, Vec<Modification>) {
    events
        .iter()
        .find_map(|e| match e {
            Event::RollStarted {
                base_dice,
                stress_dice,
                stress_dice_removed,
                modifications,
                ..
            } => Some((
                *base_dice,
                *stress_dice,
                *stress_dice_removed,
                modifications.clone(),
            )),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no roll started in {events:#?}"))
}

/// Value and effect of the last panic effect.
pub fn panic_effect(events: &[Event]) -> (u8, String) {
    events
        .iter()
        .rev()
        .find_map(|e| match e {
            Event::PanicEffect { value, effect, .. } => Some((*value, effect.clone())),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no panic effect in {events:#?}"))
}

pub fn has<F: Fn(&Event) -> bool>(events: &[Event], f: F) -> bool {
    events.iter().any(f)
}

/// A human PC with Health, so that nobody starts out Broken.
pub fn pc() -> Actor {
    Actor::pc().attr("strength", 4)
}

/// An NPC with Health.
pub fn npc() -> Actor {
    Actor::npc().attr("strength", 3)
}

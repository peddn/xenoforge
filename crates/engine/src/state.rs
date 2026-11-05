//! Game state the engine works on. Everything here can be stored as JSON;
//! the rule data itself is not part of it, only references by id.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::data::Id;
use crate::data::effects::{Blocked, Cold, EndEvent, Gear, Op, Unit, Who};
use crate::roll::RollState;

pub type ActorId = String;
pub type PlayerId = String;
pub type PendingId = u32;

/// Cinematic or Campaign play [p. 19].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    #[default]
    Cinematic,
    Campaign,
}

/// The state of one team's game.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub mode: Mode,
    /// Chosen option per ruling; rulings not listed use their default.
    pub rulings: BTreeMap<Id, Id>,
    pub actors: BTreeMap<ActorId, Actor>,
    /// Story Points belong to the player, not the character [p. 61].
    pub story_points: BTreeMap<PlayerId, u8>,
    /// Acts ended so far in Cinematic play; the current Act is the next
    /// one [p. 216].
    #[serde(default)]
    pub acts_ended: u8,
    /// Decisions the engine waits for.
    pub pending: BTreeMap<PendingId, Pending>,
    pub next_pending: PendingId,
}

impl GameState {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            ..Self::default()
        }
    }

    /// Adds an actor (builder style).
    pub fn with(mut self, id: &str, actor: Actor) -> Self {
        self.actors.insert(id.to_owned(), actor);
        self
    }

    pub fn actor(&self, id: &str) -> Option<&Actor> {
        self.actors.get(id)
    }
}

/// Player character or non-player character [p. 62].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorKind {
    #[default]
    Pc,
    Npc,
}

/// Consumables with a Supply rating [p. 34].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Supply {
    Air,
    Food,
    Water,
    Power,
}

/// A character or creature with dice to roll. Missing fields take their
/// defaults when read from JSON.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Actor {
    pub kind: ActorKind,
    pub android: bool,
    /// An android that poses as human; others do not learn it until the
    /// reveal (A-14) [p. 29, 103].
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub covert: bool,
    /// Player who controls the character, for Story Points.
    pub player: Option<PlayerId>,
    pub attributes: BTreeMap<Id, u8>,
    pub skills: BTreeMap<Id, u8>,
    pub talents: BTreeSet<Id>,
    /// What a talent names, e.g. the vehicle of Like the Back of Your Hand
    /// [p. 76]; a repeatable talent can name several.
    pub talent_params: BTreeMap<Id, Vec<String>>,
    pub health: u8,
    /// STRESS LEVEL; only human PCs have one [p. 103, 105].
    pub stress: u8,
    pub supplies: BTreeMap<Supply, u8>,
    pub wearing: BTreeSet<Gear>,
    /// Traumas, conditions, injuries and blocks currently in effect.
    pub sources: Vec<ActiveSource>,
    pub panic: Option<ActivePanic>,
    /// Talent uses in the current Act or session.
    pub uses: BTreeMap<Id, u8>,
    /// Signature item used in the current Act or session [p. 104].
    pub signature_item_used: bool,
    /// Rolled 13 or more on a Panic Roll this session [p. 104].
    pub panic_13_plus: bool,
    /// The next slow action is lost [p. 105]; the combat rules of phase 3
    /// use it.
    pub loses_next_slow_action: bool,
}

impl Actor {
    pub fn pc() -> Self {
        Self::default()
    }

    pub fn npc() -> Self {
        Self {
            kind: ActorKind::Npc,
            ..Self::default()
        }
    }

    pub fn android(mut self) -> Self {
        self.android = true;
        self
    }

    /// Sets an attribute. STRENGTH also sets Health, which starts equal to
    /// it [p. 30].
    pub fn attr(mut self, id: &str, value: u8) -> Self {
        if id == "strength" {
            self.health = value;
        }
        self.attributes.insert(id.to_owned(), value);
        self
    }

    pub fn skill(mut self, id: &str, level: u8) -> Self {
        self.skills.insert(id.to_owned(), level);
        self
    }

    pub fn talent(mut self, id: &str) -> Self {
        self.talents.insert(id.to_owned());
        self
    }

    pub fn stress(mut self, level: u8) -> Self {
        self.stress = level;
        self
    }

    pub fn health(mut self, health: u8) -> Self {
        self.health = health;
        self
    }

    pub fn player(mut self, player: &str) -> Self {
        self.player = Some(player.to_owned());
        self
    }

    pub fn supply(mut self, supply: Supply, rating: u8) -> Self {
        self.supplies.insert(supply, rating);
        self
    }

    /// Has a STRESS LEVEL, rolls Stress Dice, pushes and panics: human PCs
    /// only [p. 29, 62, 105].
    pub fn human_pc(&self) -> bool {
        self.kind == ActorKind::Pc && !self.android
    }

    /// Health 0 [p. 98].
    pub fn broken(&self) -> bool {
        self.health == 0
    }

    pub fn attribute(&self, id: &str) -> u8 {
        self.attributes.get(id).copied().unwrap_or(0)
    }

    pub fn skill_level(&self, id: &str) -> u8 {
        self.skills.get(id).copied().unwrap_or(0)
    }
}

/// A trauma, condition, injury or block on an actor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveSource {
    pub origin: Origin,
    pub ends: Ending,
}

/// What an active source comes from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    Trauma(Id),
    Condition {
        id: Id,
        cold: Option<Cold>,
    },
    Injury(Id),
    /// A `block` operation of the entry `by`.
    Block {
        blocks: Blocked,
        by: Id,
    },
    /// Effect number `index` of `of`, which lasts differently than `of`.
    Part {
        of: Box<Origin>,
        index: usize,
    },
}

impl Origin {
    /// Data entry this origin points to.
    pub fn entry(&self) -> &str {
        match self {
            Origin::Trauma(id) | Origin::Condition { id, .. } | Origin::Injury(id) => id,
            Origin::Block { by, .. } => by,
            Origin::Part { of, .. } => of.entry(),
        }
    }

    /// Label for logs and explanations, e.g. `injury/broken_ribs`.
    pub fn label(&self) -> String {
        match self {
            Origin::Trauma(id) => format!("trauma/{id}"),
            Origin::Condition { id, .. } => format!("condition/{id}"),
            Origin::Injury(id) => format!("injury/{id}"),
            Origin::Block { by, .. } => format!("block/{by}"),
            Origin::Part { of, .. } => of.label(),
        }
    }
}

/// When an active source or panic effect ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Ending {
    Never,
    /// After this many steps of the unit. Days count as four Shifts
    /// (plan section 9.1).
    After {
        unit: Unit,
        left: u16,
    },
    On(EndEvent),
}

/// A running panic effect [p. 104].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivePanic {
    /// Entry in `panic.json`.
    pub effect: Id,
    /// Final value of the Panic Roll, used for More Panic (F-23 to F-26).
    pub value: u8,
    /// When the entry's effects end; `None` once they are over. The panic
    /// itself runs on until it stops (F-25).
    pub effects_end: Option<Ending>,
    /// The character was already Broken when it began (F-48).
    #[serde(default)]
    pub started_broken: bool,
}

/// A decision the engine waits for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Pending {
    Roll(Box<RollState>),
    /// A roll started by an operation, waiting for the choice of an
    /// optional talent such as Stoic.
    StartRoll {
        request: Box<crate::roll::RollRequest>,
        then: Option<crate::roll::Continuation>,
        options: Vec<crate::engine::Choice>,
    },
    /// Use Overkill instead of the panic effect? [p. 104]
    Overkill {
        actor: ActorId,
        value: u8,
        talent: Id,
        effect: Id,
        cancelled_slow: bool,
    },
    /// A yes/no question from an `ask` operation.
    Question {
        actor: ActorId,
        owner: Id,
        question: Id,
        on_no: Vec<Op>,
    },
    /// Who of the candidates is affected? The game master decides.
    Targets {
        actor: ActorId,
        owner: Id,
        who: Who,
        candidates: Vec<ActorId>,
        ops: Vec<Op>,
    },
}

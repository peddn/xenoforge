//! Entry types of the rule data files in `data/rules`.
//!
//! Every entry has an `id` (unique across all files) and the book page it
//! comes from. Names and descriptions are not stored here but in
//! `data/i18n`, under the keys `<id>.name` and `<id>.desc`.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::effects::{Effect, Lasts, Op, Unit};
use super::{Id, Page};

fn is_false(value: &bool) -> bool {
    !*value
}

/// A file in `data/rules`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DataFile<T> {
    /// Path to the JSON Schema of this file, for editors.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub entries: Vec<T>,
}

/// An attribute [p. 27].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Attribute {
    pub id: Id,
    pub page: Page,
}

/// A career [p. 38–55]. Phase 0 only holds what the probe data needs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Career {
    pub id: Id,
    pub page: Page,
    pub key_attribute: Id,
}

/// A skill and the attribute it is based on [p. 58].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Skill {
    pub id: Id,
    pub page: Page,
    pub attribute: Id,
}

/// When an action can be taken.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    /// Slow action in a Round [p. 88].
    Slow,
    /// Fast action in a Round [p. 88].
    Fast,
    /// Space combat, sensor phase [p. 192].
    SpaceSensor,
    /// Space combat, pilot phase [p. 192].
    SpacePilot,
    /// Space combat, gunner phase [p. 192].
    SpaceGunner,
    /// Space combat, engineer phase [p. 192].
    SpaceEngineer,
}

/// An action that effects can block, grant or listen to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub id: Id,
    pub page: Page,
    pub kind: ActionKind,
}

/// A range of results on a table. Without `to` the range is open
/// ("15+"). On D66 tables both ends are D66 results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RollRange {
    pub from: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<u8>,
}

/// A row of the Panic Roll table [p. 104–105].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PanicResult {
    pub id: Id,
    pub page: Page,
    /// Row of the Panic table. Effects outside the table, such as Overkill,
    /// have none and are reached through a replacement [p. 104].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roll: Option<RollRange>,
    /// A panic action replaces the action the roll came from (10+, p. 104).
    #[serde(default, skip_serializing_if = "is_false")]
    pub cancels_action: bool,
    /// Calls for an EMPATHY roll after the session (13+, p. 104; F-01).
    #[serde(default, skip_serializing_if = "is_false")]
    pub trauma_check: bool,
    /// Happens once when the result is rolled.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub on_apply: Vec<Op>,
    /// Works while the panic effect lasts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
    /// How long the panic effect lasts; default for its effects.
    pub lasts: Lasts,
}

/// Switches between the options of a ruling (`rulings.json`). The entry's
/// own `effects` belong to the ruling's default option; `alternatives`
/// holds the effects for every other option.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RulingSwitch {
    pub id: Id,
    pub alternatives: BTreeMap<Id, Vec<Effect>>,
}

/// A permanent mental trauma [p. 101].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Trauma {
    pub id: Id,
    pub page: Page,
    pub d6: u8,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ruling: Option<RulingSwitch>,
}

/// A fatal critical injury needs a Death Roll when the time limit runs
/// out [p. 99].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Fatal {
    /// One step of this unit.
    pub time_limit: Unit,
    /// Modification to the MEDICAL AID roll that saves the character.
    #[serde(default)]
    pub medical_aid_modifier: i8,
}

/// Healing time of a critical injury [p. 100–101]. It is also the default
/// duration of the injury's effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Healing {
    /// No healing time; the effects are instant.
    None,
    /// A number of days rolled with this many D6.
    DaysD6(u8),
    /// Until a MEDICAL AID roll is made.
    UntilMedicalAid,
    Permanent,
}

impl Healing {
    /// The duration this healing time gives to effects.
    pub fn lasts(self) -> Lasts {
        use super::effects::EndEvent;
        match self {
            Healing::None => Lasts::Instant,
            Healing::DaysD6(_) => Lasts::Until(EndEvent::Healed),
            Healing::UntilMedicalAid => Lasts::Until(EndEvent::MedicalAid),
            Healing::Permanent => Lasts::Permanent,
        }
    }
}

/// A row of the critical injury table [p. 100].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CriticalInjury {
    pub id: Id,
    pub page: Page,
    pub d66: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fatal: Option<Fatal>,
    /// Kills outright; no Death Roll [p. 99].
    #[serde(default, skip_serializing_if = "is_false")]
    pub instant_death: bool,
    pub healing: Healing,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub on_apply: Vec<Op>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
}

/// A condition such as Freezing [p. 106–107].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Condition {
    pub id: Id,
    pub page: Page,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
    /// The event that ends the condition.
    pub ends: super::effects::EndEvent,
}

/// A talent [p. 74–79].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Talent {
    pub id: Id,
    pub page: Page,
    /// Career talent of this career; general talent without it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub career: Option<Id>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
}

/// Group of a ship system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SystemCategory {
    /// Core component [p. 170].
    Core,
    /// Module [p. 171].
    Module,
    /// Part of every ship that the damage tables name [p. 198–199].
    Other,
}

/// A part of a ship that damage can hit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ShipSystem {
    pub id: Id,
    pub page: Page,
    pub category: SystemCategory,
    /// What the system does while it works.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
}

/// Which component damage table [p. 197].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DamageTable {
    /// D66 [p. 198].
    Minor,
    /// 2D6 [p. 199].
    Major,
}

/// Rolls needed to repair a component [p. 200].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Repair {
    pub skill: Id,
    pub count: u8,
}

/// A row of a component damage table [p. 198–199]. Its effects last until
/// the component is repaired.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ShipDamage {
    pub id: Id,
    pub page: Page,
    pub table: DamageTable,
    pub roll: RollRange,
    /// The system that is hit. If the ship lacks it, roll again
    /// [p. 197; F-14].
    pub system: Id,
    /// Empty if the damage cannot be repaired.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repair: Vec<Repair>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ruling: Option<RulingSwitch>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub on_apply: Vec<Op>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
}

/// A single rule value, such as the most helpers that count [p. 63].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Constant {
    pub id: Id,
    pub page: Page,
    pub value: i16,
}

/// A ruling from `docs/rules/00-errata-und-festlegungen.md`. The game
/// master can switch between its options per team.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Ruling {
    /// `F-NN`, as in the catalog.
    pub id: Id,
    pub pages: Vec<Page>,
    pub options: Vec<Id>,
    pub default: Id,
}

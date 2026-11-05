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

/// A career [p. 38–55].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Career {
    pub id: Id,
    pub page: Page,
    /// May go to 5 at creation [p. 28].
    pub key_attribute: Id,
    /// May go to 3 at creation [p. 28].
    pub key_skills: Vec<Id>,
    /// The three career talents; one is chosen at creation [p. 29, 74].
    pub talents: Vec<Id>,
    /// Starting cash, rolled once at creation.
    pub cash: Cash,
    /// Suggested names as in the book [p. 30]; not translated.
    pub names: Vec<String>,
    /// Suggestions for appearance, signature item and Personal Agenda.
    /// Their texts are `<career>.look.<id>`, `<career>.item.<id>` and
    /// `<career>.agenda.<id>` (A-11).
    pub looks: Vec<Id>,
    pub items: Vec<Id>,
    pub agendas: Vec<Id>,
    /// Starting gear: four lines of "A or B"; two items from two different
    /// lines (F-60) [p. 33].
    pub gear: Vec<[StartItem; 2]>,
}

/// A starting item: one, `count` of them, or `dice` D6 of them [p. 39–55].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StartItem {
    pub item: Id,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dice: Option<u8>,
}

/// A piece of gear from chapter 6, or an item the careers list without
/// stats (F-64) [p. 39–55, 118–147].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub id: Id,
    pub page: Page,
    pub category: ItemCategory,
    /// How much it encumbers; none: it cannot be carried (F-63) [p. 34, 118].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<Weight>,
    /// Typical price; none where it varies or is not given [p. 118].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<Price>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weapon: Option<WeaponStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub armor: Option<ArmorStats>,
    /// Air or power it holds when full [p. 34].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supply: Option<ItemSupply>,
    /// Skill levels of its own, e.g. a mainframe or the AutoDoc.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub skills: BTreeMap<Id, u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vehicle: Option<VehicleStats>,
    /// Modifiers. Those of a suit count while it is worn; those of other
    /// items are offered in the roll dialog while carried (F-59).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ItemCategory {
    Pistol,
    Rifle,
    HeavyWeapon,
    CloseCombat,
    VehicleWeapon,
    Armor,
    Mainframe,
    DataStorage,
    Diagnostics,
    Vision,
    Tool,
    Medical,
    Pharmaceutical,
    Food,
    Vehicle,
    /// Items of the career lists without stats in chapter 6 (F-64).
    Other,
}

/// Weight as printed [p. 34].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub enum Weight {
    /// Tiny: listed, but does not encumber.
    #[serde(rename = "0")]
    Tiny,
    #[serde(rename = "1/4")]
    Quarter,
    #[serde(rename = "1/2")]
    Half,
    #[serde(rename = "1")]
    One,
    #[serde(rename = "2")]
    Two,
    #[serde(rename = "3")]
    Three,
    #[serde(rename = "4")]
    Four,
}

impl Weight {
    /// In quarters of a regular item.
    pub fn quarters(self) -> u32 {
        match self {
            Self::Tiny => 0,
            Self::Quarter => 1,
            Self::Half => 2,
            Self::One => 4,
            Self::Two => 8,
            Self::Three => 12,
            Self::Four => 16,
        }
    }
}

/// A price in cents, or a range where the book gives one [p. 118].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Price {
    pub cents: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub up_to_cents: Option<u64>,
}

/// Columns of the weapon tables [p. 118].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WeaponStats {
    /// CLOSE COMBAT or RANGED COMBAT.
    pub skill: Id,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub bonus: i8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage: Option<u8>,
    /// Blast Power of the explosion it causes [p. 108].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blast: Option<u8>,
    /// Stuns; the value modifies the STAMINA roll against it [p. 125, 127].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stun: Option<i8>,
    /// Sets the target on fire with this Intensity [p. 108].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fire: Option<u8>,
    pub range: Range,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<WeaponFeature>,
    /// Comes with two spare reloads at the start (F-61) [p. 33].
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub reloads: bool,
}

fn is_zero(value: &i8) -> bool {
    *value == 0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Range {
    Engaged,
    Short,
    Medium,
    Long,
    Extreme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WeaponFeature {
    ArmorPiercing,
    ArmorDoubled,
    FullAuto,
    SingleShot,
}

/// A suit or armor; one is worn at a time [p. 98, 128].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ArmorStats {
    pub rating: u8,
    /// Protects in a vacuum (F-18) [p. 107, 128].
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub vacuum: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ItemSupply {
    pub kind: SupplyKind,
    pub max: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SupplyKind {
    Air,
    Power,
}

/// The vehicle table [p. 140].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VehicleStats {
    /// Without the driver [p. 114].
    pub passengers: u8,
    pub maneuverability: i8,
    pub speed: u8,
    pub hull: u8,
    pub armor: u8,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub aerial: bool,
    /// Standard armament, items with weapon stats.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weapons: Vec<Id>,
}

/// Starting cash: `dice` D6 times `times` dollars.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Cash {
    pub dice: u8,
    pub times: u32,
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
    /// Takes a Turn or more outside the Round order, e.g. studying an
    /// artifact [p. 74, 77].
    Turn,
}

/// An action that effects can block, grant or listen to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub id: Id,
    pub page: Page,
    pub kind: ActionKind,
    /// Aimed at another character, e.g. the order of Pull Rank [p. 76].
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub target: bool,
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
    /// The talent names something of its own: a vehicle or a weapon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameter: Option<TalentParameter>,
    /// Can be taken more than once, each time for another parameter.
    #[serde(default, skip_serializing_if = "is_false")]
    pub repeatable: bool,
    /// The part of the app that will make the talent work; until then it
    /// can be chosen and is shown, without effect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub later: Option<Later>,
}

/// What a talent names, e.g. Like the Back of Your Hand: one vehicle
/// [p. 76].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TalentParameter {
    Vehicle,
    Weapon,
}

/// Parts of the app still to come (plan section 12).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Later {
    /// Equipment, phase 2c.
    Gear,
    /// Combat, phase 3.
    Combat,
    /// Time, rest and campaign, phase 4.
    Time,
    /// Space combat, phase 6.
    SpaceCombat,
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

//! Effects and one-shot operations (plan section 4.4).
//!
//! An [`Effect`] is continuous: it works as long as its source is active
//! (an injury until healed, a ship damage until repaired, a talent forever).
//! An [`Op`] happens once, either when its source is applied or when a
//! trigger or periodic effect fires.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::Id;

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

fn is_false(value: &bool) -> bool {
    !*value
}

/// Who or what a continuous effect works on.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Target {
    /// The character, creature or ship that carries the source.
    #[default]
    Bearer,
    /// The ship itself (stats, actions).
    Ship,
    /// Everyone aboard the ship.
    Aboard,
}

/// A step of game time (plan section 9.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    Round,
    Turn,
    Shift,
    Day,
}

/// An event that ends an effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EndEvent {
    /// Panic stops: COMMAND roll by another, Broken, or one Turn passes [p. 104].
    PanicStops,
    /// The healing time of a critical injury has run out [p. 101].
    Healed,
    /// A successful MEDICAL AID roll was made on the character [p. 100].
    MedicalAid,
    /// The damaged ship system is repaired [p. 200].
    Repaired,
    /// The character is warm again [p. 107].
    Warm,
    /// The next change of day (Night to Morning).
    DayChange,
}

/// How long an effect lasts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Lasts {
    /// Only the one-shot operations happen; nothing continues.
    Instant,
    /// A number of steps of one time unit.
    Steps { unit: Unit, count: u8 },
    /// Until an event happens.
    Until(EndEvent),
    /// For good.
    Permanent,
    /// As long as the source exists (talent, installed module).
    WhileSource,
}

/// Kinds of rolls that are not skill rolls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RollKind {
    /// Panic Roll; a modifier changes the result [p. 104].
    Panic,
    /// Supply rolls; a modifier changes the number of Stress Dice [p. 34].
    AirSupply,
    FoodSupply,
    WaterSupply,
    PowerSupply,
}

/// What a modifier changes on the rolls it applies to.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Modified {
    /// The dice pool (the result for Panic Rolls, the value for ship stats).
    #[default]
    Dice,
    /// How often the roll may be pushed [p. 60].
    Pushes,
}

/// Ship stats [p. 167].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ShipStat {
    FtlRating,
    Signature,
    Thrusters,
    Hull,
    Armor,
}

/// What a modifier applies to. At least one list must be filled.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ModTarget {
    /// Skill rolls with these skills.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<Id>,
    /// Skill rolls based on these attributes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<Id>,
    /// Rolls that are not skill rolls.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rolls: Vec<RollKind>,
    /// Ship stats.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ship_stats: Vec<ShipStat>,
    /// Values of a character.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stats: Vec<ActorStat>,
}

/// Values of a character that effects can change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActorStat {
    /// Full Health, normally STRENGTH [p. 30].
    HealthMax,
    /// How much a character carries without being encumbered [p. 34, 79].
    CarryCapacity,
}

/// How a modifier changes a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Change {
    Add(i8),
    Multiply(u8),
    Divide(u8),
}

/// Something a restriction or block op forbids.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Blocked {
    /// An action from `actions.json`.
    Action(Id),
    /// Relieving stress [p. 104].
    StressRelief,
    /// Recovering Health [p. 98].
    HealthRecovery,
}

/// The event a trigger listens to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TriggerEvent {
    /// The bearer makes a skill roll with one of these skills.
    SkillRoll(Vec<Id>),
    /// The bearer performs this action.
    Action(Id),
    /// The bearer sleeps.
    Sleep,
}

/// Roll intervals for Freezing [p. 107].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ColdIntervals {
    pub above_freezing: Unit,
    pub sub_zero: Unit,
    pub deep_space: Unit,
}

/// How often a periodic effect fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Interval {
    /// Every step of this unit.
    Step(Unit),
    /// Depends on how cold it is.
    ByCold(ColdIntervals),
}

/// How cold it is [p. 107].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cold {
    AboveFreezing,
    SubZero,
    DeepSpace,
}

/// Gear that exempts a character from an effect while it is worn.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum Gear {
    /// Any suit that protects in a vacuum; the book says "pressure suit"
    /// [p. 198]. Which items count is ruling F-18.
    VacuumSuit,
}

/// What a replacement effect replaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Replaceable {
    /// All facehugger results (ones on Stress Dice) of a single roll.
    FacehuggersInRoll,
    /// The effect of a Panic Roll with at least this result.
    PanicResult { from: u8 },
    /// A whole skill roll with this skill: it is not rolled.
    Roll(Id),
    /// The attribute of skill rolls with this skill.
    AttributeFor(Id),
    /// This skill.
    Skill(Id),
}

/// What happens instead. Each kind of `Replaceable` has one matching kind
/// here (checked by the validation).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReplaceWith {
    /// For `facehuggers_in_roll`: they count as blank.
    Ignore,
    /// For `panic_result`: this entry of `panic.json`.
    PanicEffect(Id),
    /// For `roll`: the roll succeeds with one success.
    Success,
    /// For `attribute_for`: roll with this attribute.
    Attribute(Id),
    /// For `skill`: roll with this skill.
    Skill(Id),
}

/// The period a limited use refers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UsePeriod {
    /// Once per Act in Cinematic play, once per session in Campaign play.
    ActOrSession,
    Session,
}

/// A limit on how often something can be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Uses {
    pub count: u8,
    pub per: UsePeriod,
}

/// How a hint is shown in the overview (plan section 7.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HintTier {
    /// The game master has to act on it.
    Hint,
    /// No game consequence.
    Info,
}

/// A continuous effect (plan section 4.4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Effect {
    /// Changes the dice pool of rolls or a ship stat.
    Modifier {
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
        applies_to: ModTarget,
        #[serde(default, skip_serializing_if = "is_default")]
        modifies: Modified,
        change: Change,
        /// The changed value never goes below this, e.g. "to a minimum of
        /// one die" [p. 78].
        #[serde(default, skip_serializing_if = "Option::is_none")]
        at_least: Option<u8>,
        /// Applies only in a situation the app cannot see, e.g. "when
        /// moving undetected"; the player ticks it for a roll (A-10).
        /// Translation key of the situation.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        when: Option<Id>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lasts: Option<Lasts>,
        #[serde(default, skip_serializing_if = "is_false")]
        gm_only: bool,
    },
    /// Forbids an action or a kind of recovery.
    Restriction {
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
        blocks: Blocked,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lasts: Option<Lasts>,
        #[serde(default, skip_serializing_if = "is_false")]
        gm_only: bool,
    },
    /// Runs operations whenever an event happens.
    Trigger {
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
        on: TriggerEvent,
        then: Vec<Op>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lasts: Option<Lasts>,
        #[serde(default, skip_serializing_if = "is_false")]
        gm_only: bool,
    },
    /// Runs operations on every step of a time unit.
    Periodic {
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
        every: Interval,
        then: Vec<Op>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lasts: Option<Lasts>,
        #[serde(default, skip_serializing_if = "is_false")]
        gm_only: bool,
    },
    /// Replaces an outcome ("instead").
    Replacement {
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
        replaces: Replaceable,
        with: ReplaceWith,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        uses: Option<Uses>,
        /// The player decides whether to use it.
        #[serde(default, skip_serializing_if = "is_false")]
        optional: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lasts: Option<Lasts>,
        #[serde(default, skip_serializing_if = "is_false")]
        gm_only: bool,
    },
    /// Puts a condition from `conditions.json` on the target.
    Condition {
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
        condition: Id,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cold: Option<Cold>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        unless_wearing: Vec<Gear>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lasts: Option<Lasts>,
        #[serde(default, skip_serializing_if = "is_false")]
        gm_only: bool,
    },
    /// Makes an action possible while the source works.
    Capability {
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
        grants: Id,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lasts: Option<Lasts>,
        #[serde(default, skip_serializing_if = "is_false")]
        gm_only: bool,
    },
    /// Shows a note; the game master acts on it.
    Hint {
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
        /// Translation key of the note.
        text: Id,
        tier: HintTier,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lasts: Option<Lasts>,
        #[serde(default, skip_serializing_if = "is_false")]
        gm_only: bool,
    },
}

impl Effect {
    /// The effect kind as written in the data.
    pub fn kind(&self) -> &'static str {
        match self {
            Effect::Modifier { .. } => "modifier",
            Effect::Restriction { .. } => "restriction",
            Effect::Trigger { .. } => "trigger",
            Effect::Periodic { .. } => "periodic",
            Effect::Replacement { .. } => "replacement",
            Effect::Condition { .. } => "condition",
            Effect::Capability { .. } => "capability",
            Effect::Hint { .. } => "hint",
        }
    }

    /// All effect kinds.
    pub const KINDS: [&'static str; 8] = [
        "modifier",
        "restriction",
        "trigger",
        "periodic",
        "replacement",
        "condition",
        "capability",
        "hint",
    ];

    /// The operations this effect runs, if any.
    pub fn ops(&self) -> &[Op] {
        match self {
            Effect::Trigger { then, .. } | Effect::Periodic { then, .. } => then,
            _ => &[],
        }
    }

    /// Fields every effect kind has: target, own duration, visibility.
    pub fn common(&self) -> (Target, Option<Lasts>, bool) {
        match self {
            Effect::Modifier {
                target,
                lasts,
                gm_only,
                ..
            }
            | Effect::Restriction {
                target,
                lasts,
                gm_only,
                ..
            }
            | Effect::Trigger {
                target,
                lasts,
                gm_only,
                ..
            }
            | Effect::Periodic {
                target,
                lasts,
                gm_only,
                ..
            }
            | Effect::Replacement {
                target,
                lasts,
                gm_only,
                ..
            }
            | Effect::Condition {
                target,
                lasts,
                gm_only,
                ..
            }
            | Effect::Capability {
                target,
                lasts,
                gm_only,
                ..
            }
            | Effect::Hint {
                target,
                lasts,
                gm_only,
                ..
            } => (*target, *lasts, *gm_only),
        }
    }
}

/// Who a one-shot operation affects. Groups that depend on where people
/// are (range, hearing, seeing) are proposed by the app and confirmed by
/// the game master, because the app does not track positions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Who {
    #[default]
    Bearer,
    FriendlyPcsShortRange,
    FriendlyWhoHear,
    FriendlyWhoSee,
    FriendlyWhoWitness,
    Aboard,
}

/// A roll made by an operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RollSpec {
    /// A skill roll: attribute + skill (+ Stress Dice).
    Skill(Id),
    /// An attribute roll without skill.
    Attribute(Id),
}

/// Which action is lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActionSlot {
    Slow,
}

/// Who chooses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Chooser {
    Gm,
}

/// Consumables a character keeps themselves; air and power are in the
/// suit and the device (F-75) [p. 34].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Consumable {
    Food,
    Water,
}

/// A one-shot operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Op {
    /// Changes the STRESS LEVEL.
    Stress {
        delta: i8,
        #[serde(default, skip_serializing_if = "is_default")]
        who: Who,
    },
    /// Sets the STRESS LEVEL to zero, e.g. Naproleve [p. 137].
    ClearStress,
    /// Changes the Food or Water Supply, e.g. a prefab meal [p. 137].
    Supply { supply: Consumable, delta: i8 },
    /// Deals damage that ignores armor.
    Damage {
        amount: u8,
        #[serde(default, skip_serializing_if = "is_default")]
        who: Who,
    },
    /// Makes a roll and branches on the result.
    Roll {
        roll: RollSpec,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        on_success: Vec<Op>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        on_fail: Vec<Op>,
    },
    /// Makes a Panic Roll.
    PanicRoll {
        #[serde(default, skip_serializing_if = "is_default")]
        who: Who,
    },
    /// The next action of this kind is lost.
    LoseNextAction { action: ActionSlot },
    /// Drops a weapon or other important item.
    DropItem { chosen_by: Chooser },
    /// Blocks something for a while.
    Block { blocks: Blocked, lasts: Lasts },
    /// Asks the player or game master a yes/no question.
    Ask {
        /// Translation key of the question.
        question: Id,
        on_no: Vec<Op>,
    },
    /// Shows a note.
    Hint {
        /// Translation key of the note.
        text: Id,
    },
    /// Calls a named function in the engine for a one-off rule.
    Special { handler: Id },
}

impl Op {
    /// Nested operations, if any.
    pub fn children(&self) -> Vec<&Op> {
        match self {
            Op::Roll {
                on_success,
                on_fail,
                ..
            } => on_success.iter().chain(on_fail.iter()).collect(),
            Op::Ask { on_no, .. } => on_no.iter().collect(),
            _ => Vec::new(),
        }
    }
}

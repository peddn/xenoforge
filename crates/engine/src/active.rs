//! Which effects work on an actor right now, and what they mean for a roll
//! or an action (plan section 4.4).
//!
//! Base values are never changed. Every question ("which modifications
//! apply to this roll?") is answered fresh from the actor's talents, active
//! sources and panic, so a value is right again as soon as its source ends.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::data::effects::{
    Blocked, Cold, Effect, Interval, Modified, ReplaceWith, Replaceable, RollKind, Target, Unit,
};
use crate::engine::Engine;
use crate::state::{Actor, GameState, Origin};

/// An effect working on an actor, with where it comes from.
#[derive(Debug, Clone)]
pub struct ActiveEffect<'a> {
    /// Source for explanations, e.g. `injury/broken_ribs`.
    pub label: String,
    /// Data entry the effect belongs to (owner of its text keys).
    pub owner: String,
    pub effect: Cow<'a, Effect>,
    /// Cold level, for effects of a condition such as Freezing.
    pub cold: Option<Cold>,
    /// Comes from a condition; its damage gives no extra stress (F-32).
    pub from_condition: bool,
}

/// One modification of a dice pool, with its source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Modification {
    pub source: String,
    pub value: i8,
}

/// What a roll is, for matching modifiers.
#[derive(Debug, Clone, Copy)]
pub struct RollMatch<'s> {
    pub skill: Option<&'s str>,
    pub attribute: Option<&'s str>,
    pub kind: Option<RollKind>,
}

/// Nested conditions deeper than this are ignored (guards against loops).
const MAX_DEPTH: u8 = 3;

impl Engine {
    /// All effects working on `actor` right now. Effects aimed at a ship
    /// or at everyone aboard are not part of this (phase 5).
    pub fn active_effects(&self, state: &GameState, actor: &Actor) -> Vec<ActiveEffect<'_>> {
        let mut out = Vec::new();
        for talent in &actor.talents {
            if let Some(entry) = self.talent(talent) {
                for effect in &entry.effects {
                    self.push_effect(
                        &mut out,
                        actor,
                        effect,
                        &format!("talent/{talent}"),
                        talent,
                        None,
                        false,
                        0,
                    );
                }
            }
        }
        for source in &actor.sources {
            let (cold, from_condition) = match &source.origin {
                Origin::Condition { cold, .. } => (*cold, true),
                Origin::Part { of, .. } => match of.as_ref() {
                    Origin::Condition { cold, .. } => (*cold, true),
                    _ => (None, false),
                },
                _ => (None, false),
            };
            let label = source.origin.label();
            let owner = source.origin.entry().to_owned();
            for effect in self.source_effects(state, &source.origin) {
                self.push_cow(
                    &mut out,
                    actor,
                    effect,
                    &label,
                    &owner,
                    cold,
                    from_condition,
                    0,
                );
            }
        }
        if let Some(panic) = &actor.panic {
            if panic.effects_end.is_some() {
                if let Some(entry) = self.panic_entry(&panic.effect) {
                    for effect in &entry.effects {
                        let label = format!("panic/{}", panic.effect);
                        self.push_effect(
                            &mut out,
                            actor,
                            effect,
                            &label,
                            &panic.effect,
                            None,
                            false,
                            0,
                        );
                    }
                }
            }
        }
        out
    }

    /// The effects an origin contributes. Effects with their own duration
    /// live in a separate `Part` and are left out of the whole.
    pub(crate) fn source_effects(
        &self,
        state: &GameState,
        origin: &Origin,
    ) -> Vec<Cow<'_, Effect>> {
        match origin {
            Origin::Part { of, index } => self
                .origin_effects(state, of)
                .get(*index)
                .map(|e| vec![Cow::Borrowed(*e)])
                .unwrap_or_default(),
            Origin::Block { blocks, .. } => vec![Cow::Owned(Effect::Restriction {
                target: Target::Bearer,
                blocks: blocks.clone(),
                lasts: None,
                gm_only: false,
            })],
            _ => self
                .origin_effects(state, origin)
                .into_iter()
                .filter(|e| e.common().1.is_none())
                .map(Cow::Borrowed)
                .collect(),
        }
    }

    /// All effects of the data entry behind an origin, with the ruling
    /// options of the team applied.
    pub(crate) fn origin_effects(&self, state: &GameState, origin: &Origin) -> Vec<&Effect> {
        match origin {
            Origin::Trauma(id) => match self.trauma(id) {
                Some(trauma) => self.with_ruling(state, &trauma.effects, trauma.ruling.as_ref()),
                None => Vec::new(),
            },
            Origin::Condition { id, .. } => self
                .condition(id)
                .map(|c| c.effects.iter().collect())
                .unwrap_or_default(),
            Origin::Injury(id) => self
                .injury(id)
                .map(|i| i.effects.iter().collect())
                .unwrap_or_default(),
            Origin::Block { .. } | Origin::Part { .. } => Vec::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn push_effect<'a>(
        &'a self,
        out: &mut Vec<ActiveEffect<'a>>,
        actor: &Actor,
        effect: &'a Effect,
        label: &str,
        owner: &str,
        cold: Option<Cold>,
        from_condition: bool,
        depth: u8,
    ) {
        self.push_cow(
            out,
            actor,
            Cow::Borrowed(effect),
            label,
            owner,
            cold,
            from_condition,
            depth,
        );
    }

    /// Adds an effect; a `condition` effect also brings the condition's
    /// own effects, unless the actor wears gear that protects from it.
    #[allow(clippy::too_many_arguments)]
    fn push_cow<'a>(
        &'a self,
        out: &mut Vec<ActiveEffect<'a>>,
        actor: &Actor,
        effect: Cow<'a, Effect>,
        label: &str,
        owner: &str,
        cold: Option<Cold>,
        from_condition: bool,
        depth: u8,
    ) {
        if effect.common().0 != Target::Bearer {
            return;
        }
        if let Effect::Condition {
            condition,
            cold: inner_cold,
            unless_wearing,
            ..
        } = effect.as_ref()
        {
            let protected = unless_wearing.iter().any(|g| actor.wearing.contains(g));
            if !protected && depth < MAX_DEPTH {
                if let Some(entry) = self.condition(condition) {
                    let label = format!("{label} → condition/{condition}");
                    for inner in &entry.effects {
                        self.push_effect(
                            out,
                            actor,
                            inner,
                            &label,
                            condition,
                            inner_cold.or(cold),
                            true,
                            depth + 1,
                        );
                    }
                }
            }
        }
        out.push(ActiveEffect {
            label: label.to_owned(),
            owner: owner.to_owned(),
            effect,
            cold,
            from_condition,
        });
    }
}

/// Dice modifications for a roll [p. 62]. `based_on` and `skills` only
/// match rolls with a skill; `rolls` match the other roll kinds.
pub fn roll_modifiers(effects: &[ActiveEffect], roll: RollMatch) -> Vec<Modification> {
    effects
        .iter()
        .filter_map(|active| match active.effect.as_ref() {
            Effect::Modifier {
                applies_to,
                modifies: Modified::Dice,
                change: crate::data::effects::Change::Add(value),
                ..
            } if matches_roll(applies_to, roll) => Some(Modification {
                source: active.label.clone(),
                value: *value,
            }),
            _ => None,
        })
        .collect()
}

/// The highest `at_least` of the dice modifiers that apply to a roll.
pub fn floor(effects: &[ActiveEffect], roll: RollMatch) -> Option<u8> {
    effects
        .iter()
        .filter_map(|active| match active.effect.as_ref() {
            Effect::Modifier {
                applies_to,
                modifies: Modified::Dice,
                at_least,
                ..
            } if matches_roll(applies_to, roll) => *at_least,
            _ => None,
        })
        .max()
}

/// Extra pushes for a skill roll, e.g. Past the Limit [p. 74]. The
/// attribute is the one actually rolled (F-35).
pub fn extra_pushes(effects: &[ActiveEffect], roll: RollMatch) -> u8 {
    effects
        .iter()
        .filter_map(|active| match active.effect.as_ref() {
            Effect::Modifier {
                applies_to,
                modifies: Modified::Pushes,
                change: crate::data::effects::Change::Add(value),
                ..
            } if matches_roll(applies_to, roll) => u8::try_from(*value).ok(),
            _ => None,
        })
        .sum()
}

fn matches_roll(target: &crate::data::effects::ModTarget, roll: RollMatch) -> bool {
    let skill_roll = roll.skill.is_some();
    (skill_roll
        && roll
            .skill
            .is_some_and(|s| target.skills.iter().any(|t| t == s)))
        || (skill_roll
            && roll
                .attribute
                .is_some_and(|a| target.based_on.iter().any(|t| t == a)))
        || roll.kind.is_some_and(|k| target.rolls.contains(&k))
}

/// The first effect that blocks something, as its source label.
pub fn blocking(effects: &[ActiveEffect], blocked: &Blocked) -> Option<String> {
    effects
        .iter()
        .find_map(|active| match active.effect.as_ref() {
            Effect::Restriction { blocks, .. } if blocks == blocked => Some(active.label.clone()),
            _ => None,
        })
}

/// A replacement the actor can use, with its talent or source.
pub struct Usable<'e> {
    pub owner: String,
    pub replaces: &'e Replaceable,
    pub with: &'e ReplaceWith,
}

/// All replacement effects of the actor.
pub fn replacements<'e>(effects: &'e [ActiveEffect]) -> Vec<Usable<'e>> {
    effects
        .iter()
        .filter_map(|active| match active.effect.as_ref() {
            Effect::Replacement { replaces, with, .. } => Some(Usable {
                owner: active.owner.clone(),
                replaces,
                with,
            }),
            _ => None,
        })
        .collect()
}

/// Does a periodic interval fire on this step? `ByCold` uses the cold
/// level of the condition the effect belongs to.
pub fn fires(every: &Interval, cold: Option<Cold>, unit: Unit, new_day: bool) -> bool {
    let wanted = match every {
        Interval::Step(unit) => Some(*unit),
        Interval::ByCold(intervals) => cold.map(|c| match c {
            Cold::AboveFreezing => intervals.above_freezing,
            Cold::SubZero => intervals.sub_zero,
            Cold::DeepSpace => intervals.deep_space,
        }),
    };
    match wanted {
        Some(Unit::Day) => new_day,
        Some(wanted) => wanted == unit,
        None => false,
    }
}

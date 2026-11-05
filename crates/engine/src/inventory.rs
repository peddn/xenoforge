//! What characters carry (plan 5.4): items from the gear catalog or free
//! ones, where they are, encumbrance and the bonuses of gear in a roll
//! [p. 33–34, 62, 98, 118].
//!
//! Carried and worn items count for encumbrance; stowed ones do not and
//! cannot be used in a roll. A worn suit works by itself: its modifiers
//! count and it may protect in a vacuum (F-18). Other gear is offered in
//! the roll dialog; each chosen item counts once, a weapon only for the
//! skill it is used with, and only one weapon per roll (F-59).

use serde::{Deserialize, Serialize};

use crate::active::{Modification, RollMatch, matches_roll};
use crate::data::Id;
use crate::data::effects::{ActorStat, Change, Effect, Gear, Modified};
use crate::data::model::{Item, Weight};
use crate::engine::{Ctx, Engine, Event, Result, RuleError};
use crate::state::{Actor, ActorId, Place, Possession, Supply};

/// Encumbrance in quarters of a regular item [p. 34].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Encumbrance {
    /// Carried and worn items, food and water.
    pub carried: u32,
    /// Without being encumbered: STRENGTH × 2, more with Pack Mule.
    pub normal: u32,
    /// Overloaded at most: STRENGTH × 4 (F-62).
    pub max: u32,
}

/// An item the person can choose for a roll, with what it adds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemOffer {
    pub uid: u32,
    pub item: Option<Id>,
    pub value: i8,
    /// The situation the bonus needs, as a text key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<Id>,
    /// Used as the weapon of the roll.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub weapon: bool,
}

/// A new item for a character.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct NewItem {
    /// From `gear.json`; or a free item with `name` and `weight`.
    pub item: Option<Id>,
    pub name: Option<String>,
    pub weight: Option<Weight>,
    /// One if left out.
    pub count: Option<u16>,
    pub place: Place,
    pub reloads: u8,
}

/// Where an item goes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Destination {
    Carried,
    /// Put on; a suit only, and only one at a time [p. 98].
    Worn,
    Stowed,
    /// Gone: dropped, lost or used up.
    Dropped,
    /// Handed to another character, who carries it.
    Character(ActorId),
}

impl Engine {
    pub fn item(&self, id: &str) -> Option<&Item> {
        self.rules.gear.iter().find(|e| e.id == id)
    }

    /// Weight of one of them: from the catalog or the free item.
    fn weight_of(&self, possession: &Possession) -> Option<Weight> {
        match &possession.item {
            Some(id) => self.item(id).and_then(|i| i.weight),
            None => possession.weight,
        }
    }

    pub fn encumbrance(&self, actor: &Actor) -> Encumbrance {
        let mut carried: u32 = actor
            .items
            .iter()
            .filter(|p| p.place != Place::Stowed)
            .map(|p| {
                self.weight_of(p)
                    .map_or(0, |w| w.quarters().saturating_mul(u32::from(p.count)))
            })
            .sum();
        // Food and water: one item per started four of the rating (F-69)
        // [p. 34].
        let per_item = u32::from(self.constant_u8("supply_per_item")).max(1);
        for supply in [Supply::Food, Supply::Water] {
            let rating = u32::from(actor.supplies.get(&supply).copied().unwrap_or(0));
            carried = carried.saturating_add(rating.div_ceil(per_item) * Weight::One.quarters());
        }
        let strength = u32::from(actor.attribute("strength")) * Weight::One.quarters();
        let max = strength * u32::from(self.constant_u8("carry_max_factor"));
        let factor: u32 = actor
            .talents
            .iter()
            .filter_map(|t| self.talent(t))
            .flat_map(|t| &t.effects)
            .filter_map(|e| match e {
                Effect::Modifier {
                    applies_to,
                    change: Change::Multiply(by),
                    ..
                } if applies_to.stats.contains(&ActorStat::CarryCapacity) => Some(u32::from(*by)),
                _ => None,
            })
            .product();
        let normal = (strength * u32::from(self.constant_u8("carry_factor")) * factor).min(max);
        Encumbrance {
            carried,
            normal,
            max,
        }
    }

    /// Wears a suit that protects against `gear`, e.g. in a vacuum (F-18).
    pub(crate) fn wears(&self, actor: &Actor, gear: Gear) -> bool {
        actor.items.iter().any(|p| {
            p.place == Place::Worn
                && p.item
                    .as_deref()
                    .and_then(|id| self.item(id))
                    .and_then(|i| i.armor)
                    .is_some_and(|armor| match gear {
                        Gear::VacuumSuit => armor.vacuum,
                    })
        })
    }

    /// Effects of the worn suit, which work by themselves.
    pub(crate) fn worn_effects<'a>(&'a self, actor: &Actor) -> Vec<(&'a Item, &'a Effect)> {
        actor
            .items
            .iter()
            .filter(|p| p.place == Place::Worn)
            .filter_map(|p| p.item.as_deref().and_then(|id| self.item(id)))
            .flat_map(|item| item.effects.iter().map(move |e| (item, e)))
            .collect()
    }

    /// What one carried item adds to a roll: its modifiers that match,
    /// and a weapon's bonus for its own skill.
    fn item_bonus(&self, possession: &Possession, roll: RollMatch) -> Option<ItemOffer> {
        if possession.place != Place::Carried {
            return None;
        }
        let item = self.item(possession.item.as_deref()?)?;
        // A suit helps only while worn, by itself.
        if item.armor.is_some() {
            return None;
        }
        let mut value: i8 = 0;
        let mut when = None;
        let mut fits = false;
        for effect in &item.effects {
            if let Effect::Modifier {
                applies_to,
                modifies: Modified::Dice,
                change: Change::Add(add),
                when: situation,
                ..
            } = effect
            {
                if matches_roll(applies_to, roll) {
                    value = value.saturating_add(*add);
                    when = when.or_else(|| situation.clone());
                    fits = true;
                }
            }
        }
        let weapon = item
            .weapon
            .as_ref()
            .is_some_and(|w| roll.skill == Some(w.skill.as_str()));
        if weapon {
            value = value.saturating_add(item.weapon.as_ref().map_or(0, |w| w.bonus));
            fits = true;
        }
        fits.then(|| ItemOffer {
            uid: possession.uid,
            item: possession.item.clone(),
            value,
            when,
            weapon,
        })
    }

    /// Carried items that fit a roll, for the roll dialog (F-59).
    /// The same kind of item is offered once (F-71).
    pub fn item_offers(&self, actor: &Actor, roll: RollMatch) -> Vec<ItemOffer> {
        let mut out: Vec<ItemOffer> = Vec::new();
        for offer in actor.items.iter().filter_map(|p| self.item_bonus(p, roll)) {
            if !out.iter().any(|o| o.item == offer.item) {
                out.push(offer);
            }
        }
        out
    }

    /// Modifications of the items chosen for a roll: each once, carried,
    /// fitting the roll, at most one weapon (F-59); two of the same kind
    /// count once (F-71).
    pub(crate) fn item_modifications(
        &self,
        actor: &Actor,
        uids: &[u32],
        roll: RollMatch,
    ) -> Result<Vec<Modification>> {
        let mut out = Vec::new();
        let mut kinds: Vec<Id> = Vec::new();
        let mut weapons = 0;
        for (i, uid) in uids.iter().enumerate() {
            if uids[..i].contains(uid) {
                return Err(RuleError::NotAllowed(format!("item {uid} is chosen twice")));
            }
            let possession = actor
                .items
                .iter()
                .find(|p| p.uid == *uid)
                .ok_or_else(|| RuleError::NotAllowed(format!("no item {uid}")))?;
            let offer = self.item_bonus(possession, roll).ok_or_else(|| {
                RuleError::NotAllowed(format!("item {uid} does not help with this roll"))
            })?;
            if let Some(kind) = &offer.item {
                if kinds.contains(kind) {
                    return Err(RuleError::NotAllowed(format!(
                        "\"{kind}\" counts once per roll (F-71)"
                    )));
                }
                kinds.push(kind.clone());
            }
            if offer.weapon {
                weapons += 1;
            }
            // A weapon shows even without a bonus, so the log names it.
            if offer.value != 0 || offer.weapon {
                out.push(Modification {
                    source: format!("item/{}", offer.item.unwrap_or_default()),
                    value: offer.value,
                });
            }
        }
        if weapons > 1 {
            return Err(RuleError::NotAllowed("one weapon per roll (F-59)".into()));
        }
        Ok(out)
    }

    /// The chosen items that are used up by the roll (F-68).
    pub(crate) fn single_use(&self, actor: &Actor, uids: &[u32]) -> Vec<u32> {
        actor
            .items
            .iter()
            .filter(|p| uids.contains(&p.uid))
            .filter(|p| {
                p.item
                    .as_deref()
                    .and_then(|id| self.item(id))
                    .is_some_and(|i| i.single_use)
            })
            .map(|p| p.uid)
            .collect()
    }

    /// A possession for a new item: checked, with a full supply.
    pub(crate) fn possession(&self, new: &NewItem, uid: u32) -> Result<Possession> {
        let count = new.count.unwrap_or(1);
        if count == 0 {
            return Err(RuleError::NotAllowed("count must be at least 1".into()));
        }
        let (item, supply) = match &new.item {
            Some(id) => {
                let item = self
                    .item(id)
                    .ok_or_else(|| RuleError::UnknownEntry(id.clone()))?;
                if item.weight.is_none() {
                    return Err(RuleError::NotAllowed(format!(
                        "\"{id}\" cannot be carried (F-63)"
                    )));
                }
                (Some(item), item.supply.map(|s| s.max))
            }
            None => {
                let named = new.name.as_deref().is_some_and(|n| !n.trim().is_empty());
                if !named || new.weight.is_none() {
                    return Err(RuleError::NotAllowed(
                        "a free item needs a name and a weight".into(),
                    ));
                }
                (None, None)
            }
        };
        if new.place == Place::Worn && item.and_then(|i| i.armor).is_none() {
            return Err(RuleError::NotAllowed("only suits are worn".into()));
        }
        if new.place == Place::Worn && count > 1 {
            return Err(RuleError::NotAllowed("one suit at a time [p. 98]".into()));
        }
        Ok(Possession {
            uid,
            item: new.item.clone(),
            name: if new.item.is_some() {
                None
            } else {
                new.name.as_deref().map(|n| n.trim().to_owned())
            },
            weight: if new.item.is_some() { None } else { new.weight },
            count,
            place: new.place,
            reloads: new.reloads,
            supply,
        })
    }

    /// Items set by hand, e.g. in a correction by the game master: they
    /// must be what `AddItem` and `MoveItem` could have made.
    pub fn check_items(&self, actor: &Actor) -> Result<()> {
        let mut worn = 0;
        for (i, p) in actor.items.iter().enumerate() {
            if p.uid >= actor.next_item || actor.items[..i].iter().any(|o| o.uid == p.uid) {
                return Err(RuleError::NotAllowed(format!(
                    "item {} needs its own number below {}",
                    p.uid, actor.next_item
                )));
            }
            let new = NewItem {
                item: p.item.clone(),
                name: p.name.clone(),
                weight: p.weight,
                count: Some(p.count),
                place: p.place,
                reloads: p.reloads,
            };
            let full = self.possession(&new, p.uid)?.supply;
            let fits = match (p.supply, full) {
                (None, _) => true,
                (Some(left), Some(max)) => left <= max,
                (Some(_), None) => false,
            };
            if !fits {
                return Err(RuleError::NotAllowed(format!(
                    "item {} holds no more than its full supply",
                    p.uid
                )));
            }
            if p.place == Place::Worn {
                worn += 1;
            }
        }
        if worn > 1 {
            return Err(RuleError::NotAllowed("one suit at a time [p. 98]".into()));
        }
        self.check_load(actor, "the character")
    }

    /// Too much to carry, even overloaded [p. 34].
    fn check_load(&self, actor: &Actor, who: &str) -> Result<()> {
        let load = self.encumbrance(actor);
        if load.carried > load.max {
            return Err(RuleError::NotAllowed(format!(
                "\"{who}\" cannot carry that much (p. 34)"
            )));
        }
        Ok(())
    }
}

/// Gives the actor a new item; it gets the next number.
pub(crate) fn give(engine: &Engine, actor: &mut Actor, new: &NewItem) -> Result<u32> {
    let uid = actor.next_item;
    let possession = engine.possession(new, uid)?;
    if possession.place == Place::Worn {
        take_off(actor);
    }
    actor.items.push(possession);
    actor.next_item = uid + 1;
    Ok(uid)
}

/// One suit at a time: the worn one is carried again [p. 98].
fn take_off(actor: &mut Actor) {
    for p in &mut actor.items {
        if p.place == Place::Worn {
            p.place = Place::Carried;
        }
    }
}

impl Ctx<'_> {
    /// One of the item is gone: a single-use item after its roll (F-68).
    pub(crate) fn use_up(&mut self, actor: &str, uid: u32) -> Result<()> {
        let a = self.actor_mut(actor)?;
        let Some(index) = a.items.iter().position(|p| p.uid == uid) else {
            return Ok(());
        };
        let item = a.items[index].item.clone();
        if a.items[index].count > 1 {
            a.items[index].count -= 1;
        } else {
            a.items.remove(index);
        }
        self.emit(Event::ItemUsedUp {
            actor: actor.to_owned(),
            uid,
            item,
        });
        Ok(())
    }

    pub(crate) fn add_item(&mut self, actor: &str, new: &NewItem) -> Result<()> {
        let engine = self.engine;
        let a = self.actor_mut(actor)?;
        let uid = give(engine, a, new)?;
        let snapshot = a.clone();
        engine.check_load(&snapshot, actor)?;
        let possession = snapshot.items.iter().find(|p| p.uid == uid).cloned();
        if let Some(p) = possession {
            self.emit(Event::ItemAdded {
                actor: actor.to_owned(),
                uid,
                item: p.item,
                name: p.name,
                count: p.count,
                place: p.place,
            });
        }
        Ok(())
    }

    pub(crate) fn move_item(
        &mut self,
        actor: &str,
        uid: u32,
        to: Destination,
        count: Option<u16>,
    ) -> Result<()> {
        let engine = self.engine;
        // Moving never makes too much worse; dropping or stowing always
        // helps (p. 34).
        let before = engine.encumbrance(self.actor(actor)?).carried;
        let a = self.actor_mut(actor)?;
        let index = a
            .items
            .iter()
            .position(|p| p.uid == uid)
            .ok_or_else(|| RuleError::NotAllowed(format!("no item {uid}")))?;
        let have = a.items[index].count;
        let count = count.unwrap_or(have);
        if count == 0 || count > have {
            return Err(RuleError::NotAllowed(format!(
                "there are {have} of item {uid}"
            )));
        }
        // Part of a stack goes elsewhere on the character: it becomes an
        // item of its own. Spare reloads belong to the whole stack and stay
        // with what is left (F-61).
        let split = count < have
            && matches!(
                to,
                Destination::Carried | Destination::Stowed | Destination::Worn
            );
        let (item, name) = (a.items[index].item.clone(), a.items[index].name.clone());
        let index = if split {
            let mut part = a.items[index].clone();
            a.items[index].count -= count;
            part.uid = a.next_item;
            part.count = count;
            part.reloads = 0;
            a.next_item += 1;
            a.items.push(part);
            a.items.len() - 1
        } else {
            index
        };
        match &to {
            Destination::Carried => a.items[index].place = Place::Carried,
            Destination::Stowed => a.items[index].place = Place::Stowed,
            Destination::Worn => {
                let suit = a.items[index]
                    .item
                    .as_deref()
                    .and_then(|id| engine.item(id))
                    .and_then(|i| i.armor)
                    .is_some();
                if !suit {
                    return Err(RuleError::NotAllowed("only suits are worn".into()));
                }
                if a.items[index].count > 1 {
                    return Err(RuleError::NotAllowed("one suit at a time [p. 98]".into()));
                }
                take_off(a);
                a.items[index].place = Place::Worn;
            }
            Destination::Dropped | Destination::Character(_) => {
                let mut part = a.items[index].clone();
                if count == have {
                    a.items.remove(index);
                } else {
                    a.items[index].count -= count;
                    part.reloads = 0;
                }
                if let Destination::Character(other) = &to {
                    if other == actor {
                        return Err(RuleError::NotAllowed("give it to someone else".into()));
                    }
                    let b = self.actor_mut(other)?;
                    part.uid = b.next_item;
                    part.count = count;
                    part.place = Place::Carried;
                    b.next_item += 1;
                    b.items.push(part);
                    let snapshot = b.clone();
                    engine.check_load(&snapshot, other)?;
                }
            }
        }
        let after = engine.encumbrance(self.actor(actor)?);
        if after.carried > after.max && after.carried > before {
            return Err(RuleError::NotAllowed(format!(
                "\"{actor}\" cannot carry that much (p. 34)"
            )));
        }
        self.emit(Event::ItemMoved {
            actor: actor.to_owned(),
            uid,
            item,
            name,
            to,
            count,
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::RuleData;

    /// Only a worn suit that is airtight protects in a vacuum (F-18).
    #[test]
    fn a_worn_vacuum_suit_protects() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/rules");
        let engine = Engine::new(RuleData::load(&dir).unwrap());
        let mut actor = Actor::pc();
        let worn = NewItem {
            item: Some("m3_personnel_armor".into()),
            place: Place::Worn,
            ..NewItem::default()
        };
        give(&engine, &mut actor, &worn).unwrap();
        assert!(
            !engine.wears(&actor, Gear::VacuumSuit),
            "M3 is not airtight"
        );
        let carried = NewItem {
            item: Some("irc_mk35_pressure_suit".into()),
            ..NewItem::default()
        };
        give(&engine, &mut actor, &carried).unwrap();
        assert!(!engine.wears(&actor, Gear::VacuumSuit), "carried only");
        actor.items[0].place = Place::Carried;
        actor.items[1].place = Place::Worn;
        assert!(engine.wears(&actor, Gear::VacuumSuit));
    }
}

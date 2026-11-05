//! Money (plan 5.4): a list of bookings per character, the balance is
//! their sum, in cents. The starting cash is the first booking [p. 33]; the
//! game master books wages, rewards and costs and sells gear. Buying needs
//! enough money; a loan is a booking of its own (F-76) [p. 124, 161].

use serde::{Deserialize, Serialize};

use crate::creation::Cash;
use crate::data::Id;
use crate::engine::{Ctx, Engine, Event, Result, RuleError};
use crate::inventory::NewItem;
use crate::state::Actor;

/// One booking; positive is money in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Booking {
    pub amount: i64,
    pub reason: Reason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Reason {
    /// The career's starting cash, rolled when finishing [p. 39–55].
    Start,
    /// Wages, a reward, an expense, a loan, in the game master's words.
    Note { text: String },
    /// Gear bought [p. 118].
    Purchase {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        item: Option<Id>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        count: u16,
    },
    /// Spare reloads for a firearm [p. 124].
    Reloads { item: Id, count: u8 },
}

/// The balance: the sum of all bookings, in cents.
pub fn balance(actor: &Actor) -> i64 {
    actor.ledger.iter().map(|b| b.amount).sum()
}

/// Opens the account with the starting cash, in dollars [p. 39–55].
pub fn open_account(actor: &mut Actor, cash: &Cash) {
    actor.ledger.push(Booking {
        amount: i64::from(cash.amount) * 100,
        reason: Reason::Start,
    });
}

impl Engine {
    /// The usual price of an item, the lower end of a range; none where
    /// the book gives none [p. 118].
    pub fn price(&self, item: &str) -> Option<u64> {
        self.item(item)?.cost.map(|c| c.cents)
    }

    /// A full reload costs about 5% of the weapon [p. 124].
    pub fn reload_price(&self, item: &str) -> Option<u64> {
        let percent = u64::from(self.constant_u8("reload_cost_percent"));
        self.price(item)
            .map(|cents| (cents * percent).div_ceil(100))
    }
}

impl Ctx<'_> {
    fn book(&mut self, actor: &str, amount: i64, reason: Reason) -> Result<()> {
        let a = self.actor_mut(actor)?;
        let balance = balance(a) + amount;
        if balance < 0 {
            return Err(RuleError::NotAllowed(format!(
                "\"{actor}\" has not enough money; a loan is a booking of its own (F-76)"
            )));
        }
        let booking = Booking { amount, reason };
        a.ledger.push(booking.clone());
        self.emit(Event::Booked {
            actor: actor.to_owned(),
            booking,
            balance,
        });
        Ok(())
    }

    /// A booking by the game master: wages, a reward, an expense.
    pub(crate) fn book_note(&mut self, actor: &str, amount: i64, text: &str) -> Result<()> {
        let text = text.trim();
        if amount == 0 || text.is_empty() {
            return Err(RuleError::NotAllowed(
                "a booking needs an amount and a reason".into(),
            ));
        }
        self.book(
            actor,
            amount,
            Reason::Note {
                text: text.to_owned(),
            },
        )
    }

    /// Buys an item: the game master's price or the catalog's for each
    /// one, then the item as if given (F-76) [p. 118].
    pub(crate) fn buy_item(
        &mut self,
        actor: &str,
        new: &NewItem,
        price: Option<u64>,
    ) -> Result<()> {
        let count = new.count.unwrap_or(1);
        let price = match (price, &new.item) {
            (Some(price), _) => price,
            (None, Some(id)) => self
                .engine
                .price(id)
                .map(|cents| cents * u64::from(count))
                .ok_or_else(|| {
                    RuleError::NotAllowed(format!("\"{id}\" has no price in the book; name one"))
                })?,
            (None, None) => {
                return Err(RuleError::NotAllowed("a free item needs a price".into()));
            }
        };
        let amount = i64::try_from(price)
            .map_err(|_| RuleError::NotAllowed("that price is too high".into()))?;
        self.book(
            actor,
            -amount,
            Reason::Purchase {
                item: new.item.clone(),
                name: new.name.as_deref().map(|n| n.trim().to_owned()),
                count,
            },
        )?;
        self.add_item(actor, new)
    }

    /// Buys spare reloads for a firearm that takes them (F-61) [p. 124].
    pub(crate) fn buy_reloads(
        &mut self,
        actor: &str,
        uid: u32,
        count: u8,
        price: Option<u64>,
    ) -> Result<()> {
        let engine = self.engine;
        let possession = self
            .actor(actor)?
            .items
            .iter()
            .find(|p| p.uid == uid)
            .cloned()
            .ok_or_else(|| RuleError::NotAllowed(format!("no item {uid}")))?;
        let id = possession.item.clone().unwrap_or_default();
        let takes = engine
            .item(&id)
            .and_then(|i| i.weapon.as_ref())
            .is_some_and(|w| w.reloads);
        if !takes || count == 0 {
            return Err(RuleError::NotAllowed(format!(
                "item {uid} takes no reloads (F-61)"
            )));
        }
        let price = match price {
            Some(price) => price,
            None => engine
                .reload_price(&id)
                .map(|cents| cents * u64::from(count))
                .ok_or_else(|| {
                    RuleError::NotAllowed(format!("\"{id}\" has no price in the book; name one"))
                })?,
        };
        let amount = i64::try_from(price)
            .map_err(|_| RuleError::NotAllowed("that price is too high".into()))?;
        self.book(
            actor,
            -amount,
            Reason::Reloads {
                item: id.clone(),
                count,
            },
        )?;
        if let Some(p) = self
            .actor_mut(actor)?
            .items
            .iter_mut()
            .find(|p| p.uid == uid)
        {
            p.reloads = p.reloads.saturating_add(count);
        }
        self.emit(Event::ReloadsAdded {
            actor: actor.to_owned(),
            uid,
            item: id,
            count,
        });
        Ok(())
    }
}

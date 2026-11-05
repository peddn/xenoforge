//! Money: bookings, buying gear and reloads; enough money needed (F-76)
//! [p. 33, 118, 124, 161].

mod common;

use common::*;
use xenoforge_engine::creation::Cash;
use xenoforge_engine::inventory::NewItem;
use xenoforge_engine::money::{self, Reason};
use xenoforge_engine::state::Mode;
use xenoforge_engine::{Command, Event};

/// Nat with $400 starting cash.
fn table() -> Table {
    let mut nat = pc();
    money::open_account(
        &mut nat,
        &Cash {
            dice: vec![4],
            amount: 400,
        },
    );
    Table::new(Mode::Campaign).with("nat", nat)
}

fn balance(t: &Table) -> i64 {
    money::balance(t.actor("nat"))
}

fn catalog(item: &str) -> NewItem {
    NewItem {
        item: Some(item.into()),
        ..NewItem::default()
    }
}

/// The starting cash is the first booking [p. 33].
#[test]
fn starting_cash_opens_the_account() {
    let t = table();
    assert_eq!(balance(&t), 40_000);
    assert_eq!(t.actor("nat").ledger[0].reason, Reason::Start);
}

/// The game master books wages, rewards and costs; never below zero, a
/// loan is a booking of its own (F-76).
#[test]
fn bookings() {
    let mut t = table();
    let ev = t.run(
        Command::Book {
            actor: "nat".into(),
            amount: 64_000,
            note: " Lohn ".into(),
        },
        &[],
    );
    assert!(has(&ev, |e| matches!(
        e,
        Event::Booked {
            balance: 104_000,
            ..
        }
    )));
    assert_eq!(
        t.actor("nat").ledger[1].reason,
        Reason::Note {
            text: "Lohn".into()
        }
    );
    for (amount, note) in [
        (-104_001, "Bar"),
        (0, "Nichts"),
        (100, " "),
        (i64::MAX, "Zu viel"),
    ] {
        let refused = t.try_run(
            Command::Book {
                actor: "nat".into(),
                amount,
                note: note.into(),
            },
            &[],
        );
        assert!(refused.is_err(), "{amount} {note}");
    }
    assert_eq!(balance(&t), 104_000);
}

/// Buying: the catalog's price for each one, or the game master's; the
/// item as if given; not without enough money (F-76) [p. 118].
#[test]
fn buying_gear() {
    let mut t = table();
    let ev = t.run(
        Command::BuyItem {
            actor: "nat".into(),
            item: NewItem {
                count: Some(2),
                ..catalog("prefab_meal")
            },
            price: None,
        },
        &[],
    );
    assert!(has(&ev, |e| matches!(e, Event::ItemAdded { count: 2, .. })));
    assert_eq!(balance(&t), 40_000 - 2_000);
    assert_eq!(
        t.actor("nat").ledger[1].reason,
        Reason::Purchase {
            item: Some("prefab_meal".into()),
            name: None,
            count: 2,
            reloads: 0,
        }
    );
    // A range: the lower end, unless the game master names a price.
    t.run(
        Command::BuyItem {
            actor: "nat".into(),
            item: catalog("water_bottle"),
            price: Some(1_000),
        },
        &[],
    );
    assert_eq!(balance(&t), 37_000);
    for (item, price) in [
        (catalog("m41a_pulse_rifle_typo"), None),
        (catalog("armat_m41a_pulse_rifle"), None),
        (catalog("x_drugs"), None),
        (
            NewItem {
                name: Some("Kiste".into()),
                weight: Some(xenoforge_engine::data::model::Weight::One),
                ..NewItem::default()
            },
            None,
        ),
        (catalog("autodoc"), Some(100)),
    ] {
        let refused = t.try_run(
            Command::BuyItem {
                actor: "nat".into(),
                item,
                price,
            },
            &[],
        );
        assert!(refused.is_err());
    }
    assert_eq!(balance(&t), 37_000, "nothing booked for refusals");
    assert_eq!(t.actor("nat").items.len(), 2);
}

/// A firearm with spare reloads: the reloads are paid for as well, at the
/// catalog's price; a price named by the game master is for all of it
/// [p. 118, 124].
#[test]
fn buying_a_firearm_with_reloads() {
    let mut t = table();
    t.run(
        Command::BuyItem {
            actor: "nat".into(),
            item: NewItem {
                reloads: 2,
                ..catalog("m4a3_service_pistol")
            },
            price: None,
        },
        &[],
    );
    assert_eq!(balance(&t), 40_000 - 20_000 - 2 * 1_000);
    assert_eq!(t.actor("nat").items[0].reloads, 2);
    assert_eq!(
        t.actor("nat").ledger[1].reason,
        Reason::Purchase {
            item: Some("m4a3_service_pistol".into()),
            name: None,
            count: 1,
            reloads: 2,
        }
    );
    t.run(
        Command::BuyItem {
            actor: "nat".into(),
            item: NewItem {
                reloads: 1,
                ..catalog("m4a3_service_pistol")
            },
            price: Some(15_000),
        },
        &[],
    );
    assert_eq!(balance(&t), 3_000);
}

/// Reloads: about 5% of the weapon each, for firearms that take them
/// (F-61) [p. 124].
#[test]
fn buying_reloads() {
    let mut t = table();
    t.run(
        Command::BuyItem {
            actor: "nat".into(),
            item: catalog("m4a3_service_pistol"),
            price: None,
        },
        &[],
    );
    assert_eq!(balance(&t), 20_000);
    let ev = t.run(
        Command::BuyReloads {
            actor: "nat".into(),
            uid: 0,
            count: 2,
            price: None,
        },
        &[],
    );
    assert!(has(&ev, |e| matches!(
        e,
        Event::ReloadsAdded { count: 2, .. }
    )));
    assert_eq!(balance(&t), 18_000, "$10 each");
    assert_eq!(t.actor("nat").items[0].reloads, 2);
    t.run(
        Command::AddItem {
            actor: "nat".into(),
            item: catalog("knife"),
        },
        &[],
    );
    let refused = t.try_run(
        Command::BuyReloads {
            actor: "nat".into(),
            uid: 1,
            count: 1,
            price: None,
        },
        &[],
    );
    assert!(refused.is_err(), "a knife takes no reloads");
    // No more than fit; nothing is paid for what would not.
    let mut t = table();
    t.run(
        Command::AddItem {
            actor: "nat".into(),
            item: NewItem {
                reloads: 254,
                ..catalog("m4a3_service_pistol")
            },
        },
        &[],
    );
    let refused = t.try_run(
        Command::BuyReloads {
            actor: "nat".into(),
            uid: 0,
            count: 2,
            price: None,
        },
        &[],
    );
    assert!(refused.is_err(), "too many reloads");
    assert_eq!(balance(&t), 40_000);
}

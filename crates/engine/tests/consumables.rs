//! Using items [p. 34, 134–139]: doses, meals and drinks with their
//! effects (F-72 to F-74), devices with power and suits with air (F-75).

mod common;

use common::*;
use xenoforge_engine::engine::Step;
use xenoforge_engine::inventory::NewItem;
use xenoforge_engine::roll::RollRequest;
use xenoforge_engine::state::{Mode, Origin, Place, Supply};
use xenoforge_engine::{Choice, Command, Event, Modification};

fn table() -> Table {
    Table::new(Mode::Campaign).with(
        "nat",
        pc().attr("agility", 3)
            .attr("wits", 3)
            .skill("heavy_machinery", 1)
            .stress(4)
            .supply(Supply::Food, 2),
    )
}

/// Gives Nat an item and answers its number.
fn give(t: &mut Table, item: &str, count: u16) -> u32 {
    let events = t.run(
        Command::AddItem {
            actor: "nat".into(),
            item: NewItem {
                item: Some(item.into()),
                count: Some(count),
                ..NewItem::default()
            },
        },
        &[],
    );
    events
        .iter()
        .find_map(|e| match e {
            Event::ItemAdded { uid, .. } => Some(*uid),
            _ => None,
        })
        .expect("item added")
}

fn use_item(t: &mut Table, uid: u32, faces: &[u8]) -> Vec<Event> {
    t.run(
        Command::UseItem {
            actor: "nat".into(),
            uid,
        },
        faces,
    )
}

fn count(t: &Table, uid: u32) -> u16 {
    t.actor("nat")
        .items
        .iter()
        .find(|p| p.uid == uid)
        .map_or(0, |p| p.count)
}

/// Modifications of a skill roll as the preview builds it.
fn mods(t: &Table, skill: &str) -> Vec<Modification> {
    t.engine
        .preview(&t.state, RollRequest::skill("nat", skill))
        .unwrap()
        .modifications
}

fn shift(t: &mut Table) {
    t.run(
        Command::Step {
            step: Step::Shift { new_day: false },
        },
        &[],
    );
}

/// A prefab meal: Food Supply +1, and the meal is gone [p. 137].
#[test]
fn eating() {
    let mut t = table();
    let meals = give(&mut t, "prefab_meal", 2);
    let ev = use_item(&mut t, meals, &[]);
    assert!(has(&ev, |e| matches!(
        e,
        Event::SupplyChanged {
            supply: Supply::Food,
            from: 2,
            to: 3,
            ..
        }
    )));
    assert_eq!(count(&t, meals), 1);
    // Bug Juice gives both, soda Water (F-72).
    let juice = give(&mut t, "bug_juice_protein_drink", 1);
    use_item(&mut t, juice, &[]);
    let soda = give(&mut t, "carbonated_beverage", 1);
    use_item(&mut t, soda, &[]);
    let supplies = &t.actor("nat").supplies;
    assert_eq!((supplies[&Supply::Food], supplies[&Supply::Water]), (4, 2));
}

/// Naproleve: STRESS LEVEL to zero; each dose beyond the first in a Shift
/// gives −1 to AGILITY-based rolls until the Shift ends (F-74) [p. 137].
#[test]
fn naproleve() {
    let mut t = table();
    let doses = give(&mut t, "naproleve", 4);
    use_item(&mut t, doses, &[]);
    assert_eq!(t.actor("nat").stress, 0);
    assert!(mods(&t, "mobility").is_empty(), "the first dose is fine");
    use_item(&mut t, doses, &[]);
    use_item(&mut t, doses, &[]);
    let penalty = Modification {
        source: "item/naproleve".into(),
        value: -1,
    };
    assert_eq!(mods(&t, "mobility"), [penalty.clone(), penalty]);
    assert!(mods(&t, "observation").is_empty(), "AGILITY only");
    shift(&mut t);
    assert!(mods(&t, "mobility").is_empty());
    use_item(&mut t, doses, &[]);
    assert!(mods(&t, "mobility").is_empty(), "a new Shift counts anew");
    assert_eq!(count(&t, doses), 0);
}

/// Beer and liquor: stress −1 and −1 to WITS-based rolls per drink until
/// the next Shift (F-74); the bottle stays (F-73) [p. 137, 139].
#[test]
fn drinking() {
    let mut t = table();
    let beer = give(&mut t, "beer", 1);
    let bottle = give(&mut t, "hard_liquor", 1);
    use_item(&mut t, beer, &[]);
    use_item(&mut t, bottle, &[]);
    use_item(&mut t, bottle, &[]);
    assert_eq!(t.actor("nat").stress, 1);
    assert_eq!(count(&t, beer), 0);
    assert_eq!(count(&t, bottle), 1, "drunk by the glass");
    let total: i8 = mods(&t, "observation").iter().map(|m| m.value).sum();
    assert_eq!(total, -3);
    assert!(
        t.actor("nat")
            .sources
            .iter()
            .all(|s| matches!(&s.origin, Origin::Used { index: 0, .. }))
    );
    shift(&mut t);
    assert!(mods(&t, "observation").is_empty());
}

/// Neversleep: stress +1 and no stress relief for a day; a note for the
/// game master [p. 137].
#[test]
fn neversleep() {
    let mut t = table();
    let pills = give(&mut t, "neversleep_pills", 1);
    let ev = use_item(&mut t, pills, &[]);
    assert_eq!(t.actor("nat").stress, 5);
    assert!(has(&ev, |e| matches!(
        e,
        Event::Hint { text, .. } if text == "neversleep_pills.use"
    )));
    assert!(
        t.try_run(
            Command::UseSignatureItem {
                actor: "nat".into()
            },
            &[]
        )
        .is_err(),
        "no stress relief"
    );
}

/// Only items at hand that do something by themselves are used.
#[test]
fn what_can_be_used() {
    let mut t = table();
    let knife = give(&mut t, "knife", 1);
    let meal = give(&mut t, "prefab_meal", 1);
    t.run(
        Command::MoveItem {
            actor: "nat".into(),
            uid: meal,
            to: xenoforge_engine::inventory::Destination::Stowed,
            count: None,
        },
        &[],
    );
    for uid in [knife, meal, 99] {
        let refused = t.try_run(
            Command::UseItem {
                actor: "nat".into(),
                uid,
            },
            &[],
        );
        assert!(refused.is_err());
    }
}

/// A device: each use costs a Power Supply roll; empty, it does nothing
/// until refilled (F-75) [p. 34, 134].
#[test]
fn devices_with_power() {
    let mut t = table();
    let tracker = give(&mut t, "m314_motion_tracker", 1);
    let ev = use_item(&mut t, tracker, &[1, 1, 1, 1, 1]);
    assert!(has(&ev, |e| matches!(
        e,
        Event::SupplyRolled {
            supply: Supply::Power,
            item: Some(_),
            from: 5,
            to: 0,
            ..
        }
    )));
    let refused = t.try_run(
        Command::UseItem {
            actor: "nat".into(),
            uid: tracker,
        },
        &[],
    );
    assert!(refused.is_err(), "the battery is empty");
    assert!(
        t.try_run(
            Command::RefillItem {
                actor: "nat".into(),
                uid: tracker,
                supply: Some(6),
            },
            &[],
        )
        .is_err(),
        "holds at most 5"
    );
    t.run(
        Command::RefillItem {
            actor: "nat".into(),
            uid: tracker,
            supply: None,
        },
        &[],
    );
    assert_eq!(t.actor("nat").items[0].supply, Some(5));
}

/// A cutting torch ticked for a roll is used: its Power Supply roll comes
/// after the roll; an empty torch is not offered (F-75) [p. 135].
#[test]
fn a_device_in_a_roll() {
    let mut t = table();
    let torch = give(&mut t, "cutting_torch", 1);
    let mut request = RollRequest::skill("nat", "heavy_machinery");
    request.items = vec![torch];
    // STRENGTH 4 + 1 + torch 2; then 4 Stress Dice.
    let ev = t.roll(request.clone(), &[6, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3]);
    assert!(
        !has(&ev, |e| matches!(e, Event::SupplyRolled { .. })),
        "the roll is not over yet"
    );
    let ev = t.decide(&ev, Choice::Accept, &[1, 1, 1, 1, 1]);
    assert!(has(&ev, |e| matches!(
        e,
        Event::SupplyRolled {
            supply: Supply::Power,
            to: 0,
            ..
        }
    )));
    let preview = t
        .engine
        .preview(&t.state, RollRequest::skill("nat", "heavy_machinery"))
        .unwrap();
    assert!(preview.items.is_empty());
    assert!(t.try_run(Command::Roll { request }, &[]).is_err());
}

/// Using a dose or drink needs it at hand: carried; a worn suit's air is
/// its own matter (F-75).
#[test]
fn stowed_is_not_at_hand() {
    let mut t = table();
    let beer = give(&mut t, "beer", 1);
    t.run(
        Command::MoveItem {
            actor: "nat".into(),
            uid: beer,
            to: xenoforge_engine::inventory::Destination::Stowed,
            count: None,
        },
        &[],
    );
    assert_eq!(t.actor("nat").items[0].place, Place::Stowed);
    assert!(
        t.try_run(
            Command::UseItem {
                actor: "nat".into(),
                uid: beer,
            },
            &[],
        )
        .is_err()
    );
}

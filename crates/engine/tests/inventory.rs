//! What characters carry [p. 33–34, 118–139]: encumbrance, Pack Mule
//! (F-62), wearing a suit, moving and handing over items, gear in a roll
//! (F-59) and single use (F-68).

mod common;

use common::*;
use xenoforge_engine::inventory::{Destination, Encumbrance, NewItem};
use xenoforge_engine::roll::RollRequest;
use xenoforge_engine::state::{Mode, Place, Supply};
use xenoforge_engine::{Command, Event, Modification, RuleError};

fn table() -> Table {
    Table::new(Mode::Campaign)
        .with("nat", pc().attr("agility", 3).skill("ranged_combat", 2))
        .with("ilse", pc())
}

fn catalog(item: &str) -> NewItem {
    NewItem {
        item: Some(item.into()),
        ..NewItem::default()
    }
}

/// Gives `actor` an item and answers its number.
fn give(t: &mut Table, actor: &str, item: NewItem) -> u32 {
    let events = t.run(
        Command::AddItem {
            actor: actor.into(),
            item,
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

fn move_item(t: &mut Table, uid: u32, to: Destination) -> Result<Vec<Event>, RuleError> {
    t.try_run(
        Command::MoveItem {
            actor: "nat".into(),
            uid,
            to,
            count: None,
        },
        &[],
    )
}

fn load(t: &Table, actor: &str) -> Encumbrance {
    t.engine.encumbrance(t.actor(actor))
}

fn item(source: &str, value: i8) -> Modification {
    Modification {
        source: source.into(),
        value,
    }
}

/// p. 34: STRENGTH 4 carries eight regular items, overloaded sixteen; in
/// quarters of an item. Stowed things do not count; a pulse rifle weighs
/// one, a pistol a half.
#[test]
fn book_p34_encumbrance() {
    let mut t = table();
    assert_eq!(
        load(&t, "nat"),
        Encumbrance {
            carried: 0,
            normal: 32,
            max: 64
        }
    );
    give(&mut t, "nat", catalog("armat_m41a_pulse_rifle"));
    give(&mut t, "nat", catalog("m4a3_service_pistol"));
    let suit = give(&mut t, "nat", catalog("irc_mk35_pressure_suit"));
    assert_eq!(load(&t, "nat").carried, 4 + 2 + 8);
    move_item(&mut t, suit, Destination::Worn).unwrap();
    assert_eq!(load(&t, "nat").carried, 14, "a worn suit counts too");
    move_item(&mut t, suit, Destination::Stowed).unwrap();
    assert_eq!(load(&t, "nat").carried, 6);
}

/// p. 34: food and water count as one item up to Supply 4, two up to 8,
/// and so on for every started four (F-69); nothing at 0.
#[test]
fn book_p34_food_and_water() {
    let t = Table::new(Mode::Campaign).with(
        "nat",
        pc().supply(Supply::Food, 5)
            .supply(Supply::Water, 4)
            .supply(Supply::Air, 6),
    );
    assert_eq!(load(&t, "nat").carried, 8 + 4, "air does not count");
    let t = Table::new(Mode::Campaign).with("nat", pc().supply(Supply::Food, 9));
    assert_eq!(load(&t, "nat").carried, 12);
    let t = Table::new(Mode::Campaign).with("nat", pc().supply(Supply::Food, 0));
    assert_eq!(load(&t, "nat").carried, 0);
}

/// Pack Mule doubles the normal load; the most stays STRENGTH × 4 (F-62)
/// [p. 79].
#[test]
fn pack_mule() {
    let t = Table::new(Mode::Campaign)
        .with("nat", pc().attr("strength", 3).talent("pack_mule"))
        .with("ilse", pc().attr("strength", 3));
    assert_eq!(load(&t, "ilse").normal, 24);
    assert_eq!(load(&t, "nat").normal, 48);
    assert_eq!(load(&t, "nat").max, 48);
}

/// More than STRENGTH × 4 cannot be carried [p. 34]; things without a
/// weight cannot be carried at all (F-63); a free item needs a name and a
/// weight.
#[test]
fn what_can_be_carried() {
    let mut t = Table::new(Mode::Campaign).with("nat", pc().attr("strength", 2));
    give(&mut t, "nat", catalog("m56a2_smart_gun"));
    give(&mut t, "nat", catalog("m56a2_smart_gun"));
    let refused = t.try_run(
        Command::AddItem {
            actor: "nat".into(),
            item: catalog("m56a2_smart_gun"),
        },
        &[],
    );
    assert!(matches!(refused, Err(RuleError::NotAllowed(_))), "9 > 8");
    assert_eq!(t.actor("nat").items.len(), 2, "nothing changed");

    for item in [
        catalog("autodoc"),
        NewItem {
            name: Some("Kiste".into()),
            ..NewItem::default()
        },
        NewItem {
            weight: Some(xenoforge_engine::data::model::Weight::Half),
            ..NewItem::default()
        },
        NewItem {
            count: Some(0),
            ..catalog("knife")
        },
        NewItem {
            place: Place::Worn,
            ..catalog("knife")
        },
    ] {
        let refused = t.try_run(
            Command::AddItem {
                actor: "nat".into(),
                item,
            },
            &[],
        );
        assert!(refused.is_err());
    }
    let uid = give(
        &mut t,
        "nat",
        NewItem {
            name: Some(" Notizbuch ".into()),
            weight: Some(xenoforge_engine::data::model::Weight::Quarter),
            ..NewItem::default()
        },
    );
    let notes = &t.actor("nat").items[2];
    assert_eq!(
        (notes.uid, notes.name.as_deref(), notes.item.as_deref()),
        (uid, Some("Notizbuch"), None)
    );
    assert_eq!(load(&t, "nat").carried, 25);
}

/// A suit is worn one at a time: putting on another one takes the first
/// off [p. 98]; other things are not worn. Supplies start full.
#[test]
fn wearing_a_suit() {
    let mut t = table();
    let mk35 = give(&mut t, "nat", catalog("irc_mk35_pressure_suit"));
    let mk50 = give(
        &mut t,
        "nat",
        NewItem {
            place: Place::Worn,
            ..catalog("irc_mk50_compression_suit")
        },
    );
    let knife = give(&mut t, "nat", catalog("knife"));
    assert_eq!(t.actor("nat").items[1].supply, Some(5));
    move_item(&mut t, mk35, Destination::Worn).unwrap();
    let places: Vec<_> = t.actor("nat").items.iter().map(|p| p.place).collect();
    assert_eq!(places, [Place::Worn, Place::Carried, Place::Carried]);
    assert!(move_item(&mut t, knife, Destination::Worn).is_err());
    assert_eq!(t.actor("nat").items[1].uid, mk50);
}

/// Dropping removes; handing over gives the other character a new number;
/// part of a stack can go.
#[test]
fn dropping_and_handing_over() {
    let mut t = table();
    let flares = give(
        &mut t,
        "nat",
        NewItem {
            count: Some(3),
            ..catalog("signal_flare")
        },
    );
    let knife = give(&mut t, "nat", catalog("knife"));
    give(&mut t, "ilse", catalog("knife"));
    let events = t.run(
        Command::MoveItem {
            actor: "nat".into(),
            uid: flares,
            to: Destination::Character("ilse".into()),
            count: Some(2),
        },
        &[],
    );
    assert!(has(&events, |e| matches!(
        e,
        Event::ItemMoved { count: 2, .. }
    )));
    assert_eq!(t.actor("nat").items[0].count, 1);
    let given = &t.actor("ilse").items[1];
    assert_eq!(
        (given.uid, given.count, given.item.as_deref()),
        (1, 2, Some("signal_flare"))
    );
    for (to, count) in [
        (Destination::Character("nat".into()), None),
        (Destination::Dropped, Some(2)),
        (Destination::Character("nobody".into()), None),
    ] {
        let refused = t.try_run(
            Command::MoveItem {
                actor: "nat".into(),
                uid: flares,
                to,
                count,
            },
            &[],
        );
        assert!(refused.is_err());
    }
    move_item(&mut t, knife, Destination::Dropped).unwrap();
    assert_eq!(t.actor("nat").items.len(), 1);
    assert!(move_item(&mut t, knife, Destination::Carried).is_err());
}

/// A worn Mk.50 gives −1 on AGILITY-based rolls by itself (F-66); carried
/// it does nothing and is not offered [p. 127].
#[test]
fn worn_suit_works_by_itself() {
    let mut t = table();
    let suit = give(&mut t, "nat", catalog("irc_mk50_compression_suit"));
    let request = RollRequest::skill("nat", "ranged_combat");
    let preview = t.engine.preview(&t.state, request.clone()).unwrap();
    assert!(preview.modifications.is_empty());
    assert!(preview.items.is_empty());
    move_item(&mut t, suit, Destination::Worn).unwrap();
    let preview = t.engine.preview(&t.state, request).unwrap();
    assert_eq!(
        preview.modifications,
        [item("item/irc_mk50_compression_suit", -1)]
    );
}

/// Chosen gear adds its bonus; a weapon only for its own skill, one weapon
/// per roll, each item once, carried only (F-59) [p. 62, 118].
#[test]
fn gear_in_a_roll() {
    let mut t = table();
    let pistol = give(&mut t, "nat", catalog("m4a3_service_pistol"));
    let rifle = give(&mut t, "nat", catalog("armat_m41a_pulse_rifle"));
    let binoculars = give(&mut t, "nat", catalog("binoculars"));
    let knife = give(&mut t, "nat", catalog("knife"));

    let preview = t
        .engine
        .preview(&t.state, RollRequest::skill("nat", "ranged_combat"))
        .unwrap();
    let offered: Vec<_> = preview.items.iter().map(|o| (o.uid, o.value)).collect();
    assert_eq!(offered, [(pistol, 2), (rifle, 1)]);
    let preview = t
        .engine
        .preview(&t.state, RollRequest::skill("nat", "observation"))
        .unwrap();
    assert_eq!(preview.items.len(), 1);
    assert_eq!(preview.items[0].when.as_deref(), Some("binoculars.when"));
    assert!(!preview.items[0].weapon);
    let preview = t
        .engine
        .preview(&t.state, RollRequest::skill("nat", "close_combat"))
        .unwrap();
    assert_eq!(
        (
            preview.items[0].uid,
            preview.items[0].value,
            preview.items[0].weapon
        ),
        (knife, 0, true),
        "a weapon without a bonus is offered too"
    );

    let mut request = RollRequest::skill("nat", "ranged_combat");
    request.items = vec![pistol];
    let events = t.roll(request, &[6, 6, 6, 6, 6, 6, 6]);
    assert_eq!(started(&events).0, 3 + 2 + 2);
    assert_eq!(started(&events).3, [item("item/m4a3_service_pistol", 2)]);

    for items in [
        vec![pistol, rifle],
        vec![pistol, pistol],
        vec![binoculars],
        vec![99],
    ] {
        let mut request = RollRequest::skill("nat", "ranged_combat");
        request.items = items;
        assert!(t.try_run(Command::Roll { request }, &[]).is_err());
    }
    let mut request = RollRequest::new(
        "nat",
        xenoforge_engine::roll::RollKind::Attribute {
            attribute: "agility".into(),
        },
    );
    request.items = vec![pistol];
    assert!(
        t.try_run(Command::Roll { request }, &[]).is_err(),
        "gear helps skill rolls only"
    );
    move_item(&mut t, pistol, Destination::Stowed).unwrap();
    let mut request = RollRequest::skill("nat", "ranged_combat");
    request.items = vec![pistol];
    assert!(
        t.try_run(Command::Roll { request }, &[]).is_err(),
        "stowed gear is not at hand"
    );
}

/// The Personal Medkit counts once and is gone (F-68) [p. 136].
#[test]
fn single_use_medkit() {
    let mut t = table();
    let kits = give(
        &mut t,
        "nat",
        NewItem {
            count: Some(2),
            ..catalog("personal_medkit")
        },
    );
    let mut request = RollRequest::skill("nat", "medical_aid");
    request.items = vec![kits];
    let preview = t.engine.preview(&t.state, request.clone()).unwrap();
    assert_eq!(preview.modifications, [item("item/personal_medkit", 2)]);
    assert_eq!(t.actor("nat").items[0].count, 2, "a preview uses nothing");
    let events = t.roll(request.clone(), &[6, 6]);
    assert!(has(&events, |e| matches!(
        e,
        Event::ItemUsedUp { uid, .. } if *uid == kits
    )));
    assert_eq!(t.actor("nat").items[0].count, 1);
    t.roll(request.clone(), &[6, 6]);
    assert!(t.actor("nat").items.is_empty());
    assert!(t.try_run(Command::Roll { request }, &[]).is_err());
}

/// Weapon Specialist: +2 with the chosen weapon, ticked like any situation
/// (A-10) [p. 79].
#[test]
fn weapon_specialist() {
    let mut t = Table::new(Mode::Campaign).with(
        "nat",
        pc().skill("close_combat", 1).talent("weapon_specialist"),
    );
    let mut request = RollRequest::skill("nat", "close_combat");
    let preview = t.engine.preview(&t.state, request.clone()).unwrap();
    assert!(preview.talents.contains(&"weapon_specialist".to_owned()));
    request.talents = vec!["weapon_specialist".into()];
    let events = t.roll(request, &[6, 6, 6, 6, 6, 6, 6]);
    assert_eq!(started(&events).0, 4 + 1 + 2);
}

/// Hidden Stash shows as a note for the game master to act on [p. 78].
#[test]
fn hidden_stash_is_a_note() {
    let t = Table::new(Mode::Campaign).with("nat", pc().talent("hidden_stash"));
    let hints = t.engine.hints(&t.state, "nat");
    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].1, "hidden_stash.hint");
}

/// Part of a stack put elsewhere on the character becomes an item of its
/// own.
#[test]
fn splitting_a_stack() {
    let mut t = table();
    let flares = give(
        &mut t,
        "nat",
        NewItem {
            count: Some(3),
            ..catalog("signal_flare")
        },
    );
    t.run(
        Command::MoveItem {
            actor: "nat".into(),
            uid: flares,
            to: Destination::Stowed,
            count: Some(1),
        },
        &[],
    );
    let items: Vec<_> = t
        .actor("nat")
        .items
        .iter()
        .map(|p| (p.uid, p.count, p.place))
        .collect();
    assert_eq!(items, [(0, 2, Place::Carried), (1, 1, Place::Stowed)]);
    assert_eq!(load(&t, "nat").carried, 2);
}

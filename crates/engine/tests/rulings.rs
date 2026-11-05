//! Rulings of phase 2 checked on their own: damage and stress (F-33),
//! Authority and gear (F-49), the weight of reloads (F-70), Story Points
//! to start with (F-53) [p. 33, 61, 70, 74, 98, 103].

mod common;

use common::*;
use xenoforge_engine::inventory::NewItem;
use xenoforge_engine::roll::RollRequest;
use xenoforge_engine::state::Mode;
use xenoforge_engine::{Command, Event};

fn catalog(item: &str) -> NewItem {
    NewItem {
        item: Some(item.into()),
        ..NewItem::default()
    }
}

/// Only damage that is suffered brings stress; none, none (F-33)
/// [p. 98, 103].
#[test]
fn only_suffered_damage_brings_stress() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("strength", 3));
    let ev = t.run(
        Command::Damage {
            actor: "a".into(),
            amount: 0,
        },
        &[],
    );
    assert!(!has(&ev, |e| matches!(e, Event::StressChanged { .. })));
    assert_eq!(t.actor("a").stress, 0);
    t.run(
        Command::Damage {
            actor: "a".into(),
            amount: 1,
        },
        &[],
    );
    assert_eq!((t.actor("a").health, t.actor("a").stress), (2, 1));
}

/// Authority rolls COMMAND with what helps COMMAND, not what helps
/// MANIPULATION, such as a tape recorder (F-49) [p. 70, 74, 131].
#[test]
fn authority_takes_the_gear_of_command() {
    let mut t = Table::new(Mode::Campaign).with(
        "marshal",
        pc().attr("empathy", 3)
            .skill("command", 2)
            .skill("manipulation", 1)
            .talent("authority"),
    );
    t.run(
        Command::AddItem {
            actor: "marshal".into(),
            item: catalog("seegson_c_series_tape_recorder"),
        },
        &[],
    );
    let mut request = RollRequest::skill("marshal", "manipulation").talent("authority");
    request.items = vec![0];
    let as_command = t.try_run(Command::Roll { request }, &[]);
    assert!(as_command.is_err(), "the tape recorder helps MANIPULATION");
    let mut request = RollRequest::skill("marshal", "manipulation");
    request.items = vec![0];
    let ev = t.roll(request, &[1, 2, 3, 4, 5]);
    assert_eq!(started(&ev).0, 5, "EMPATHY 3 + MANIPULATION 1 + 1");
}

/// Spare reloads are a number at the weapon and weigh nothing (F-70)
/// [p. 33, 124].
#[test]
fn reloads_weigh_nothing() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("strength", 3));
    t.run(
        Command::AddItem {
            actor: "a".into(),
            item: catalog("m4a3_service_pistol"),
        },
        &[],
    );
    let without = t.engine.encumbrance(t.actor("a")).carried;
    t.run(
        Command::AddItem {
            actor: "a".into(),
            item: NewItem {
                reloads: 2,
                ..catalog("m4a3_service_pistol")
            },
        },
        &[],
    );
    let pistols = t.engine.encumbrance(t.actor("a")).carried;
    assert_eq!(pistols, 2 * without, "two pistols, the reloads add nothing");
}

/// New players start with no Story Points and may bring some, at most
/// three (F-53) [p. 61, 383].
#[test]
fn story_points_to_start_with() {
    let mut t = Table::new(Mode::Cinematic).with("a", pc().player("p"));
    assert_eq!(t.state.story_points.get("p").copied().unwrap_or(0), 0);
    t.run(
        Command::SetStoryPoints {
            player: "p".into(),
            points: 1,
        },
        &[],
    );
    assert_eq!(t.state.story_points["p"], 1);
    let refused = t.try_run(
        Command::SetStoryPoints {
            player: "p".into(),
            points: 4,
        },
        &[],
    );
    assert!(refused.is_err());
}

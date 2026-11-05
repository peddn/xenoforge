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

/// In Cinematic play a player's skill roll waits for them wherever a
/// Story Point could be spent, whether they have one or not; the log reads
/// the same either way, so it gives nothing away (A-01, F-06) [p. 61].
#[test]
fn cinematic_rolls_wait_with_or_without_a_story_point() {
    let after_push = |mode, points| {
        let mut t = Table::new(mode).with("a", pc().attr("wits", 2).player("p"));
        t.state.story_points.insert("p".into(), points);
        let ev = t.roll(RollRequest::skill("a", "observation"), &[2, 3]);
        let ev = t.decide(&ev, xenoforge_engine::Choice::Push, &[2, 3, 4]);
        let offered = if has(&ev, |e| matches!(e, Event::RollFinished { .. })) {
            None
        } else {
            Some(offered(&t, &ev))
        };
        (options(&ev), offered)
    };
    use xenoforge_engine::Choice::{Accept, StoryPoint};
    assert_eq!(
        after_push(Mode::Cinematic, 1),
        (vec![Accept], Some(vec![StoryPoint, Accept]))
    );
    assert_eq!(
        after_push(Mode::Cinematic, 0),
        (vec![Accept], Some(vec![Accept]))
    );
    assert_eq!(after_push(Mode::Campaign, 0), (vec![], None));
}

/// A fatally injured character rolls for their life even with Health back
/// [p. 99]; neither Broken nor fatally injured, no Death Roll.
#[test]
fn death_rolls_for_the_fatally_injured() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("strength", 2).health(1));
    let refused = t.try_run(
        Command::Roll {
            request: RollRequest::new("a", xenoforge_engine::roll::RollKind::Death),
        },
        &[6, 6],
    );
    assert!(refused.is_err());
    t.run(
        Command::Apply {
            actor: "a".into(),
            source: xenoforge_engine::NewSource::Injury("ruptured_jugular".into()),
        },
        // Healing time, 2D6 days [p. 100].
        &[3, 3],
    );
    let ev = t.roll(
        RollRequest::new("a", xenoforge_engine::roll::RollKind::Death),
        // STRENGTH 2 − 1 for the injury.
        &[6],
    );
    assert!(has(&ev, |e| matches!(e, Event::RollFinished { .. })));
}

/// A fixed pool, such as armor, counts successes only: no stunts [p. 96].
#[test]
fn fixed_pools_have_no_stunts() {
    let mut t = Table::new(Mode::Campaign);
    let ev = t.roll(RollRequest::pool(3), &[6, 6, 6]);
    assert_eq!(finished(&ev).1, 3);
    assert_eq!(finished(&ev).2, 0);
}

/// A choice of targets comes also when the other player characters are
/// androids only; its absence would give away a covert one (A-14).
#[test]
fn targets_are_asked_for_with_only_androids_near() {
    let mut ash = pc().android();
    ash.covert = true;
    let mut t = Table::new(Mode::Campaign)
        .with("a", pc().attr("wits", 3).stress(6))
        .with("ash", ash);
    // 6 + Stress 6 = 12: Scream, which asks who hears it [p. 105].
    let ev = t.run(Command::PanicRoll { actor: "a".into() }, &[6]);
    assert!(has(&ev, |e| matches!(e, Event::ChooseTargets { .. })));
    assert!(candidates(&t, &ev).is_empty());
}

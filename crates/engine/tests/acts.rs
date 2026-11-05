//! Acts and Story Points in Cinematic play (A-13): a Story Point for
//! furthering the Personal Agenda at the end of an Act, at most three
//! [p. 31, 61, 216].

mod common;

use common::*;
use xenoforge_engine::state::Mode;
use xenoforge_engine::{Command, Event, RuleError};

fn end_act(agenda: &[&str]) -> Command {
    Command::EndAct {
        agenda: agenda.iter().map(|a| (*a).to_owned()).collect(),
    }
}

fn table() -> Table {
    Table::new(Mode::Cinematic)
        .with("ann", pc().player("1"))
        .with("bob", pc().player("2"))
        .with("cid", pc().player("2"))
}

/// The Acts are counted; each player gets one Story Point, even with two
/// characters who furthered their Agenda.
#[test]
fn story_point_per_player_at_the_end_of_an_act() {
    let mut t = table();
    let ev = t.run(end_act(&["ann", "bob", "cid"]), &[]);
    assert_eq!(ev[0], Event::ActEnded { act: 1 });
    assert!(ev.contains(&Event::StoryPointAwarded { player: "1".into() }));
    assert_eq!(
        ev.iter()
            .filter(|e| matches!(e, Event::StoryPointAwarded { .. }))
            .count(),
        2,
        "one per player"
    );
    assert_eq!(t.state.story_points.get("2"), Some(&1));
    let ev = t.run(end_act(&[]), &[]);
    assert_eq!(ev, vec![Event::ActEnded { act: 2 }]);
    t.run(end_act(&[]), &[]);
    assert_eq!(t.state.acts_ended, 3);
    assert!(
        t.try_run(end_act(&[]), &[]).is_err(),
        "a scenario has three Acts [p. 216]"
    );
}

/// Never more than three Story Points [p. 61].
#[test]
fn at_most_three() {
    let mut t = table();
    let ev = t.run(
        Command::SetStoryPoints {
            player: "1".into(),
            points: 3,
        },
        &[],
    );
    assert_eq!(
        ev,
        vec![Event::StoryPointsChanged {
            player: "1".into(),
            from: 0,
            to: 3
        }]
    );
    let ev = t.run(end_act(&["ann"]), &[]);
    assert_eq!(ev, vec![Event::ActEnded { act: 1 }], "no change at three");
    assert_eq!(t.state.story_points.get("1"), Some(&3));
    assert!(matches!(
        t.try_run(
            Command::SetStoryPoints {
                player: "1".into(),
                points: 4
            },
            &[]
        ),
        Err(RuleError::NotAllowed(_))
    ));
}

/// Only player characters with a player further an Agenda; no Acts and
/// no Story Points in Campaign play (F-06).
#[test]
fn checks() {
    let mut t = table().with("drone", npc());
    assert!(t.try_run(end_act(&["drone"]), &[]).is_err());
    assert!(t.try_run(end_act(&["nobody"]), &[]).is_err());
    assert_eq!(t.state.acts_ended, 0, "a refused command changes nothing");

    let mut t = Table::new(Mode::Campaign).with("ann", pc().player("1"));
    assert!(t.try_run(end_act(&[]), &[]).is_err());
    assert!(
        t.try_run(
            Command::SetStoryPoints {
                player: "1".into(),
                points: 1
            },
            &[]
        )
        .is_err()
    );
}

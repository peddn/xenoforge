//! Talents of phase 2b: situational bonuses (A-10), Analysis,
//! Investigator and Pull Rank [p. 74–79].

mod common;

use common::*;
use xenoforge_engine::roll::{Outcome, RollRequest};
use xenoforge_engine::state::Mode;
use xenoforge_engine::{Choice, Command, Event, RuleError};

fn act(actor: &str, action: &str, target: Option<&str>) -> Command {
    Command::PerformAction {
        actor: actor.into(),
        action: action.into(),
        target: target.map(Into::into),
    }
}

fn stress(t: &Table, id: &str) -> u8 {
    t.actor(id).stress
}

fn hint(events: &[Event], key: &str) -> Option<bool> {
    events.iter().find_map(|e| match e {
        Event::Hint { text, gm_only, .. } if text == key => Some(*gm_only),
        _ => None,
    })
}

// ---- situational talents ---------------------------------------------------

/// Stealthy: +2 to MOBILITY when sneaking [p. 79]. The roll dialog
/// offers it; the bonus counts only when the player ticks it (A-10).
#[test]
fn situational_talent_is_ticked() {
    let t = Table::new(Mode::Campaign).with("a", pc().attr("agility", 3).talent("stealthy"));
    let plain = t
        .engine
        .preview(&t.state, RollRequest::skill("a", "mobility"))
        .unwrap();
    assert_eq!(plain.talents, ["stealthy"]);
    assert_eq!(plain.base_dice, 3);

    let mut ticked = RollRequest::skill("a", "mobility");
    ticked.talents.push("stealthy".into());
    let preview = t.engine.preview(&t.state, ticked).unwrap();
    assert_eq!(preview.base_dice, 5);
    assert!(
        preview
            .modifications
            .iter()
            .any(|m| m.value == 2 && m.source == "talent/stealthy")
    );

    let other = t
        .engine
        .preview(&t.state, RollRequest::skill("a", "observation"))
        .unwrap();
    assert!(other.talents.is_empty());
}

/// A situational talent for another skill is refused.
#[test]
fn situational_talent_must_fit() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("wits", 3).talent("stealthy"));
    let mut request = RollRequest::skill("a", "observation");
    request.talents.push("stealthy".into());
    assert!(matches!(
        t.try_run(Command::Roll { request }, &[]),
        Err(RuleError::NotAllowed(_))
    ));
    // Ticked twice it still counts once: the roll is refused.
    let mut request = RollRequest::skill("a", "mobility");
    request.talents = vec!["stealthy".into(), "stealthy".into()];
    assert!(matches!(
        t.try_run(Command::Roll { request }, &[]),
        Err(RuleError::NotAllowed(_))
    ));
}

/// Rolls asked for by a talent offer situational talents too (A-10).
#[test]
fn situational_talent_on_a_talent_roll() {
    let mut t = Table::new(Mode::Campaign).with(
        "a",
        pc().attr("wits", 2)
            .talent("investigator")
            .talent("watchful"),
    );
    let ev = t.run(act("a", "investigate", None), &[]);
    assert_eq!(
        options(&ev),
        [Choice::UseTalent("watchful".into()), Choice::Accept]
    );
    let ev = t.decide(&ev, Choice::UseTalent("watchful".into()), &[1, 2, 3, 4]);
    assert_eq!(started(&ev).0, 4, "WITS 2 and +2");
}

// ---- Investigator and Analysis --------------------------------------------

/// Investigator: OBSERVATION; on a success the game master answers
/// questions [p. 74]. Only the talent grants the action.
#[test]
fn investigator() {
    let mut t = Table::new(Mode::Campaign)
        .with("a", pc().attr("wits", 2).talent("investigator"))
        .with("b", pc().attr("wits", 2));
    assert!(matches!(
        t.try_run(act("b", "investigate", None), &[]),
        Err(RuleError::NotAllowed(_))
    ));
    let ev = t.run(act("a", "investigate", None), &[6, 2]);
    let ev = t.decide(&ev, Choice::Accept, &[]);
    assert_eq!(finished(&ev).0, Outcome::Success);
    assert_eq!(hint(&ev, "investigator.questions"), Some(false));
}

/// Analysis (F-34: the other PCs at short range, chosen by the game
/// master): on a success questions and stress −1 for the other PCs at
/// SHORT range; the game master picks them [p. 77].
#[test]
fn analysis_success() {
    let mut t = Table::new(Mode::Campaign)
        .with("s", pc().attr("wits", 2).talent("analysis"))
        .with("b", pc().stress(2))
        .with("c", pc().stress(1));
    let ev = t.run(act("s", "analyze", None), &[6, 3]);
    let ev = t.decide(&ev, Choice::Accept, &[]);
    assert_eq!(hint(&ev, "analysis.questions"), Some(false));
    let candidates = candidates(&t, &ev);
    assert_eq!(candidates, ["b", "c"]);
    t.decide(&ev, Choice::Targets(vec!["b".into()]), &[]);
    assert_eq!(
        (stress(&t, "b"), stress(&t, "c"), stress(&t, "s")),
        (1, 1, 0)
    );
}

/// The game master's choice is an event of its own, also when it changes
/// nothing: no one picked, or stress −1 at 0 (F-39).
#[test]
fn choosing_targets_is_logged() {
    let mut t = Table::new(Mode::Campaign)
        .with("s", pc().attr("wits", 2).talent("analysis"))
        .with("b", pc());
    let ev = t.run(act("s", "analyze", None), &[6, 3]);
    let ev = t.decide(&ev, Choice::Accept, &[]);
    assert!(has(&ev, |e| matches!(
        e,
        Event::ChooseTargets { actor, source, .. } if actor == "s" && source == "analysis"
    )));
    let chosen = t.decide(&ev, Choice::Targets(vec!["b".into()]), &[]);
    assert_eq!(
        chosen,
        [Event::TargetsChosen {
            pending: pending(&ev),
            actor: "s".into(),
            source: "analysis".into(),
            targets: vec!["b".into()],
        }],
        "stress stays 0, but the choice is in the log"
    );
    assert!(t.state.pending.is_empty());
}

/// Analysis: a failure raises the others' stress by one [p. 77].
#[test]
fn analysis_failure() {
    let mut t = Table::new(Mode::Campaign)
        .with("s", pc().attr("wits", 2).talent("analysis"))
        .with("b", pc().stress(2))
        .with("c", pc().stress(1));
    let ev = t.run(act("s", "analyze", None), &[1, 2]);
    let ev = t.decide(&ev, Choice::Accept, &[]);
    assert_eq!(finished(&ev).0, Outcome::Failure);
    assert_eq!(hint(&ev, "analysis.questions"), None);
    t.decide(&ev, Choice::Targets(vec!["b".into(), "c".into()]), &[]);
    assert_eq!((stress(&t, "b"), stress(&t, "c")), (3, 2));
}

// ---- Pull Rank -------------------------------------------------------------

fn officer() -> Table {
    Table::new(Mode::Campaign)
        .with("o", pc().attr("empathy", 3).talent("pull_rank"))
        .with("b", pc().attr("empathy", 2))
}

/// Pull Rank needs someone else as target [p. 76].
#[test]
fn pull_rank_needs_a_target() {
    let mut t = officer();
    for target in [None, Some("o")] {
        assert!(matches!(
            t.try_run(act("o", "pull_rank_order", target), &[]),
            Err(RuleError::NotAllowed(_))
        ));
    }
    assert_eq!(stress(&t, "o"), 0, "a refused order costs nothing");
    assert!(matches!(
        t.try_run(act("o", "pull_rank_order", Some("nobody")), &[]),
        Err(RuleError::UnknownActor(_))
    ));
    // Other actions take no target.
    let mut t = Table::new(Mode::Campaign)
        .with("a", pc().attr("wits", 2).talent("investigator"))
        .with("b", pc());
    assert!(matches!(
        t.try_run(act("a", "investigate", Some("b")), &[]),
        Err(RuleError::NotAllowed(_))
    ));
}

/// COMMAND against MANIPULATION, then stress +1 (F-56) [p. 76].
#[test]
fn pull_rank_is_opposed() {
    let mut t = officer();
    let ev = t.run(
        act("o", "pull_rank_order", Some("b")),
        &[/* b */ 6, 1, /* o */ 6, 6, 2],
    );
    let (base, stress_dice, _, _) = started(&ev);
    assert_eq!(
        (base, stress_dice),
        (3, 0),
        "the stress comes after the throw"
    );
    assert_eq!(stress(&t, "o"), 1);
    let ev = t.decide(&ev, Choice::Accept, &[]);
    assert_eq!(finished(&ev), (Outcome::Success, 2, 0));
    assert!(has(&ev, |e| matches!(
        e,
        Event::RollFinished {
            against: Some(1),
            ..
        }
    )));
}

/// Hothead gives the target +2 against Pull Rank [p. 78].
#[test]
fn pull_rank_against_a_hothead() {
    let mut t = Table::new(Mode::Campaign)
        .with("o", pc().attr("empathy", 3).talent("pull_rank"))
        .with("b", pc().attr("empathy", 2).talent("hothead"));
    let ev = t.run(
        act("o", "pull_rank_order", Some("b")),
        &[/* b */ 6, 6, 1, 2, /* o */ 6, 3, 2],
    );
    let resist = ev
        .iter()
        .find_map(|e| match e {
            Event::RollStarted {
                actor: Some(a),
                base_dice,
                modifications,
                ..
            } if a == "b" => Some((*base_dice, modifications.clone())),
            _ => None,
        })
        .unwrap();
    assert_eq!(resist.0, 4);
    assert!(
        resist
            .1
            .iter()
            .any(|m| m.source == "talent/hothead" && m.value == 2)
    );
    let ev = t.decide(&ev, Choice::Accept, &[]);
    assert_eq!(finished(&ev).0, Outcome::Failure);
}

/// An android officer pays no stress; the game master gets a note
/// (F-55).
#[test]
fn pull_rank_by_an_android() {
    let mut t = Table::new(Mode::Campaign)
        .with("o", pc().android().attr("empathy", 3).talent("pull_rank"))
        .with("b", pc().attr("empathy", 2));
    let ev = t.run(
        act("o", "pull_rank_order", Some("b")),
        &[/* b */ 2, 3, /* o */ 6, 2, 3],
    );
    assert_eq!(hint(&ev, "pull_rank.android"), Some(true));
    assert_eq!(stress(&t, "o"), 0);
}

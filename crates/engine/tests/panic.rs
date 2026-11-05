//! Panic [p. 104–105] with the rulings F-19, F-23 to F-30, F-39, F-42 to
//! F-44.

mod common;

use common::*;
use xenoforge_engine::roll::{Outcome, RollRequest, Speed};
use xenoforge_engine::state::{ActivePanic, Actor, Ending, Mode};
use xenoforge_engine::{Choice, Command, Event, RuleError, Step};

fn panicking(effect: &str, value: u8) -> Option<ActivePanic> {
    Some(ActivePanic {
        effect: effect.into(),
        value,
        effects_end: Some(Ending::Never),
        started_broken: false,
    })
}

fn panic_roll(t: &mut Table, actor: &str, d6: u8) -> Vec<Event> {
    t.run(
        Command::PanicRoll {
            actor: actor.into(),
        },
        &[d6],
    )
}

/// p. 104–105: D6 + STRESS LEVEL; 4 + 5 = 9 Drop Item, stress +1.
#[test]
fn panic_value() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().stress(4));
    let ev = panic_roll(&mut t, "a", 5);
    assert_eq!(panic_effect(&ev), (9, "drop_item".into()));
    assert!(has(&ev, |e| matches!(e, Event::ItemDropped { .. })));
    assert_eq!(t.actor("a").stress, 5);
}

/// Nerves of Steel: −2 on Panic Rolls [p. 78].
#[test]
fn nerves_of_steel() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().stress(4).talent("nerves_of_steel"));
    let ev = panic_roll(&mut t, "a", 5);
    assert_eq!(panic_effect(&ev), (7, "nervous_twitch".into()));
}

/// Below 1 counts as the first row (docs/datenformat.md).
#[test]
fn below_one_keeps_it_together() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().talent("nerves_of_steel"));
    let ev = panic_roll(&mut t, "a", 1);
    assert_eq!(panic_effect(&ev), (1, "keeping_it_together".into()));
    assert!(t.actor("a").panic.is_none());
}

/// More Panic [p. 104]: a lower result is one step more severe; a higher
/// one replaces; equal applies again (F-23); beyond 15+ stays Catatonic
/// (F-24).
#[test]
fn more_panic() {
    let cases = [
        // (running, stress, d6, expected value, expected effect)
        (10, 3, 5, 11, "seek_cover"),
        (10, 7, 6, 13, "flee"),
        (14, 0, 3, 15, "catatonic"),
        (10, 4, 6, 10, "freeze"),
        (15, 0, 2, 16, "catatonic"),
    ];
    for (running, stress, d6, value, effect) in cases {
        let mut pc = pc().stress(stress);
        pc.panic = panicking("x", running);
        let mut t = Table::new(Mode::Campaign).with("a", pc);
        let ev = panic_roll(&mut t, "a", d6);
        assert_eq!(
            panic_effect(&ev),
            (value, effect.into()),
            "running {running}, rolled {}",
            stress + d6
        );
        let raised = ev.iter().find_map(|e| match e {
            Event::PanicRolled { raised_from, .. } => Some(*raised_from),
            _ => None,
        });
        assert_eq!(raised.unwrap().is_some(), value > stress + d6);
    }
}

/// Stress never goes below 0 (F-19).
#[test]
fn stress_floor() {
    let mut pc = pc();
    pc.panic = panicking("freeze", 10);
    let mut t = Table::new(Mode::Campaign).with("a", pc);
    let ev = panic_roll(&mut t, "a", 1); // raised to 11 Seek Cover: −1
    assert_eq!(panic_effect(&ev).1, "seek_cover");
    assert_eq!(t.actor("a").stress, 0);
    assert!(!has(&ev, |e| matches!(e, Event::StressChanged { .. })));
}

/// Panic 1–9 after a roll keeps its result (F-44); Tremble then takes
/// AGILITY −2 until the panic stops.
#[test]
fn panic_below_ten_keeps_the_roll() {
    let mut t =
        Table::new(Mode::Campaign).with("a", pc().attr("wits", 2).attr("agility", 3).stress(3));
    let ev = t.roll(
        RollRequest::skill("a", "observation"),
        &[6, 6, 1, 2, 3, /* panic */ 5],
    );
    assert_eq!(panic_effect(&ev), (8, "tremble".into()));
    assert_eq!(finished(&ev), (Outcome::Success, 2, 1));
    let ev = t.roll(RollRequest::skill("a", "mobility"), &[2, 3, 4, 5]);
    assert_eq!(started(&ev).0, 1, "AGILITY 3 − 2");
    let _ = t.decide(&ev, Choice::Accept, &[]);
    t.run(Command::Step { step: Step::Turn }, &[]);
    assert!(t.actor("a").panic.is_none(), "one Turn passed");
}

/// 10+ cancels the action, successes or not [p. 104].
#[test]
fn panic_ten_plus_cancels() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("wits", 3).stress(5));
    let ev = t.roll(
        RollRequest::skill("a", "observation"),
        &[6, 6, 6, 1, 2, 3, 4, 5, /* panic */ 6],
    );
    assert_eq!(panic_effect(&ev), (11, "seek_cover".into()));
    assert_eq!(finished(&ev).0, Outcome::Cancelled);
    assert_eq!(t.actor("a").stress, 4);
}

/// Freeze after a slow action: the cancelled action is the lost one;
/// after a fast action the next slow action is lost (F-28).
#[test]
fn freeze_and_the_lost_action() {
    for (speed, already) in [(Speed::Slow, true), (Speed::Fast, false)] {
        let mut t = Table::new(Mode::Campaign).with("a", pc().attr("agility", 1).stress(4));
        let ev = t.roll(
            RollRequest::skill("a", "ranged_combat").action(speed),
            &[2, 1, 2, 3, 4, /* panic */ 6],
        );
        assert_eq!(panic_effect(&ev).1, "freeze");
        assert!(has(
            &ev,
            |e| matches!(e, Event::NextSlowActionLost { already_lost, .. } if *already_lost == already)
        ));
        assert_eq!(t.actor("a").loses_next_slow_action, !already);
    }
}

/// Freeze lasts one Round; the panic itself runs on until it stops (F-25).
#[test]
fn panic_outlasts_its_effect() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().stress(4));
    panic_roll(&mut t, "a", 6);
    assert!(t.actor("a").panic.as_ref().unwrap().effects_end.is_some());
    t.run(Command::Step { step: Step::Round }, &[]);
    let panic = t.actor("a").panic.clone().unwrap();
    assert_eq!((panic.value, panic.effects_end), (10, None));
    // Still panicking: a lower result is raised (More Panic).
    t.state.actors.get_mut("a").unwrap().stress = 0;
    let ev = panic_roll(&mut t, "a", 2);
    assert_eq!(panic_effect(&ev).0, 11);
}

/// Scream: friendly characters who hear it roll for panic [p. 105]. The
/// game master picks them among human PCs (F-39).
#[test]
fn scream_makes_listeners_panic() {
    let mut t = Table::new(Mode::Campaign)
        .with("a", pc().stress(6))
        .with("b", pc().stress(2))
        .with("ash", pc().android())
        .with("npc", Actor::npc());
    let ev = panic_roll(&mut t, "a", 6);
    assert_eq!(panic_effect(&ev).1, "scream");
    let candidates = candidates(&t, &ev);
    assert_eq!(candidates, ["b"]);
    let ev = t.decide(&ev, Choice::Targets(vec!["b".into()]), &[5]);
    assert_eq!(panic_effect(&ev), (7, "nervous_twitch".into()));
    assert_eq!(t.actor("b").stress, 3);
}

/// Overkill may replace a result of 11+ [p. 104; F-30]; the value still
/// counts for the trauma check (F-26).
#[test]
fn overkill() {
    let mut t = Table::new(Mode::Campaign)
        .with("marine", pc().stress(7).talent("overkill"))
        .with("b", pc());
    let ev = panic_roll(&mut t, "marine", 6);
    assert_eq!(options(&ev), [Choice::Overkill, Choice::Accept]);
    let ev = t.decide(&ev, Choice::Overkill, &[]);
    assert_eq!(panic_effect(&ev), (13, "overkill_effect".into()));
    assert!(has(&ev, |e| matches!(e, Event::ChooseTargets { .. })));
    let marine = t.actor("marine");
    assert_eq!(marine.panic.as_ref().unwrap().effect, "overkill_effect");
    assert!(marine.panic_13_plus);

    let mut t = Table::new(Mode::Campaign).with("marine", pc().stress(5).talent("overkill"));
    let ev = panic_roll(&mut t, "marine", 6);
    let ev = t.decide(&ev, Choice::Accept, &[]);
    assert_eq!(panic_effect(&ev), (11, "seek_cover".into()));
}

/// Broken ends panic [p. 104].
#[test]
fn broken_stops_panic() {
    let mut pc = pc().attr("strength", 1);
    pc.panic = panicking("tremble", 8);
    let mut t = Table::new(Mode::Campaign).with("a", pc);
    let ev = t.run(
        Command::Damage {
            actor: "a".into(),
            amount: 1,
        },
        &[],
    );
    assert!(has(&ev, |e| matches!(e, Event::Broken { .. })));
    assert!(has(&ev, |e| matches!(e, Event::PanicStopped { .. })));
}

/// NPCs and androids never roll for panic (F-43).
#[test]
fn no_panic_rolls_for_npcs() {
    let mut t = Table::new(Mode::Campaign)
        .with("npc", Actor::npc())
        .with("ash", pc().android());
    for actor in ["npc", "ash"] {
        let err = t
            .try_run(
                Command::PanicRoll {
                    actor: actor.into(),
                },
                &[],
            )
            .unwrap_err();
        assert!(matches!(err, RuleError::NotAllowed(_)));
    }
    let err = t
        .try_run(
            Command::ChangeStress {
                actors: vec!["npc".into()],
                delta: 1,
            },
            &[],
        )
        .unwrap_err();
    assert!(matches!(err, RuleError::NotAllowed(_)));
}

/// The game master ends a panic by hand, e.g. Berserk when the target is
/// Broken (F-29) [p. 104, 105].
#[test]
fn game_master_stops_panic() {
    let mut pc = pc();
    pc.panic = panicking("berserk", 14);
    let mut t = Table::new(Mode::Campaign).with("a", pc);
    t.run(Command::StopPanic { actor: "a".into() }, &[]);
    assert!(t.actor("a").panic.is_none());
    let again = t.try_run(Command::StopPanic { actor: "a".into() }, &[]);
    assert!(again.is_err());
}

// ---- trauma after the session [p. 104; F-01, F-26, F-27, F-42] ----------------

#[test]
fn trauma_on_a_failed_empathy_roll() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("empathy", 2).stress(7));
    panic_roll(&mut t, "a", 6); // 13 Flee
    panic_roll(&mut t, "a", 6); // 12, raised to 14: still one roll (F-42)
    let ev = t.run(Command::EndSession, &[1, 2, /* trauma d6 */ 3]);
    assert!(has(
        &ev,
        |e| matches!(e, Event::TraumaCheck { d6: Some(3), trauma: Some(t), .. } if t == "nightmares")
    ));
    let a = t.actor("a");
    assert!(!a.panic_13_plus);
    assert!(a.sources.iter().any(|s| s.origin.entry() == "nightmares"));
}

#[test]
fn no_trauma_on_success_or_below_thirteen() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("empathy", 2).stress(7));
    panic_roll(&mut t, "a", 6);
    let ev = t.run(Command::EndSession, &[6, 2]);
    assert!(has(&ev, |e| matches!(
        e,
        Event::TraumaCheck { trauma: None, .. }
    )));

    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("empathy", 2).stress(6));
    panic_roll(&mut t, "a", 6); // 12
    let ev = t.run(Command::EndSession, &[]);
    assert!(!has(&ev, |e| matches!(e, Event::TraumaCheck { .. })));
    // The end of the session is always reported, first.
    assert_eq!(ev.first(), Some(&Event::SessionEnded));
}

/// The raised value counts for 13+ (F-26).
#[test]
fn raised_value_counts_for_trauma() {
    let mut pc = pc().stress(0);
    pc.panic = panicking("scream", 12);
    let mut t = Table::new(Mode::Campaign).with("a", pc);
    let ev = panic_roll(&mut t, "a", 2);
    assert_eq!(panic_effect(&ev).0, 13);
    assert!(t.actor("a").panic_13_plus);
}

/// A panic that begins while Broken ends at once; its one-shot effects
/// still happen (F-48 default). The alternative lets it run on.
#[test]
fn panic_while_broken_ruling_f48() {
    for (option, runs) in [("ends_at_once", false), ("runs_on", true)] {
        let mut t = Table::new(Mode::Campaign).with("a", pc().stress(3).health(0));
        t.state.rulings.insert("F-48".into(), option.into());
        let ev = panic_roll(&mut t, "a", 6); // 9 Drop Item
        assert!(has(&ev, |e| matches!(e, Event::ItemDropped { .. })));
        assert_eq!(t.actor("a").stress, 4);
        assert_eq!(t.actor("a").panic.is_some(), runs, "{option}");
    }
}

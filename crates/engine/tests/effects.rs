//! The effect system (plan section 4.4): sources, durations, triggers,
//! periodic effects, blocks, rulings; Hayes freezing [p. 108].

mod common;

use common::*;
use xenoforge_engine::data::effects::{
    Change, Cold, Effect, EndEvent, Lasts, ModTarget, Modified, Target, Unit,
};
use xenoforge_engine::data::model::Condition;
use xenoforge_engine::roll::{Outcome, RollRequest};
use xenoforge_engine::state::{Ending, GameState, Mode, Origin};
use xenoforge_engine::{Choice, Command, Engine, Event, NewSource, RuleError, Step};

fn apply(t: &mut Table, actor: &str, source: NewSource, faces: &[u8]) -> Vec<Event> {
    t.run(
        Command::Apply {
            actor: actor.into(),
            source,
        },
        faces,
    )
}

fn shift(t: &mut Table, new_day: bool, faces: &[u8]) -> Vec<Event> {
    t.run(
        Command::Step {
            step: Step::Shift { new_day },
        },
        faces,
    )
}

fn signature(t: &mut Table, actor: &str) -> Result<Vec<Event>, RuleError> {
    t.try_run(
        Command::UseSignatureItem {
            actor: actor.into(),
        },
        &[],
    )
}

/// p. 108: Hayes (STRENGTH 2, STAMINA 0) is Freezing and rolls two dice
/// without success: 1 damage and STRESS 0 → 1, only +1 in all (F-32).
/// His next roll has one Stress Die.
#[test]
fn book_p108_hayes_freezes() {
    let mut t = Table::new(Mode::Campaign).with("hayes", pc().attr("strength", 2));
    apply(
        &mut t,
        "hayes",
        NewSource::Condition {
            id: "freezing".into(),
            cold: Some(Cold::SubZero),
        },
        &[],
    );
    let ev = shift(&mut t, false, &[2, 3]);
    assert_eq!(started(&ev).0, 2);
    let _ = t.decide(&ev, Choice::Accept, &[]);
    let hayes = t.actor("hayes");
    assert_eq!((hayes.health, hayes.stress), (1, 1));
    let ev = t.roll(RollRequest::skill("hayes", "stamina"), &[2, 3, 4]);
    assert_eq!(started(&ev).1, 1);
}

/// Freezing rolls by cold [p. 107]: deep space every Turn, above freezing
/// once a day.
#[test]
fn freezing_intervals() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("strength", 2));
    apply(
        &mut t,
        "a",
        NewSource::Condition {
            id: "freezing".into(),
            cold: Some(Cold::DeepSpace),
        },
        &[],
    );
    let ev = t.run(Command::Step { step: Step::Turn }, &[6, 2]);
    assert!(has(&ev, |e| matches!(e, Event::RollStarted { .. })));

    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("strength", 2));
    apply(
        &mut t,
        "a",
        NewSource::Condition {
            id: "freezing".into(),
            cold: Some(Cold::AboveFreezing),
        },
        &[],
    );
    let ev = shift(&mut t, false, &[]);
    assert!(!has(&ev, |e| matches!(e, Event::RollStarted { .. })));
    let ev = shift(&mut t, true, &[6, 2]);
    assert!(has(&ev, |e| matches!(e, Event::RollStarted { .. })));

    let missing = t.try_run(
        Command::Apply {
            actor: "a".into(),
            source: NewSource::Condition {
                id: "freezing".into(),
                cold: None,
            },
        },
        &[],
    );
    assert!(missing.is_err(), "Freezing needs a cold level");
}

/// Freezing blocks stress relief, also the signature item (F-31), until
/// the character is warm [p. 107].
#[test]
fn freezing_blocks_the_signature_item_until_warm() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().stress(2));
    apply(
        &mut t,
        "a",
        NewSource::Condition {
            id: "freezing".into(),
            cold: Some(Cold::SubZero),
        },
        &[],
    );
    assert!(matches!(
        signature(&mut t, "a"),
        Err(RuleError::NotAllowed(_))
    ));
    t.run(
        Command::End {
            actor: "a".into(),
            event: EndEvent::Warm,
        },
        &[],
    );
    signature(&mut t, "a").unwrap();
    assert_eq!(t.actor("a").stress, 1);
}

/// Signature item: once per Act in Cinematic play [p. 104].
#[test]
fn signature_item_once_per_act() {
    let mut t = Table::new(Mode::Cinematic).with("a", pc().stress(3));
    signature(&mut t, "a").unwrap();
    assert!(signature(&mut t, "a").is_err());
    t.run(Command::EndAct, &[]);
    signature(&mut t, "a").unwrap();
    assert_eq!(t.actor("a").stress, 1);

    let mut t = Table::new(Mode::Campaign).with("a", pc().stress(3));
    assert!(
        t.try_run(Command::EndAct, &[]).is_err(),
        "no Acts in Campaign play"
    );
}

/// Crotch hit: 1 damage on every MOBILITY roll, D6 days [p. 100]. The
/// damage gives stress like any other (F-32).
#[test]
fn crotch_hit_trigger() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("agility", 2));
    let ev = apply(&mut t, "a", NewSource::Injury("crotch_hit".into()), &[3]);
    assert!(has(&ev, |e| matches!(
        e,
        Event::SourceApplied {
            ends: Ending::After {
                unit: Unit::Shift,
                left: 12
            },
            ..
        }
    )));
    let ev = t.roll(RollRequest::skill("a", "mobility"), &[2, 3]);
    let ev = t.decide(&ev, Choice::Accept, &[]);
    assert!(has(&ev, |e| matches!(e, Event::Damaged { amount: 1, .. })));
    let a = t.actor("a");
    assert_eq!((a.health, a.stress), (3, 1));
}

/// Broken ribs: MOBILITY −2 for 2D6 days, four Shifts a day [p. 100].
#[test]
fn broken_ribs_heal_after_their_days() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("agility", 3));
    apply(
        &mut t,
        "a",
        NewSource::Injury("broken_ribs".into()),
        &[1, 1],
    );
    let ev = t.roll(RollRequest::skill("a", "mobility"), &[2]);
    let (base, _, _, mods) = started(&ev);
    assert_eq!(base, 1);
    assert!(mods.iter().any(|m| m.source == "injury/broken_ribs"));
    let _ = t.decide(&ev, Choice::Accept, &[]);
    for _ in 0..7 {
        let ev = shift(&mut t, false, &[]);
        assert!(!has(&ev, |e| matches!(e, Event::SourceEnded { .. })));
    }
    let ev = shift(&mut t, false, &[]);
    assert!(has(&ev, |e| matches!(e, Event::SourceEnded { .. })));
}

/// Sprained ankle: no running until MEDICAL AID [p. 100].
#[test]
fn sprained_ankle_blocks_running() {
    let mut t = Table::new(Mode::Campaign).with("a", pc());
    apply(&mut t, "a", NewSource::Injury("sprained_ankle".into()), &[]);
    let run = Command::PerformAction {
        actor: "a".into(),
        action: "run".into(),
    };
    assert!(matches!(
        t.try_run(run.clone(), &[]),
        Err(RuleError::NotAllowed(_))
    ));
    t.run(
        Command::End {
            actor: "a".into(),
            event: EndEvent::MedicalAid,
        },
        &[],
    );
    t.run(run, &[]);
}

/// Dock needs a Docking Umbilical [p. 193]; characters have none.
#[test]
fn capabilities_are_needed() {
    let mut t = Table::new(Mode::Campaign).with("a", pc());
    let ev = t.try_run(
        Command::PerformAction {
            actor: "a".into(),
            action: "dock".into(),
        },
        &[],
    );
    assert!(matches!(ev, Err(RuleError::NotAllowed(_))));
}

/// Depression: a daily EMPATHY roll; a bad day blocks stress relief until
/// the next day [p. 101].
#[test]
fn depression() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("empathy", 2).stress(1));
    apply(&mut t, "a", NewSource::Trauma("depression".into()), &[]);
    shift(&mut t, true, &[1, 2]);
    assert_eq!(t.actor("a").stress, 2);
    assert!(signature(&mut t, "a").is_err());
    shift(&mut t, false, &[]);
    assert!(signature(&mut t, "a").is_err(), "still the same day");
    shift(&mut t, true, &[6, 2]);
    signature(&mut t, "a").unwrap();
}

/// Alcoholism: drink every Shift or stress +1 [p. 101].
#[test]
fn alcoholism_asks_every_shift() {
    let mut t = Table::new(Mode::Campaign).with("a", pc());
    apply(&mut t, "a", NewSource::Trauma("alcoholism".into()), &[]);
    let ev = shift(&mut t, false, &[]);
    assert!(has(&ev, |e| matches!(e, Event::Question { .. })));
    t.decide(&ev, Choice::Answer(false), &[]);
    assert_eq!(t.actor("a").stress, 1);
    let ev = shift(&mut t, false, &[]);
    t.decide(&ev, Choice::Answer(true), &[]);
    assert_eq!(t.actor("a").stress, 1);
    let hints = t.engine.hints(&t.state, "a");
    assert!(hints.iter().any(|(_, text, _)| text == "alcoholism.hint"));
}

/// Nightmares, as printed: a successful EMPATHY roll brings the nightmare,
/// stress +1 and no relief for a full day, four Shifts (F-17). The
/// alternative turns it around.
#[test]
fn nightmares_and_ruling_f17() {
    let sleep = |t: &mut Table, faces: &[u8]| {
        t.run(
            Command::Sleep {
                actors: vec!["a".into()],
            },
            faces,
        )
    };
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("empathy", 2).stress(1));
    apply(&mut t, "a", NewSource::Trauma("nightmares".into()), &[]);
    sleep(&mut t, &[6, 1]);
    assert_eq!(t.actor("a").stress, 2);
    for _ in 0..3 {
        shift(&mut t, false, &[]);
        assert!(signature(&mut t, "a").is_err());
    }
    shift(&mut t, false, &[]);
    signature(&mut t, "a").unwrap();

    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("empathy", 2).stress(1));
    t.state.rulings.insert("F-17".into(), "on_fail".into());
    apply(&mut t, "a", NewSource::Trauma("nightmares".into()), &[]);
    sleep(&mut t, &[6, 1]);
    assert_eq!(t.actor("a").stress, 1);
    sleep(&mut t, &[1, 2]);
    assert_eq!(t.actor("a").stress, 2);
}

/// Damage gives stress and may break [p. 98, 103].
#[test]
fn damage() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("strength", 2));
    let ev = t.run(
        Command::Damage {
            actor: "a".into(),
            amount: 3,
        },
        &[],
    );
    assert!(has(&ev, |e| matches!(e, Event::Broken { .. })));
    let a = t.actor("a");
    assert_eq!((a.health, a.stress), (0, 1));
}

#[test]
fn sources_can_be_removed_by_hand() {
    let mut t = Table::new(Mode::Campaign).with("a", pc());
    apply(&mut t, "a", NewSource::Trauma("phobia".into()), &[]);
    let remove = Command::Remove {
        actor: "a".into(),
        origin: Origin::Trauma("phobia".into()),
    };
    t.run(remove.clone(), &[]);
    assert!(t.actor("a").sources.is_empty());
    assert!(t.try_run(remove, &[]).is_err());
}

/// An effect with its own duration ends on its own; the rest of the
/// source stays (data format, `lasts` on an effect).
#[test]
fn effects_with_their_own_duration() {
    let mut rules = rules();
    rules.conditions.push(Condition {
        id: "dazzled".into(),
        page: 1,
        effects: vec![Effect::Modifier {
            target: Target::Bearer,
            applies_to: ModTarget {
                skills: vec!["observation".into()],
                ..ModTarget::default()
            },
            modifies: Modified::Dice,
            change: Change::Add(-1),
            at_least: None,
            lasts: Some(Lasts::Steps {
                unit: Unit::Round,
                count: 1,
            }),
            gm_only: false,
        }],
        ends: EndEvent::Healed,
    });
    let mut t = Table {
        engine: Engine::new(rules),
        state: GameState::new(Mode::Campaign).with("a", pc().attr("wits", 3)),
    };
    apply(
        &mut t,
        "a",
        NewSource::Condition {
            id: "dazzled".into(),
            cold: None,
        },
        &[],
    );
    assert_eq!(t.actor("a").sources.len(), 2, "the condition and its part");
    let ev = t.roll(RollRequest::skill("a", "observation"), &[1, 2]);
    let _ = t.decide(&ev, Choice::Accept, &[]);
    t.run(Command::Step { step: Step::Round }, &[]);
    assert_eq!(t.actor("a").sources.len(), 1);
    let ev = t.roll(RollRequest::skill("a", "observation"), &[1, 2, 3]);
    assert_eq!(finished_or_open(&ev), None);
}

fn finished_or_open(events: &[Event]) -> Option<Outcome> {
    events.iter().find_map(|e| match e {
        Event::RollFinished { outcome, .. } => Some(*outcome),
        _ => None,
    })
}

/// The state, including an open roll, can be stored and loaded.
#[test]
fn state_round_trips_through_json() {
    let mut t = Table::new(Mode::Cinematic).with("a", pc().attr("wits", 2).stress(1));
    t.roll(RollRequest::skill("a", "observation"), &[2, 3, 4]);
    assert_eq!(t.state.pending.len(), 1);
    let json = serde_json::to_string(&t.state).unwrap();
    let back: GameState = serde_json::from_str(&json).unwrap();
    assert_eq!(back, t.state);
}

/// A refused command changes nothing.
#[test]
fn errors_leave_the_state_alone() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().stress(2));
    let before = t.state.clone();
    let err = t.try_run(
        Command::ChangeStress {
            actors: vec!["a".into(), "ghost".into()],
            delta: 1,
        },
        &[],
    );
    assert!(matches!(err, Err(RuleError::UnknownActor(_))));
    assert_eq!(t.state, before);
}

/// Stoic works on STAMINA rolls that a condition asks for [p. 79].
#[test]
fn stoic_on_freezing_rolls() {
    let mut t = Table::new(Mode::Campaign).with(
        "a",
        pc().attr("strength", 1).attr("wits", 4).talent("stoic"),
    );
    apply(
        &mut t,
        "a",
        NewSource::Condition {
            id: "freezing".into(),
            cold: Some(Cold::SubZero),
        },
        &[],
    );
    let ev = shift(&mut t, false, &[]);
    assert_eq!(
        options(&ev),
        [Choice::UseTalent("stoic".into()), Choice::Accept]
    );
    let ev = t.decide(&ev, Choice::UseTalent("stoic".into()), &[6, 2, 3, 4]);
    assert_eq!(started(&ev).0, 4);
}

/// A Broken character makes no cold roll; the hint tells the game master
/// about the Death Roll [p. 98, 107].
#[test]
fn broken_characters_skip_condition_rolls() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().health(0));
    apply(
        &mut t,
        "a",
        NewSource::Condition {
            id: "freezing".into(),
            cold: Some(Cold::SubZero),
        },
        &[],
    );
    let ev = shift(&mut t, false, &[]);
    assert!(has(&ev, |e| matches!(e, Event::RollSkipped { .. })));
    let hints = t.engine.hints(&t.state, "a");
    assert!(hints.iter().any(|(_, text, _)| text == "freezing.broken"));
}

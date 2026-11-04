//! Rolls: book examples [p. 59–64, 107] and rule cases from
//! docs/rules/02-dice-skills-panic.md §8.4–8.5, with the rulings they use.

mod common;

use common::*;
use xenoforge_engine::data::effects::Unit;
use xenoforge_engine::roll::{Outcome, Purpose, RollKind, RollRequest};
use xenoforge_engine::state::{ActivePanic, Ending, Mode, Supply};
use xenoforge_engine::{Choice, Command, Event, RuleError};

fn max_pushes(events: &[Event]) -> u8 {
    events
        .iter()
        .find_map(|e| match e {
            Event::RollStarted { max_pushes, .. } => Some(*max_pushes),
            _ => None,
        })
        .unwrap()
}

// ---- book examples ---------------------------------------------------------

/// p. 59: Jeffries has STRENGTH 3 and no STAMINA: three Base Dice.
#[test]
fn book_p59_roll_without_skill() {
    let mut t = Table::new(Mode::Campaign).with("jeffries", pc().attr("strength", 3));
    let ev = t.roll(RollRequest::skill("jeffries", "stamina"), &[2, 3, 6]);
    let (base, stress, _, _) = started(&ev);
    assert_eq!((base, stress), (3, 0));
    let ev = t.decide(&ev, Choice::Accept, &[]);
    assert_eq!(finished(&ev), (Outcome::Success, 1, 0));
}

/// p. 61: Nat (STRENGTH 5, HEAVY MACHINERY 3, STRESS 2) fails, pushes:
/// STRESS 3, then 8 Base + 3 Stress Dice again; two successes.
#[test]
fn book_p61_push() {
    let mut t = Table::new(Mode::Campaign).with(
        "nat",
        pc().attr("strength", 5)
            .skill("heavy_machinery", 3)
            .stress(2),
    );
    let ev = t.roll(
        RollRequest::skill("nat", "heavy_machinery"),
        &[2, 3, 4, 5, 2, 3, 4, 5, 2, 3],
    );
    assert_eq!(started(&ev).0, 8);
    assert_eq!(started(&ev).1, 2);
    assert_eq!(options(&ev), [Choice::Push, Choice::Accept]);

    let ev = t.decide(&ev, Choice::Push, &[6, 6, 2, 3, 4, 5, 2, 3, 2, 3, 4]);
    assert_eq!(t.actor("nat").stress, 3);
    let thrown = ev
        .iter()
        .find_map(|e| match e {
            Event::DiceThrown { dice, .. } => Some(dice.clone()),
            _ => None,
        })
        .unwrap();
    assert_eq!(thrown.len(), 11, "8 Base + 3 Stress Dice");
    assert_eq!(finished(&ev), (Outcome::Success, 2, 1));
}

/// p. 63: WITS 4 + OBSERVATION 2 + motion tracker 1 + help 1 = 8 dice.
#[test]
fn book_p63_gear_and_help() {
    let mut t =
        Table::new(Mode::Campaign).with("charlize", pc().attr("wits", 4).skill("observation", 2));
    let ev = t.roll(
        RollRequest::skill("charlize", "observation")
            .modifier("motion_tracker", 1)
            .helpers(1),
        &[1, 2, 3, 4, 5, 1, 2, 6],
    );
    assert_eq!(started(&ev).0, 8);
}

/// p. 64: Charlize (4 dice) against Mother (8 dice), 2 to 2: she fails.
#[test]
fn book_p64_opposed_tie_fails() {
    let mut t = Table::new(Mode::Campaign).with("charlize", pc().attr("wits", 4));
    let ev = t.run(
        Command::Opposed {
            attacker: RollRequest::skill("charlize", "comtech"),
            defender: RollRequest::pool(8),
        },
        &[6, 6, 1, 2, 3, 4, 5, 1, /* Charlize */ 6, 6, 2, 3],
    );
    assert_eq!(
        options(&ev),
        [Choice::Push, Choice::Accept],
        "only the attacker may push"
    );
    let ev = t.decide(&ev, Choice::Accept, &[]);
    assert_eq!(finished(&ev), (Outcome::Failure, 2, 0));
    assert!(has(&ev, |e| matches!(
        e,
        Event::RollFinished {
            against: Some(2),
            ..
        }
    )));
}

/// p. 107: a STAMINA roll with two Base Dice and one success.
#[test]
fn book_p107_vacuum_stamina() {
    let mut t = Table::new(Mode::Campaign).with("charlize", pc().attr("strength", 2));
    let ev = t.roll(RollRequest::skill("charlize", "stamina"), &[6, 3]);
    let ev = t.decide(&ev, Choice::Accept, &[]);
    assert_eq!(finished(&ev).0, Outcome::Success);
}

// ---- pool [p. 62–63] -------------------------------------------------------

#[test]
fn help_counts_at_most_three() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("wits", 1));
    let ev = t.roll(
        RollRequest::skill("a", "observation").helpers(4),
        &[1, 2, 3, 4],
    );
    let (base, _, _, mods) = started(&ev);
    assert_eq!(base, 4);
    assert!(mods.iter().any(|m| m.source == "help" && m.value == 3));
}

#[test]
fn modifications_add_up() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("wits", 2));
    let ev = t.roll(
        RollRequest::skill("a", "observation")
            .modifier("gear", 2)
            .modifier("dark", -1),
        &[1, 2, 3],
    );
    assert_eq!(started(&ev).0, 3);
}

/// A negative overhang removes Stress Dice [p. 62]; a push then adds one
/// new Stress Die (F-20).
#[test]
fn overhang_removes_stress_dice_and_push_adds_one() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("agility", 2).stress(2));
    let ev = t.roll(
        RollRequest::skill("a", "mobility").modifier("hard", -3),
        &[3],
    );
    let (base, stress, removed, _) = started(&ev);
    assert_eq!((base, stress, removed), (0, 1, 1));
    let ev = t.decide(&ev, Choice::Push, &[4, 5]);
    assert_eq!(t.actor("a").stress, 3);
    assert_eq!(finished(&ev).0, Outcome::Failure);
}

/// No dice left: the roll fails at once, without a Story Point (F-21).
#[test]
fn no_dice_fails_without_story_point() {
    let mut t = Table::new(Mode::Cinematic).with("a", pc().attr("agility", 2).player("p"));
    t.state.story_points.insert("p".into(), 2);
    let ev = t.roll(
        RollRequest::skill("a", "mobility").modifier("hard", -3),
        &[],
    );
    assert_eq!(finished(&ev), (Outcome::Failure, 0, 0));
    assert!(options(&ev).is_empty());
}

// ---- push [p. 60–61] -------------------------------------------------------

#[test]
fn push_keeps_successes_and_adds_a_stress_die() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("wits", 3));
    let ev = t.roll(RollRequest::skill("a", "observation"), &[6, 2, 3]);
    let ev = t.decide(&ev, Choice::Push, &[4, 5, 6]);
    let dice = ev
        .iter()
        .find_map(|e| match e {
            Event::DiceThrown { dice, .. } => Some(dice.clone()),
            _ => None,
        })
        .unwrap();
    assert!(dice[0].locked && dice[0].face == 6);
    assert_eq!(dice.len(), 4);
    assert_eq!(t.actor("a").stress, 1);
    assert_eq!(finished(&ev), (Outcome::Success, 2, 1));
}

/// A facehugger forbids the push and calls for a Panic Roll [p. 61].
#[test]
fn facehugger_blocks_push() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("agility", 2).stress(1));
    let ev = t.roll(
        RollRequest::skill("a", "mobility"),
        &[2, 3, 1, /* panic d6 */ 2],
    );
    assert!(options(&ev).is_empty());
    assert!(has(&ev, |e| matches!(
        e,
        Event::PanicRolled { value: 3, .. }
    )));
    assert_eq!(finished(&ev).0, Outcome::Failure);
}

/// Past the Limit: STRENGTH rolls twice, each push stress +1 [p. 74].
#[test]
fn push_twice_with_talent() {
    let mut t =
        Table::new(Mode::Campaign).with("m", pc().attr("strength", 2).talent("past_the_limit"));
    let ev = t.roll(RollRequest::skill("m", "close_combat"), &[2, 3]);
    assert_eq!(max_pushes(&ev), 2);
    let ev = t.decide(&ev, Choice::Push, &[2, 3, 4]);
    assert_eq!(options(&ev), [Choice::Push, Choice::Accept]);
    let ev = t.decide(&ev, Choice::Push, &[2, 3, 4, 6]);
    assert_eq!(t.actor("m").stress, 2);
    assert_eq!(finished(&ev).0, Outcome::Success);
}

/// No second push after a facehugger in the first one (F-45).
#[test]
fn no_second_push_after_facehugger() {
    let mut t = Table::new(Mode::Campaign).with("m", pc().attr("strength", 2).talent("true_grit"));
    let ev = t.roll(RollRequest::skill("m", "close_combat"), &[2, 3]);
    let ev = t.decide(&ev, Choice::Push, &[2, 3, 1, /* panic */ 1]);
    assert!(options(&ev).is_empty());
    assert!(has(&ev, |e| matches!(e, Event::PanicRolled { .. })));
    assert_eq!(finished(&ev).0, Outcome::Failure);
}

/// The talent's second push counts for the attribute actually rolled
/// (F-35): Take Control rolls MANIPULATION with WITS, so Cunning (WITS)
/// applies and Compassion (EMPATHY) does not.
#[test]
fn second_push_follows_the_rolled_attribute() {
    let agent = pc()
        .attr("wits", 3)
        .attr("empathy", 1)
        .talent("take_control");
    let mut t = Table::new(Mode::Campaign).with("agent", agent.clone().talent("cunning"));
    let ev = t.roll(
        RollRequest::skill("agent", "manipulation").talent("take_control"),
        &[1, 2, 3],
    );
    assert_eq!(started(&ev).0, 3, "WITS 3 instead of EMPATHY 1");
    assert_eq!(max_pushes(&ev), 2);

    let mut t = Table::new(Mode::Campaign).with("medic", agent.talent("compassion"));
    let ev = t.roll(
        RollRequest::skill("medic", "manipulation").talent("take_control"),
        &[1, 2, 3],
    );
    assert_eq!(max_pushes(&ev), 1);
}

/// Authority: COMMAND instead of MANIPULATION [p. 74].
#[test]
fn authority_rolls_command() {
    let mut t = Table::new(Mode::Campaign).with(
        "marshal",
        pc().attr("empathy", 3)
            .skill("command", 3)
            .talent("authority"),
    );
    let ev = t.roll(
        RollRequest::skill("marshal", "manipulation").talent("authority"),
        &[1, 2, 3, 4, 5, 2],
    );
    assert_eq!(started(&ev).0, 6);
}

#[test]
fn talents_must_be_held_and_fit() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("wits", 3).talent("stoic"));
    let err = t
        .try_run(
            Command::Roll {
                request: RollRequest::skill("a", "manipulation").talent("take_control"),
            },
            &[],
        )
        .unwrap_err();
    assert!(matches!(err, RuleError::NotAllowed(_)));
    let err = t
        .try_run(
            Command::Roll {
                request: RollRequest::skill("a", "observation").talent("stoic"),
            },
            &[],
        )
        .unwrap_err();
    assert!(matches!(err, RuleError::NotAllowed(_)));
}

/// Breakthrough: once per session, an OBSERVATION roll passes without
/// dice [p. 77].
#[test]
fn breakthrough_once_per_session() {
    let mut t = Table::new(Mode::Campaign).with("s", pc().attr("wits", 3).talent("breakthrough"));
    let request = RollRequest::skill("s", "observation").talent("breakthrough");
    let ev = t.roll(request.clone(), &[]);
    assert_eq!(finished(&ev), (Outcome::Success, 1, 0));
    let again = t.try_run(
        Command::Roll {
            request: request.clone(),
        },
        &[],
    );
    assert!(matches!(again, Err(RuleError::NotAllowed(_))));
    t.run(Command::EndSession, &[]);
    t.roll(request, &[]);
}

// ---- who may push and roll Stress Dice --------------------------------------

#[test]
fn androids_roll_no_stress_dice_and_never_push() {
    let mut t = Table::new(Mode::Campaign).with("ash", pc().android().attr("wits", 2).stress(3));
    let ev = t.roll(RollRequest::skill("ash", "observation"), &[1, 2]);
    assert_eq!(started(&ev).1, 0);
    assert!(options(&ev).is_empty());
    assert_eq!(finished(&ev).0, Outcome::Failure);
}

#[test]
fn npcs_never_push() {
    let mut t = Table::new(Mode::Campaign).with("guard", npc().attr("agility", 2));
    let ev = t.roll(RollRequest::skill("guard", "ranged_combat"), &[1, 2]);
    assert!(options(&ev).is_empty());
}

/// A defender's panic of 10+ cancels the defence (F-22).
#[test]
fn defender_panic_cancels_the_defence() {
    let mut t = Table::new(Mode::Campaign)
        .with("pc", pc().attr("strength", 1).stress(5))
        .with("npc", npc().attr("strength", 1));
    let ev = t.run(
        Command::Opposed {
            attacker: RollRequest::skill("npc", "close_combat"),
            defender: RollRequest::skill("pc", "close_combat"),
        },
        &[
            6, 1, 2, 3, 4, 5, // defender: 1 Base, 5 Stress Dice
            6, // panic d6: 5 + 6 = 11
            6, // attacker
        ],
    );
    assert!(has(&ev, |e| matches!(
        e,
        Event::RollFinished {
            outcome: Outcome::Cancelled,
            ..
        }
    )));
    assert!(has(&ev, |e| matches!(
        e,
        Event::RollFinished {
            outcome: Outcome::Success,
            against: Some(0),
            ..
        }
    )));
}

// ---- other roll kinds -------------------------------------------------------

/// Death Roll: STAMINA without Stress Dice and without push [p. 99].
#[test]
fn death_roll() {
    let mut t = Table::new(Mode::Campaign)
        .with("a", pc().attr("strength", 2).skill("stamina", 1).stress(3));
    let ev = t.roll(RollRequest::new("a", RollKind::Death), &[1, 2, 3]);
    assert_eq!(started(&ev).1, 0);
    assert!(options(&ev).is_empty());
    assert_eq!(finished(&ev).0, Outcome::Failure);
}

/// Stoic applies to the Death Roll (F-46).
#[test]
fn stoic_death_roll() {
    let mut t = Table::new(Mode::Campaign).with(
        "a",
        pc().attr("strength", 1)
            .attr("wits", 4)
            .skill("stamina", 1)
            .talent("stoic"),
    );
    let ev = t.roll(
        RollRequest::new("a", RollKind::Death).talent("stoic"),
        &[1, 2, 3, 4, 6],
    );
    assert_eq!(started(&ev).0, 5);
    assert_eq!(finished(&ev).0, Outcome::Success);
}

/// Attribute-only rolls: no Stress Dice, no push (F-27).
#[test]
fn attribute_roll() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("empathy", 2).stress(4));
    let ev = t.roll(
        RollRequest::new(
            "a",
            RollKind::Attribute {
                attribute: "empathy".into(),
            },
        ),
        &[1, 2],
    );
    assert_eq!(started(&ev).1, 0);
    assert!(options(&ev).is_empty());
}

/// Supply roll: Stress Dice = rating, at most six; facehuggers lower the
/// rating; no panic (F-40) [p. 34].
#[test]
fn supply_roll() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().supply(Supply::Air, 8).stress(4));
    let ev = t.roll(
        RollRequest::new(
            "a",
            RollKind::Supply {
                supply: Supply::Air,
            },
        ),
        &[1, 1, 2, 3, 4, 5],
    );
    assert!(has(&ev, |e| matches!(
        e,
        Event::SupplyRolled { from: 8, to: 6, .. }
    )));
    assert!(!has(&ev, |e| matches!(e, Event::PanicRolled { .. })));
}

/// Calm Breather: two dice fewer, at least one (F-09 default and
/// alternative) [p. 78].
#[test]
fn calm_breather_and_ruling_f09() {
    let air = |t: &mut Table, faces: &[u8]| {
        t.roll(
            RollRequest::new(
                "a",
                RollKind::Supply {
                    supply: Supply::Air,
                },
            ),
            faces,
        )
    };
    let breather = pc().talent("calm_breather");
    let mut t = Table::new(Mode::Campaign).with("a", breather.clone().supply(Supply::Air, 8));
    air(&mut t, &[2, 3, 4, 5]); // min(8, 6) − 2 = 4
    let mut t = Table::new(Mode::Campaign).with("a", breather.clone().supply(Supply::Air, 8));
    t.state
        .rulings
        .insert("F-09".into(), "minus_then_cap".into());
    air(&mut t, &[2, 3, 4, 5, 2, 3]); // min(6, 8 − 2) = 6
    let mut t = Table::new(Mode::Campaign).with("a", breather.supply(Supply::Air, 2));
    air(&mut t, &[2]); // at least one die
    // Light Eater does not touch air.
    let mut t =
        Table::new(Mode::Campaign).with("a", pc().talent("light_eater").supply(Supply::Air, 2));
    air(&mut t, &[2, 3]);
}

#[test]
fn single_stress_dice_never_panic() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().stress(5));
    let ev = t.roll(
        RollRequest::new("a", RollKind::StressDice { count: 1 }),
        &[1],
    );
    assert!(has(&ev, |e| matches!(
        e,
        Event::DiceThrown { facehuggers: 1, .. }
    )));
    assert!(!has(&ev, |e| matches!(e, Event::PanicRolled { .. })));
}

// ---- Story Points [p. 61; F-06, F-21, F-37] -----------------------------------

#[test]
fn story_point_once_per_skill_roll_in_cinematic_play() {
    let mut t = Table::new(Mode::Cinematic).with("a", pc().attr("wits", 2).player("p"));
    t.state.story_points.insert("p".into(), 2);
    let ev = t.roll(RollRequest::skill("a", "observation"), &[2, 3]);
    assert_eq!(
        options(&ev),
        [Choice::Push, Choice::StoryPoint, Choice::Accept]
    );
    let ev = t.decide(&ev, Choice::Push, &[2, 3, 4]);
    assert_eq!(options(&ev), [Choice::StoryPoint, Choice::Accept]);
    let ev = t.decide(&ev, Choice::StoryPoint, &[]);
    assert_eq!(finished(&ev), (Outcome::Success, 1, 0));
    assert_eq!(t.state.story_points["p"], 1);
}

#[test]
fn no_story_points_in_campaign_play_or_on_other_rolls() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("wits", 2).player("p"));
    t.state.story_points.insert("p".into(), 2);
    let ev = t.roll(RollRequest::skill("a", "observation"), &[2, 3]);
    assert_eq!(options(&ev), [Choice::Push, Choice::Accept]);

    let mut t = Table::new(Mode::Cinematic).with("a", pc().attr("strength", 2).player("p"));
    t.state.story_points.insert("p".into(), 2);
    let ev = t.roll(RollRequest::new("a", RollKind::Death), &[2, 3]);
    assert!(options(&ev).is_empty());
}

/// A Story Point cannot undo a cancellation by panic (F-37).
#[test]
fn no_story_point_against_cancelled_action() {
    let mut t = Table::new(Mode::Cinematic).with("a", pc().attr("wits", 1).stress(5).player("p"));
    t.state.story_points.insert("p".into(), 3);
    let ev = t.roll(
        RollRequest::skill("a", "observation"),
        &[2, 1, 2, 3, 4, 5, /* panic */ 6],
    );
    assert!(options(&ev).is_empty());
    assert_eq!(finished(&ev).0, Outcome::Cancelled);
}

// ---- The Long Haul [p. 76; F-38] --------------------------------------------

#[test]
fn the_long_haul_ignores_facehuggers_once_per_act() {
    let mut t = Table::new(Mode::Cinematic).with(
        "r",
        pc().attr("strength", 2).stress(1).talent("the_long_haul"),
    );
    let ev = t.roll(RollRequest::skill("r", "heavy_machinery"), &[2, 3, 1]);
    assert_eq!(options(&ev), [Choice::IgnoreFacehuggers, Choice::Accept]);
    let ev = t.decide(&ev, Choice::IgnoreFacehuggers, &[]);
    assert_eq!(
        options(&ev),
        [Choice::Push, Choice::Accept],
        "push allowed again (F-38)"
    );
    let ev = t.decide(&ev, Choice::Push, &[6, 2, 3, 4]);
    assert_eq!(finished(&ev).0, Outcome::Success);

    // Used up for this Act: the next facehugger means panic.
    let ev = t.roll(
        RollRequest::skill("r", "heavy_machinery"),
        &[2, 3, 3, 1, /* panic */ 1],
    );
    assert!(has(&ev, |e| matches!(e, Event::PanicRolled { .. })));

    t.run(Command::EndAct, &[]);
    let ev = t.roll(RollRequest::skill("r", "heavy_machinery"), &[2, 3, 3, 1]);
    assert_eq!(options(&ev), [Choice::IgnoreFacehuggers, Choice::Accept]);
}

// ---- stopping panic with COMMAND [p. 71, 104; F-36] ---------------------------

#[test]
fn command_roll_stops_someone_elses_panic() {
    let panicking = ActivePanic {
        effect: "tremble".into(),
        value: 8,
        effects_end: Some(Ending::Never),
        started_broken: false,
    };
    let mut medic = pc().attr("empathy", 3).skill("command", 1);
    let mut victim = pc();
    victim.panic = Some(panicking);
    medic.panic = None;
    let mut t = Table::new(Mode::Campaign)
        .with("medic", medic)
        .with("victim", victim);

    let own = t.try_run(
        Command::Roll {
            request: RollRequest::skill("victim", "command").purpose(Purpose::StopPanic {
                target: "victim".into(),
            }),
        },
        &[],
    );
    assert!(
        matches!(own, Err(RuleError::NotAllowed(_))),
        "not on yourself"
    );

    let request = RollRequest::skill("medic", "command").purpose(Purpose::StopPanic {
        target: "victim".into(),
    });
    let ev = t.roll(request, &[2, 3, 4, 6]);
    let _ = t.decide(&ev, Choice::Accept, &[]);
    assert!(t.actor("victim").panic.is_none());
}

#[test]
fn modifiers_from_effects_name_their_source() {
    let mut victim = pc().attr("agility", 3);
    victim.panic = Some(ActivePanic {
        effect: "tremble".into(),
        value: 8,
        effects_end: Some(Ending::Never),
        started_broken: false,
    });
    let mut t = Table::new(Mode::Campaign).with("v", victim);
    let ev = t.roll(RollRequest::skill("v", "mobility"), &[2]);
    let (base, _, _, mods) = started(&ev);
    assert_eq!(base, 1);
    assert!(
        mods.iter()
            .any(|m| m.source == "panic/tremble" && m.value == -2)
    );
    let _ = Unit::Round;
}

/// After The Long Haul, new facehuggers in the push count (F-47 default);
/// with the alternative the talent covers the push too.
#[test]
fn the_long_haul_and_the_push_ruling_f47() {
    for (option, panics) in [("this_throw", true), ("including_push", false)] {
        let mut t = Table::new(Mode::Cinematic).with(
            "r",
            pc().attr("strength", 2).stress(1).talent("the_long_haul"),
        );
        t.state.rulings.insert("F-47".into(), option.into());
        let ev = t.roll(RollRequest::skill("r", "heavy_machinery"), &[2, 3, 1]);
        let ev = t.decide(&ev, Choice::IgnoreFacehuggers, &[]);
        let faces: &[u8] = if panics {
            &[2, 3, 1, 4, /* panic */ 1]
        } else {
            &[2, 3, 1, 4]
        };
        let ev = t.decide(&ev, Choice::Push, faces);
        assert_eq!(
            has(&ev, |e| matches!(e, Event::PanicRolled { .. })),
            panics,
            "{option}"
        );
    }
}

/// An NPC defender rolls its skill and never pushes; the PC attacker may.
#[test]
fn npc_defender_and_pc_attacker() {
    let mut t = Table::new(Mode::Campaign)
        .with("pc", pc().attr("agility", 2))
        .with("guard", npc().attr("wits", 2));
    let ev = t.run(
        Command::Opposed {
            attacker: RollRequest::skill("pc", "mobility"),
            defender: RollRequest::skill("guard", "observation"),
        },
        &[2, 3, /* attacker */ 6, 2],
    );
    let decisions: Vec<_> = ev
        .iter()
        .filter(|e| matches!(e, Event::Decision { .. }))
        .collect();
    assert_eq!(decisions.len(), 1, "only the attacker decides");
    assert_eq!(options(&ev), [Choice::Push, Choice::Accept]);
    let ev = t.decide(&ev, Choice::Accept, &[]);
    assert_eq!(finished(&ev).0, Outcome::Success);
}

/// A PC defender is not offered a push [p. 64].
#[test]
fn pc_defender_cannot_push() {
    let mut t = Table::new(Mode::Campaign)
        .with("pc", pc().attr("agility", 2))
        .with("npc", npc().attr("agility", 1));
    let ev = t.run(
        Command::Opposed {
            attacker: RollRequest::skill("npc", "mobility"),
            defender: RollRequest::skill("pc", "mobility"),
        },
        &[2, 3, /* attacker */ 6],
    );
    assert!(options(&ev).is_empty());
    assert_eq!(finished(&ev).0, Outcome::Success);
}

/// While the defender decides, the attacker's roll cannot be touched, and
/// only offered choices are accepted.
#[test]
fn opposed_roll_cannot_be_derailed() {
    let mut t = Table::new(Mode::Campaign)
        .with(
            "r",
            pc().attr("strength", 2).stress(1).talent("the_long_haul"),
        )
        .with("npc", npc().attr("strength", 2));
    let ev = t.run(
        Command::Opposed {
            attacker: RollRequest::skill("npc", "close_combat"),
            defender: RollRequest::skill("r", "close_combat"),
        },
        &[2, 3, 1],
    );
    assert_eq!(
        t.state.pending.len(),
        1,
        "only the defender's decision is open"
    );
    let wrong = t.try_decide(&ev, Choice::Push, &[]);
    assert!(matches!(wrong, Err(RuleError::InvalidChoice(_))));
    let ev = t.decide(&ev, Choice::IgnoreFacehuggers, &[/* attacker */ 6, 6]);
    assert_eq!(finished(&ev).0, Outcome::Success);
    assert!(t.state.pending.is_empty());
}

#[test]
fn story_point_cannot_be_spent_twice() {
    let mut t = Table::new(Mode::Cinematic).with("a", pc().attr("wits", 2).player("p"));
    t.state.story_points.insert("p".into(), 3);
    let ev = t.roll(RollRequest::skill("a", "observation"), &[2, 3]);
    let _ = t.decide(&ev, Choice::StoryPoint, &[]);
    let again = t.try_decide(&ev, Choice::StoryPoint, &[]);
    assert!(matches!(again, Err(RuleError::UnknownPending(_))));
    assert_eq!(t.state.story_points["p"], 2);
}

#[test]
fn no_story_points_on_fixed_pools_or_supply_rolls() {
    let mut t = Table::new(Mode::Cinematic).with("a", pc().player("p").supply(Supply::Air, 2));
    t.state.story_points.insert("p".into(), 3);
    let mut armor = RollRequest::pool(2);
    armor.actor = Some("a".into());
    let ev = t.roll(armor, &[2, 3]);
    assert!(options(&ev).is_empty());
    let ev = t.roll(
        RollRequest::new(
            "a",
            RollKind::Supply {
                supply: Supply::Air,
            },
        ),
        &[2, 3],
    );
    assert!(options(&ev).is_empty());
}

/// A Broken character cannot roll for skills; the Death Roll stays
/// allowed [p. 98–99].
#[test]
fn broken_characters_cannot_roll_skills() {
    let mut t = Table::new(Mode::Campaign).with("a", pc().attr("wits", 2).health(0));
    let err = t.try_run(
        Command::Roll {
            request: RollRequest::skill("a", "observation"),
        },
        &[],
    );
    assert!(matches!(err, Err(RuleError::NotAllowed(_))));
    t.roll(RollRequest::new("a", RollKind::Death), &[1, 2, 3, 4]);
}

/// The preview builds the pool without dice and names fitting talents.
#[test]
fn preview_without_dice() {
    let t = Table::new(Mode::Campaign).with(
        "a",
        pc().attr("wits", 3)
            .attr("empathy", 1)
            .stress(2)
            .talent("take_control"),
    );
    let preview = t
        .engine
        .preview(&t.state, RollRequest::skill("a", "manipulation").helpers(1))
        .unwrap();
    assert_eq!((preview.base_dice, preview.stress_dice), (2, 2));
    assert_eq!(preview.talents, ["take_control"]);
    let preview = t
        .engine
        .preview(
            &t.state,
            RollRequest::skill("a", "manipulation").talent("take_control"),
        )
        .unwrap();
    assert_eq!(preview.base_dice, 3);
    assert!(t.state.pending.is_empty());
}

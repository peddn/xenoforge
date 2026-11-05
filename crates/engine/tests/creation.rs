//! Creating a Campaign character [p. 27–36]: the book example of Nat
//! [p. 28–29], the limits, androids (F-50), Tough and the starting cash.

mod common;

use std::collections::BTreeMap;

use common::*;
use xenoforge_engine::creation::{CreationStep, Draft, GearChoice, GearRoll};
use xenoforge_engine::dice::ScriptedDice;
use xenoforge_engine::state::Place;

fn map(pairs: &[(&str, u8)]) -> BTreeMap<String, u8> {
    pairs.iter().map(|(k, v)| ((*k).to_owned(), *v)).collect()
}

/// Nat the Roughneck as the book builds her [p. 28–29].
fn nat() -> Draft {
    Draft {
        career: Some("roughneck".into()),
        attributes: map(&[("strength", 5), ("agility", 3), ("wits", 2), ("empathy", 4)]),
        skills: map(&[
            ("heavy_machinery", 3),
            ("stamina", 2),
            ("close_combat", 2),
            ("comtech", 1),
            ("observation", 1),
            ("medical_aid", 1),
        ]),
        talent: Some("true_grit".into()),
        name: "Nat".into(),
        appearance: "Wild hair".into(),
        agenda: "Never let your friends down.".into(),
        // A cutting torch and a compression suit [p. 33].
        gear: vec![gear(0, 0), gear(2, 1)],
        signature_item: "Tool belt".into(),
        ..Draft::default()
    }
}

fn gear(line: u8, option: u8) -> GearChoice {
    GearChoice { line, option }
}

fn codes(draft: &Draft) -> Vec<String> {
    Table::new(xenoforge_engine::state::Mode::Campaign)
        .engine
        .check_creation(draft)
        .into_iter()
        .map(|p| p.code)
        .collect()
}

/// p. 28: Nat's skills come to exactly 10 points; she is complete.
#[test]
fn book_p28_nat_is_complete() {
    let t = Table::new(xenoforge_engine::state::Mode::Campaign);
    assert_eq!(t.engine.check_creation(&nat()), vec![]);
    let actor = t.engine.creation_actor(&nat());
    assert_eq!(actor.attribute("strength"), 5);
    assert_eq!(actor.skill_level("heavy_machinery"), 3);
    assert!(actor.talents.contains("true_grit"));
    let mut named = nat();
    named.talent_param = Some("Cheyenne".into());
    assert!(
        t.engine.creation_actor(&named).talent_params.is_empty(),
        "True Grit names nothing"
    );
    assert_eq!(
        (actor.health, actor.stress),
        (5, 0),
        "Health = STRENGTH [p. 30]"
    );
}

/// 14 attribute points, 2 to 4, the key attribute up to 5; all spent
/// (F-51) [p. 28].
#[test]
fn attribute_limits() {
    let mut d = nat();
    d.attributes.insert("wits".into(), 1);
    assert_eq!(codes(&d), ["attribute_range", "attribute_points"]);

    let mut d = nat();
    d.attributes = map(&[("strength", 4), ("agility", 5), ("wits", 2), ("empathy", 3)]);
    let problems = Table::new(xenoforge_engine::state::Mode::Campaign)
        .engine
        .check_creation(&d);
    assert_eq!(problems.len(), 1, "only AGILITY is above 4: {problems:?}");
    assert_eq!(problems[0].id.as_deref(), Some("agility"));
    assert_eq!(problems[0].step, CreationStep::Attributes);
}

/// 10 skill points; key skills up to 3, the others up to 1; all spent
/// (F-51) [p. 28].
#[test]
fn skill_limits() {
    let mut d = nat();
    d.skills.insert("comtech".into(), 2);
    d.skills.insert("observation".into(), 0);
    assert_eq!(codes(&d), ["skill_range"]);

    let mut d = nat();
    d.skills.insert("medical_aid".into(), 0);
    assert_eq!(codes(&d), ["skill_points"]);
}

/// One of the career's three talents [p. 29, 74]; a talent that names a
/// vehicle needs it [p. 76].
#[test]
fn career_talent() {
    let mut d = nat();
    d.talent = Some("cunning".into());
    assert_eq!(codes(&d), ["talent_not_career"]);

    let mut d = nat();
    d.career = Some("pilot".into());
    d.skills = map(&[
        ("piloting", 3),
        ("ranged_combat", 2),
        ("comtech", 2),
        ("mobility", 1),
        ("observation", 1),
        ("heavy_machinery", 1),
    ]);
    d.attributes = map(&[("strength", 3), ("agility", 5), ("wits", 3), ("empathy", 3)]);
    d.talent = Some("like_the_back_of_your_hand".into());
    assert_eq!(codes(&d), ["talent_parameter"]);
    d.talent_param = Some("Cheyenne 2".into());
    assert_eq!(codes(&d), Vec::<String>::new());
}

/// Androids: +3 on two different attributes (F-50), no signature item
/// needed [p. 29]; Health follows STRENGTH with the bonus.
#[test]
fn android() {
    let t = Table::new(xenoforge_engine::state::Mode::Campaign);
    let mut d = nat();
    d.android = true;
    d.signature_item.clear();
    d.android_bonus = vec!["strength".into(), "strength".into()];
    assert_eq!(codes(&d), ["android_bonus"]);
    d.android_bonus = vec!["strength".into(), "wits".into()];
    assert_eq!(t.engine.check_creation(&d), vec![]);
    let actor = t.engine.creation_actor(&d);
    assert!(!actor.covert);
    d.android_covert = true;
    assert!(t.engine.creation_actor(&d).covert, "poses as human (A-14)");
    assert!(
        !t.engine
            .creation_actor(&Draft {
                android_covert: true,
                ..nat()
            })
            .covert,
        "only androids are covert"
    );
    assert_eq!(actor.attribute("strength"), 8);
    assert_eq!(actor.attribute("wits"), 5);
    assert_eq!(actor.health, 8);
}

/// Tough: Health +2 [p. 79].
#[test]
fn tough_adds_two_health() {
    let t = Table::new(xenoforge_engine::state::Mode::Campaign);
    let actor = pc().attr("strength", 3).talent("tough");
    assert_eq!(t.engine.full_health(&actor), 5);
    assert_eq!(t.engine.full_health(&pc().attr("strength", 3)), 3);
}

/// Name, appearance, agenda and signature item are steps of their own
/// [p. 30–33].
#[test]
fn texts_are_steps() {
    let mut d = nat();
    d.name = " ".into();
    d.agenda.clear();
    d.signature_item.clear();
    assert_eq!(
        codes(&d),
        ["name_missing", "agenda_missing", "signature_item_missing"]
    );
}

/// Starting cash: Kid $D6, Company Agent $2D6×100, others $D6×100
/// [p. 39–55].
#[test]
fn starting_cash() {
    let t = Table::new(xenoforge_engine::state::Mode::Campaign);
    let roll = |career: &str, faces: &[u8]| {
        let mut dice = ScriptedDice::new(faces.iter().copied());
        t.engine.roll_cash(career, &mut dice).unwrap().amount
    };
    assert_eq!(roll("kid", &[4]), 4);
    assert_eq!(roll("roughneck", &[4]), 400);
    assert_eq!(roll("company_agent", &[4, 5]), 900);
}

/// p. 33: Nat starts with a cutting torch and a compression suit, carried;
/// the torch has a power supply and is full, so it gets no reloads (F-61).
#[test]
fn book_p33_nat_starting_gear() {
    let t = Table::new(xenoforge_engine::state::Mode::Campaign);
    let actor = t.engine.creation_actor(&nat());
    let items: Vec<_> = actor
        .items
        .iter()
        .map(|p| (p.item.as_deref().unwrap_or(""), p.count, p.place, p.reloads))
        .collect();
    assert_eq!(
        items,
        [
            ("cutting_torch", 1, Place::Carried, 0),
            ("irc_mk50_compression_suit", 1, Place::Carried, 0),
        ]
    );
    let torch = &actor.items[0];
    assert_eq!(torch.supply, Some(5), "full Power Supply 5 [p. 135]");
    assert_ne!(actor.items[0].uid, actor.items[1].uid);
}

/// Two items from two different lines of the career (F-60).
#[test]
fn starting_gear_rules() {
    let mut d = nat();
    d.gear = vec![gear(0, 0)];
    assert_eq!(codes(&d), ["gear_count"]);
    d.gear = vec![gear(0, 0), gear(0, 1)];
    assert_eq!(codes(&d), ["gear_line"]);
    d.gear = vec![gear(0, 0), gear(4, 0)];
    assert_eq!(codes(&d), ["gear_unknown"]);
    d.gear = vec![gear(0, 0), gear(1, 2)];
    assert_eq!(codes(&d), ["gear_unknown"]);
    let t = Table::new(xenoforge_engine::state::Mode::Campaign);
    let problem = &t.engine.check_creation(&d)[0];
    assert_eq!(
        (problem.step, problem.value, problem.limit),
        (CreationStep::Gear, Some(2), Some(4))
    );
}

/// A firearm comes with two full reloads (F-61) [p. 33].
#[test]
fn starting_firearm_has_two_reloads() {
    let t = Table::new(xenoforge_engine::state::Mode::Campaign);
    let mut d = nat();
    d.career = Some("officer".into());
    d.gear = vec![gear(0, 0), gear(1, 1)];
    let actor = t.engine.creation_actor(&d);
    assert_eq!(actor.items[0].item.as_deref(), Some("m4a3_service_pistol"));
    assert_eq!(actor.items[0].reloads, 2);
    assert_eq!(actor.items[1].item.as_deref(), Some("binoculars"));
    assert_eq!(actor.items[1].reloads, 0);
}

/// "D6 doses" are rolled when finishing, not before (F-60) [p. 47].
#[test]
fn dice_quantities_are_rolled_at_the_end() {
    let t = Table::new(xenoforge_engine::state::Mode::Campaign);
    let mut d = nat();
    d.career = Some("medic".into());
    d.gear = vec![gear(1, 0), gear(3, 1)];
    let mut actor = t.engine.creation_actor(&d);
    assert_eq!(actor.items.len(), 1, "only the hand radio so far");
    let mut dice = ScriptedDice::new([4]);
    let rolls = t.engine.roll_start_gear(&mut actor, &d, &mut dice);
    assert_eq!(
        rolls,
        [GearRoll {
            item: "naproleve".into(),
            dice: vec![4]
        }]
    );
    let doses = actor
        .items
        .iter()
        .find(|p| p.item.as_deref() == Some("naproleve"));
    assert_eq!(doses.map(|p| p.count), Some(4));
}

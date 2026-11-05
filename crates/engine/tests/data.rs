//! Checks on the rule data in `data/`.
//!
//! Set `XENOFORGE_UPDATE_SCHEMAS=1` to rewrite the JSON Schemas in
//! `data/schema` after changing the Rust types.

use std::collections::BTreeSet;
use std::path::PathBuf;

use xenoforge_engine::data::effects::{Effect, Op};
use xenoforge_engine::data::{Data, Problem, RULE_FILES, schemas, validate};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load() -> Data {
    Data::load(&root().join("data")).unwrap_or_else(|e| panic!("{e}"))
}

fn report(problems: &[Problem]) -> String {
    problems.iter().map(|p| format!("\n  {p}")).collect()
}

#[test]
fn data_is_valid() {
    let problems = validate(&load());
    assert!(
        problems.is_empty(),
        "problems in data/:{}",
        report(&problems)
    );
}

#[test]
fn schemas_are_up_to_date() {
    let dir = root().join("data/schema");
    let update = std::env::var_os("XENOFORGE_UPDATE_SCHEMAS").is_some();
    let mut stale = Vec::new();
    for (name, schema) in schemas() {
        let path = dir.join(format!("{name}.schema.json"));
        let expected = serde_json::to_string_pretty(&schema).unwrap() + "\n";
        if update {
            std::fs::write(&path, &expected).unwrap();
        } else if std::fs::read_to_string(&path).ok().as_deref() != Some(expected.as_str()) {
            stale.push(path.display().to_string());
        }
    }
    assert!(
        stale.is_empty(),
        "outdated schemas (run with XENOFORGE_UPDATE_SCHEMAS=1): {stale:?}"
    );

    let mut files: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    files.sort();
    let mut expected: Vec<String> = RULE_FILES
        .iter()
        .map(|f| format!("{f}.schema.json"))
        .collect();
    expected.sort();
    assert_eq!(files, expected, "data/schema has extra or missing files");
}

#[test]
fn every_rule_file_has_a_schema_and_vice_versa() {
    let mut files: Vec<String> = std::fs::read_dir(root().join("data/rules"))
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    files.sort();
    let mut expected: Vec<String> = RULE_FILES.iter().map(|f| format!("{f}.json")).collect();
    expected.sort();
    assert_eq!(files, expected, "data/rules has extra or missing files");
}

/// The probe data must use every kind of effect, so that the format is
/// tested as a whole (plan section 4.4).
#[test]
fn every_effect_kind_is_used() {
    let data = load();
    let mut used = BTreeSet::new();
    data.rules.for_each_entry(|_, entry| {
        for effect in entry.effects() {
            used.insert(effect.kind());
        }
    });
    let all: BTreeSet<&str> = Effect::KINDS.into_iter().collect();
    let missing: Vec<_> = all.difference(&used).collect();
    assert!(missing.is_empty(), "effect kinds without data: {missing:?}");
}

/// One row of the rulings table in the catalog.
struct CatalogRow {
    id: String,
    pages: Vec<u16>,
    has_alternative: bool,
}

/// Reads the `| F-NN | … |` rows of `docs/rules/00-errata-und-festlegungen.md`.
fn catalog() -> Vec<CatalogRow> {
    let text =
        std::fs::read_to_string(root().join("docs/rules/00-errata-und-festlegungen.md")).unwrap();
    text.lines()
        .filter(|line| line.starts_with("| F-"))
        .map(|line| {
            let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
            assert_eq!(cells.len(), 5, "unexpected row: {line}");
            let place = cells[1];
            let pages = place
                .split_once("S. ")
                .map(|(_, rest)| {
                    rest.split(|c: char| !c.is_ascii_digit())
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse().unwrap())
                        .collect()
                })
                .unwrap_or_default();
            CatalogRow {
                id: cells[0].to_owned(),
                pages,
                has_alternative: cells[4] != "–",
            }
        })
        .collect()
}

#[test]
fn rulings_match_the_catalog() {
    let data = load();
    let rows = catalog();
    let in_docs: Vec<&str> = rows.iter().map(|r| r.id.as_str()).collect();
    let in_data: Vec<&str> = data.rules.rulings.iter().map(|r| r.id.as_str()).collect();
    assert_eq!(in_data, in_docs, "rulings.json and the catalog differ");

    for (row, ruling) in rows.iter().zip(&data.rules.rulings) {
        assert_eq!(
            ruling.pages, row.pages,
            "{}: pages differ from the catalog",
            row.id
        );
        let expected = if row.has_alternative { 2 } else { 1 };
        assert_eq!(
            ruling.options.len(),
            expected,
            "{}: the catalog has {} alternative",
            row.id,
            if row.has_alternative { "an" } else { "no" }
        );
    }
}

// ---- the validation itself finds problems ----------------------------------

fn problems_after(change: impl FnOnce(&mut Data)) -> Vec<String> {
    let mut data = load();
    change(&mut data);
    validate(&data).iter().map(ToString::to_string).collect()
}

fn assert_found(problems: &[String], needle: &str) {
    assert!(
        problems.iter().any(|p| p.contains(needle)),
        "expected a problem containing {needle:?}, got {problems:#?}"
    );
}

#[test]
fn finds_duplicate_ids() {
    let problems = problems_after(|d| d.rules.skills[1].id = "heavy_machinery".into());
    assert_found(
        &problems,
        "skills/heavy_machinery: id is also used in skills",
    );
}

#[test]
fn finds_unknown_references() {
    let problems = problems_after(|d| d.rules.skills[0].attribute = "luck".into());
    assert_found(&problems, "unknown attribute \"luck\"");

    let problems = problems_after(|d| d.rules.ship_damage[0].system = "warp_core".into());
    assert_found(&problems, "unknown ship system \"warp_core\"");
}

#[test]
fn finds_unknown_references_inside_effects() {
    let problems = problems_after(|d| {
        let injury = &mut d.rules.critical_injuries[0];
        if let Effect::Restriction { blocks, .. } = &mut injury.effects[1] {
            *blocks = xenoforge_engine::data::effects::Blocked::Action("teleport".into());
        }
    });
    assert_found(&problems, "unknown action \"teleport\"");
}

#[test]
fn finds_gaps_in_the_panic_table() {
    let problems = problems_after(|d| d.rules.panic.retain(|p| p.id != "tremble"));
    assert_found(&problems, "expected the range to start at 8");
}

#[test]
fn finds_overlapping_damage_ranges() {
    let problems = problems_after(|d| d.rules.ship_damage[1].roll.from = 16);
    assert_found(
        &problems,
        "roll range overlaps artificial_gravity_malfunction",
    );
}

#[test]
fn finds_missing_and_unused_translations() {
    let problems = problems_after(|d| {
        let en = d.i18n.get_mut("en").unwrap();
        en.remove("run.name");
        en.insert("nobody.name".into(), "Nobody".into());
    });
    assert_found(&problems, "missing translation \"run.name\" in en");
    assert_found(&problems, "key \"nobody.name\" is not in every language");
    assert_found(&problems, "key \"nobody.name\" is not used by any entry");
}

#[test]
fn finds_wrong_ruling_alternatives() {
    let problems = problems_after(|d| {
        let switch = d.rules.ship_damage[2].ruling.as_mut().unwrap();
        let effects = switch.alternatives.remove("halve").unwrap();
        switch.alternatives.insert("triple".into(), effects);
    });
    assert_found(&problems, "alternatives must be");
}

#[test]
fn finds_unknown_special_handlers() {
    let problems = problems_after(|d| {
        d.rules.panic[0].on_apply.push(Op::Special {
            handler: "make_coffee".into(),
        })
    });
    assert_found(&problems, "unknown special handler \"make_coffee\"");
}

#[test]
fn unknown_fields_are_rejected() {
    let ok = r#"{ "kind": "hint", "text": "x.hint", "tier": "info" }"#;
    assert!(serde_json::from_str::<Effect>(ok).is_ok());
    let extra = r#"{ "kind": "hint", "text": "x.hint", "tier": "info", "colour": "red" }"#;
    assert!(serde_json::from_str::<Effect>(extra).is_err());
    let extra_op = r#"{ "op": "stress", "delta": 1, "reason": "x" }"#;
    assert!(serde_json::from_str::<Op>(extra_op).is_err());
}

#[test]
fn finds_replacements_that_do_not_fit() {
    use xenoforge_engine::data::effects::{ReplaceWith, Replaceable};
    let problems = problems_after(|d| {
        let talent = d
            .rules
            .talents
            .iter_mut()
            .find(|t| t.id == "stoic")
            .unwrap();
        if let Effect::Replacement { with, .. } = &mut talent.effects[0] {
            *with = ReplaceWith::Success;
        }
    });
    assert_found(&problems, "replacement and its result do not fit together");

    let problems = problems_after(|d| {
        let talent = d
            .rules
            .talents
            .iter_mut()
            .find(|t| t.id == "overkill")
            .unwrap();
        if let Effect::Replacement { replaces, with, .. } = &mut talent.effects[0] {
            *replaces = Replaceable::PanicResult { from: 11 };
            *with = ReplaceWith::PanicEffect("flee".into());
        }
    });
    assert_found(&problems, "\"flee\" is not a panic entry outside the table");
    assert_found(
        &problems,
        "panic/overkill_effect: entry outside the table is never used",
    );
}

#[test]
fn finds_pushes_on_rolls_that_cannot_be_pushed() {
    use xenoforge_engine::data::effects::{Modified, RollKind};
    let problems = problems_after(|d| {
        let talent = d
            .rules
            .talents
            .iter_mut()
            .find(|t| t.id == "nerves_of_steel")
            .unwrap();
        if let Effect::Modifier {
            modifies,
            applies_to,
            ..
        } = &mut talent.effects[0]
        {
            *modifies = Modified::Pushes;
            applies_to.rolls = vec![RollKind::Panic];
        }
    });
    assert_found(&problems, "only skill rolls can be pushed");
}

// ---- errata and rulings that live in the data -----------------------------

fn item(data: &Data, id: &str) -> serde_json::Value {
    let item = data
        .rules
        .gear
        .iter()
        .find(|i| i.id == id)
        .unwrap_or_else(|| panic!("no item {id}"));
    serde_json::to_value(item).unwrap()
}

/// The Pilot's key skills as corrected (E-01) [p. 51].
#[test]
fn pilot_key_skills_follow_the_errata() {
    let data = load();
    let pilot = data.rules.careers.iter().find(|c| c.id == "pilot").unwrap();
    assert_eq!(pilot.key_skills, ["piloting", "ranged_combat", "comtech"]);
}

/// The weapon tables count, not the infographics (E-03) [p. 119–124].
#[test]
fn weapons_follow_the_tables() {
    let data = load();
    for (id, bonus, damage, range) in [
        ("m4a3_service_pistol", 2, 1, "medium"),
        ("armat_m41a_pulse_rifle", 1, 2, "long"),
        ("m42a_scope_rifle", 2, 2, "extreme"),
        ("armat_m41ae2_heavy_pulse_rifle", 1, 3, "extreme"),
    ] {
        let weapon = &item(&data, id)["weapon"];
        assert_eq!(
            (&weapon["bonus"], &weapon["damage"], &weapon["range"]),
            (
                &serde_json::json!(bonus),
                &serde_json::json!(damage),
                &serde_json::json!(range)
            ),
            "{id}"
        );
    }
}

/// Career items without stats: a weight by the rule of thumb, no price
/// (F-64) [p. 34, 39–55].
#[test]
fn career_items_have_a_weight_and_no_price() {
    let data = load();
    let others: Vec<_> = data
        .rules
        .gear
        .iter()
        .filter(|i| serde_json::to_value(i.category).unwrap() == "other")
        .collect();
    assert!(!others.is_empty());
    for item in others {
        assert!(item.weight.is_some(), "{}", item.id);
        assert!(item.cost.is_none(), "{}", item.id);
    }
}

/// The HoloTab helps COMMAND and OBSERVATION, as the text says (F-65)
/// [p. 131, 132].
#[test]
fn holotab_helps_command_and_observation() {
    let data = load();
    let holotab = item(&data, "holotab");
    assert_eq!(
        holotab["effects"][0]["applies_to"]["skills"],
        serde_json::json!(["command", "observation"])
    );
    assert_eq!(holotab["effects"][0]["change"]["add"], 2);
}

/// The surgical kit as a weapon: close combat, bonus 0, damage 2, Engaged
/// (F-67) [p. 126, 136].
#[test]
fn surgical_kit_is_a_close_combat_weapon() {
    let data = load();
    let weapon = &item(&data, "surgical_kit")["weapon"];
    assert_eq!(weapon["skill"], "close_combat");
    assert!(weapon["bonus"].is_null() || weapon["bonus"] == 0);
    assert_eq!(weapon["damage"], 2);
    assert_eq!(weapon["range"], "engaged");
}

/// Phobia: Stress near the object, a Panic Roll after more than one Round
/// near it (F-41) [p. 101]; the hint says so in both languages.
#[test]
fn phobia_counts_one_round() {
    let data = load();
    for (lang, word) in [("de", "eine Runde"), ("en", "one Round")] {
        let text = &data.i18n[lang]["phobia.hint"];
        assert!(text.contains(word), "{lang}: {text}");
    }
}

/// Every career suggests names, looks, signature items and Agendas, with
/// texts in both languages (A-11) [p. 39–55].
#[test]
fn careers_make_suggestions() {
    let data = load();
    for career in &data.rules.careers {
        assert!(!career.names.is_empty(), "{}", career.id);
        for (kind, ids) in [
            ("look", &career.looks),
            ("item", &career.items),
            ("agenda", &career.agendas),
        ] {
            assert!(!ids.is_empty(), "{} {kind}", career.id);
            for id in ids {
                for lang in ["de", "en"] {
                    let key = format!("{}.{kind}.{id}", career.id);
                    assert!(data.i18n[lang].contains_key(&key), "{lang}: {key}");
                }
            }
        }
    }
}

/// A talent is marked as without effect for androids exactly when all it
/// does is about pushes, panic or facehuggers on Stress Dice, which
/// androids do not have (plan 5.3) [p. 29, 111].
#[test]
fn talents_without_effect_for_androids() {
    let only_human = |effect: &serde_json::Value| match effect["kind"].as_str() {
        Some("modifier") => {
            effect["modifies"] == "pushes"
                || effect["applies_to"]["rolls"]
                    .as_array()
                    .is_some_and(|r| r.iter().any(|x| x == "panic"))
        }
        Some("replacement") => {
            effect["replaces"] == "facehuggers_in_roll"
                || effect["replaces"].get("panic_result").is_some()
        }
        _ => false,
    };
    for talent in &load().rules.talents {
        let effects = serde_json::to_value(&talent.effects).unwrap();
        let effects = effects.as_array().unwrap();
        let expected = !effects.is_empty() && effects.iter().all(only_human);
        assert_eq!(talent.no_effect_for_androids, expected, "{}", talent.id);
    }
}

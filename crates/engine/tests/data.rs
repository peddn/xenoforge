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

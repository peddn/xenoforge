//! Checks that go beyond the JSON Schema: unique ids, references between
//! files, translations and the shape of the roll tables.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::effects::{
    Blocked, Change, Effect, Lasts, ModTarget, Modified, Op, ReplaceWith, Replaceable, RollSpec,
    TriggerEvent,
};
use super::model::{DamageTable, RollRange, Ruling, RulingSwitch};
use super::{Data, Entry, LANGUAGES, Page};

/// Highest page number of the core rulebook.
pub const MAX_PAGE: Page = 400;

/// Constants the engine and the server read from `constants.json`.
pub const REQUIRED_CONSTANTS: &[&str] = &[
    "android_attribute_bonus",
    "android_reveal_stress",
    "android_bonus_attributes",
    "base_pushes",
    "cinematic_acts",
    "creation_attribute_max",
    "creation_attribute_min",
    "creation_attribute_points",
    "creation_key_attribute_max",
    "creation_key_skill_max",
    "creation_other_skill_max",
    "creation_skill_points",
    "gear_bonus_max",
    "help_max",
    "panic_effect_from",
    "shifts_per_day",
    "story_points_max",
    "supply_dice_max",
];

/// Names of the engine functions that `special` operations may call.
pub const SPECIAL_HANDLERS: &[&str] = &["pull_rank"];

/// Text keys the engine uses in its own code, e.g. for a hint it emits.
pub const ENGINE_TEXT_KEYS: &[&str] = &["pull_rank.android"];

/// A problem found by [`validate`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    /// `<file>/<id>`, `i18n/<lang>` or similar.
    pub location: String,
    pub message: String,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.location, self.message)
    }
}

/// Runs all checks and returns every problem found (empty if the data is
/// fine).
pub fn validate(data: &Data) -> Vec<Problem> {
    let mut checker = Checker::new(data);
    checker.entries();
    checker.files();
    checker.translations();
    checker.problems
}

fn is_id(id: &str) -> bool {
    let mut chars = id.chars();
    chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn is_ruling_id(id: &str) -> bool {
    let bytes = id.as_bytes();
    bytes.len() == 4 && id.starts_with("F-") && bytes[2..].iter().all(u8::is_ascii_digit)
}

fn is_d66(value: u8) -> bool {
    (1..=6).contains(&(value / 10)) && (1..=6).contains(&(value % 10))
}

struct Checker<'a> {
    data: &'a Data,
    problems: Vec<Problem>,
    /// Translation keys that the data refers to.
    used_keys: BTreeSet<String>,
    attributes: BTreeSet<&'a str>,
    careers: BTreeSet<&'a str>,
    skills: BTreeSet<&'a str>,
    actions: BTreeSet<&'a str>,
    conditions: BTreeSet<&'a str>,
    systems: BTreeSet<&'a str>,
    rulings: BTreeMap<&'a str, &'a Ruling>,
    /// Panic entries outside the table, which only a replacement reaches.
    off_table_panic: BTreeSet<&'a str>,
    /// Panic entries a replacement refers to.
    used_panic_effects: BTreeSet<String>,
}

impl<'a> Checker<'a> {
    fn new(data: &'a Data) -> Self {
        let r = &data.rules;
        Self {
            data,
            problems: Vec::new(),
            used_keys: BTreeSet::new(),
            attributes: r.attributes.iter().map(|e| e.id.as_str()).collect(),
            careers: r.careers.iter().map(|e| e.id.as_str()).collect(),
            skills: r.skills.iter().map(|e| e.id.as_str()).collect(),
            actions: r.actions.iter().map(|e| e.id.as_str()).collect(),
            conditions: r.conditions.iter().map(|e| e.id.as_str()).collect(),
            systems: r.ship_systems.iter().map(|e| e.id.as_str()).collect(),
            rulings: r.rulings.iter().map(|e| (e.id.as_str(), e)).collect(),
            off_table_panic: r
                .panic
                .iter()
                .filter(|e| e.roll.is_none())
                .map(|e| e.id.as_str())
                .collect(),
            used_panic_effects: BTreeSet::new(),
        }
    }

    fn problem(&mut self, location: impl Into<String>, message: impl Into<String>) {
        self.problems.push(Problem {
            location: location.into(),
            message: message.into(),
        });
    }

    fn reference(&mut self, at: &str, kind: &str, id: &str) {
        let known = match kind {
            "attribute" => self.attributes.contains(id),
            "career" => self.careers.contains(id),
            "skill" => self.skills.contains(id),
            "action" => self.actions.contains(id),
            "condition" => self.conditions.contains(id),
            "ship system" => self.systems.contains(id),
            "ruling" => self.rulings.contains_key(id),
            _ => unreachable!("unknown reference kind {kind}"),
        };
        if !known {
            self.problem(at, format!("unknown {kind} \"{id}\""));
        }
    }

    /// A translation key used by an entry. It must start with the entry id
    /// and exist in every language.
    fn text_key(&mut self, at: &str, owner: &str, key: &str) {
        if !key.starts_with(&format!("{owner}.")) {
            self.problem(
                at,
                format!("text key \"{key}\" must start with \"{owner}.\""),
            );
        }
        self.require_key(at, key);
    }

    fn require_key(&mut self, at: &str, key: &str) {
        self.used_keys.insert(key.to_owned());
        for lang in LANGUAGES {
            let present = self
                .data
                .i18n
                .get(lang)
                .is_some_and(|t| t.contains_key(key));
            if !present {
                self.problem(at, format!("missing translation \"{key}\" in {lang}"));
            }
        }
    }

    // ---- checks on every entry -------------------------------------------

    fn entries(&mut self) {
        let data = self.data;
        let mut entries: Vec<(&'static str, &'a dyn Entry)> = Vec::new();
        data.rules
            .for_each_entry(|file, entry| entries.push((file, entry)));

        let mut seen: BTreeMap<&str, &str> = BTreeMap::new();
        for (file, entry) in entries {
            let id = entry.id();
            let at = format!("{file}/{id}");

            let valid = if file == "rulings" {
                is_ruling_id(id)
            } else {
                is_id(id)
            };
            if !valid {
                self.problem(&at, "id does not match the required pattern");
            }
            if let Some(other) = seen.insert(id, file) {
                self.problem(&at, format!("id is also used in {other}"));
            }

            let pages = entry.pages();
            if pages.is_empty() {
                self.problem(&at, "no page given");
            }
            for page in pages {
                if !(1..=MAX_PAGE).contains(&page) {
                    self.problem(&at, format!("page {page} is out of range"));
                }
            }

            self.require_key(&at, &format!("{id}.name"));
            let desc = format!("{id}.desc");
            self.used_keys.insert(desc);

            for effect in entry.effects() {
                self.effect(&at, id, effect);
            }
            for op in entry.on_apply() {
                self.op(&at, id, op);
            }
        }
    }

    fn lasts(&mut self, at: &str, lasts: &Lasts) {
        if let Lasts::Steps { count: 0, .. } = lasts {
            self.problem(at, "a duration needs at least one step");
        }
    }

    fn mod_target(&mut self, at: &str, target: &ModTarget) {
        if target.skills.is_empty()
            && target.based_on.is_empty()
            && target.rolls.is_empty()
            && target.ship_stats.is_empty()
            && target.stats.is_empty()
        {
            self.problem(at, "modifier applies to nothing");
        }
        for skill in &target.skills {
            self.reference(at, "skill", skill);
        }
        for attribute in &target.based_on {
            self.reference(at, "attribute", attribute);
        }
    }

    fn blocked(&mut self, at: &str, blocked: &Blocked) {
        if let Blocked::Action(action) = blocked {
            self.reference(at, "action", action);
        }
    }

    fn effect(&mut self, at: &str, owner: &str, effect: &Effect) {
        let at = format!("{at} ({})", effect.kind());
        let at = at.as_str();
        match effect {
            Effect::Modifier {
                applies_to,
                modifies,
                change,
                when,
                lasts,
                ..
            } => {
                self.mod_target(at, applies_to);
                if let Some(key) = when {
                    self.text_key(at, owner, key);
                    if applies_to.skills.is_empty() && applies_to.based_on.is_empty() {
                        self.problem(at, "a situation applies to skill rolls only");
                    }
                }
                let ship_only = applies_to.skills.is_empty()
                    && applies_to.based_on.is_empty()
                    && applies_to.rolls.is_empty()
                    && applies_to.stats.is_empty();
                match change {
                    Change::Add(0) => self.problem(at, "adding 0 changes nothing"),
                    Change::Multiply(n) | Change::Divide(n) if *n < 2 => {
                        self.problem(at, "factor must be at least 2")
                    }
                    Change::Multiply(_) | Change::Divide(_) if !ship_only => {
                        self.problem(at, "only ship stats can be multiplied or divided")
                    }
                    _ => {}
                }
                if *modifies == Modified::Pushes
                    && (!applies_to.rolls.is_empty() || !applies_to.ship_stats.is_empty())
                {
                    self.problem(at, "only skill rolls can be pushed");
                }
                lasts.iter().for_each(|l| self.lasts(at, l));
            }
            Effect::Restriction { blocks, lasts, .. } => {
                self.blocked(at, blocks);
                lasts.iter().for_each(|l| self.lasts(at, l));
            }
            Effect::Trigger {
                on, then, lasts, ..
            } => {
                match on {
                    TriggerEvent::SkillRoll(skills) => {
                        if skills.is_empty() {
                            self.problem(at, "trigger needs at least one skill");
                        }
                        for skill in skills {
                            self.reference(at, "skill", skill);
                        }
                    }
                    TriggerEvent::Action(action) => self.reference(at, "action", action),
                    TriggerEvent::Sleep => {}
                }
                self.ops(at, owner, then);
                lasts.iter().for_each(|l| self.lasts(at, l));
            }
            Effect::Periodic { then, lasts, .. } => {
                self.ops(at, owner, then);
                lasts.iter().for_each(|l| self.lasts(at, l));
            }
            Effect::Replacement {
                replaces,
                with,
                uses,
                lasts,
                ..
            } => {
                if uses.is_some_and(|u| u.count == 0) {
                    self.problem(at, "uses must be at least 1");
                }
                self.replacement(at, replaces, with);
                lasts.iter().for_each(|l| self.lasts(at, l));
            }
            Effect::Condition {
                condition, lasts, ..
            } => {
                self.reference(at, "condition", condition);
                lasts.iter().for_each(|l| self.lasts(at, l));
            }
            Effect::Capability { grants, lasts, .. } => {
                self.reference(at, "action", grants);
                lasts.iter().for_each(|l| self.lasts(at, l));
            }
            Effect::Hint { text, lasts, .. } => {
                self.text_key(at, owner, text);
                lasts.iter().for_each(|l| self.lasts(at, l));
            }
        }
    }

    fn replacement(&mut self, at: &str, replaces: &Replaceable, with: &ReplaceWith) {
        match (replaces, with) {
            (Replaceable::FacehuggersInRoll, ReplaceWith::Ignore) => {}
            (Replaceable::PanicResult { from }, ReplaceWith::PanicEffect(effect)) => {
                if !(7..=15).contains(from) {
                    self.problem(at, format!("panic result {from} is not a panic effect"));
                }
                if !self.off_table_panic.contains(effect.as_str()) {
                    self.problem(
                        at,
                        format!("\"{effect}\" is not a panic entry outside the table"),
                    );
                }
                self.used_panic_effects.insert(effect.clone());
            }
            (Replaceable::Roll(skill), ReplaceWith::Success) => self.reference(at, "skill", skill),
            (Replaceable::AttributeFor(skill), ReplaceWith::Attribute(attribute)) => {
                self.reference(at, "skill", skill);
                self.reference(at, "attribute", attribute);
            }
            (Replaceable::Skill(skill), ReplaceWith::Skill(other)) => {
                self.reference(at, "skill", skill);
                self.reference(at, "skill", other);
                if skill == other {
                    self.problem(at, "a skill cannot replace itself");
                }
            }
            _ => self.problem(at, "replacement and its result do not fit together"),
        }
    }

    fn ops(&mut self, at: &str, owner: &str, ops: &[Op]) {
        if ops.is_empty() {
            self.problem(at, "no operations given");
        }
        for op in ops {
            self.op(at, owner, op);
        }
    }

    fn op(&mut self, at: &str, owner: &str, op: &Op) {
        match op {
            Op::Stress { delta: 0, .. } => self.problem(at, "stress change of 0"),
            Op::Damage { amount: 0, .. } => self.problem(at, "damage of 0"),
            Op::Roll {
                roll,
                on_success,
                on_fail,
            } => {
                match roll {
                    RollSpec::Skill(skill) => self.reference(at, "skill", skill),
                    RollSpec::Attribute(attribute) => self.reference(at, "attribute", attribute),
                }
                if on_success.is_empty() && on_fail.is_empty() {
                    self.problem(at, "roll without consequence");
                }
            }
            Op::Block { blocks, lasts } => {
                self.blocked(at, blocks);
                self.lasts(at, lasts);
            }
            Op::Ask { question, on_no } => {
                self.text_key(at, owner, question);
                if on_no.is_empty() {
                    self.problem(at, "question without consequence");
                }
            }
            Op::Hint { text } => self.text_key(at, owner, text),
            Op::Special { handler } if !SPECIAL_HANDLERS.contains(&handler.as_str()) => {
                self.problem(at, format!("unknown special handler \"{handler}\""));
            }
            _ => {}
        }
        for child in op.children() {
            self.op(at, owner, child);
        }
    }

    // ---- checks per file -------------------------------------------------

    fn files(&mut self) {
        let data = self.data;
        let rules = &data.rules;

        for skill in &rules.skills {
            self.reference(
                &format!("skills/{}", skill.id),
                "attribute",
                &skill.attribute,
            );
        }
        for career in &rules.careers {
            self.career(career);
        }
        for talent in &rules.talents {
            let at = format!("talents/{}", talent.id);
            if let Some(career) = &talent.career {
                self.reference(&at, "career", career);
                let listed = rules
                    .careers
                    .iter()
                    .any(|c| c.id == *career && c.talents.contains(&talent.id));
                if !listed {
                    self.problem(&at, format!("not listed as a talent of \"{career}\""));
                }
            }
            // A talent works now or names the part of the app that will
            // make it work, not both.
            if talent.effects.is_empty() == talent.later.is_none() {
                self.problem(&at, "needs either effects or `later`");
            }
            if talent.repeatable && talent.parameter.is_none() {
                self.problem(&at, "a repeatable talent needs a parameter");
            }
        }

        self.panic_table();

        for key in ENGINE_TEXT_KEYS {
            self.require_key("engine", key);
        }
        for id in REQUIRED_CONSTANTS {
            if !rules.constants.iter().any(|c| c.id == *id) {
                self.problem("constants", format!("missing constant \"{id}\""));
            }
        }

        let mut d6 = BTreeSet::new();
        for trauma in &rules.traumas {
            let at = format!("traumas/{}", trauma.id);
            if !(1..=6).contains(&trauma.d6) {
                self.problem(&at, "d6 must be 1–6");
            }
            if !d6.insert(trauma.d6) {
                self.problem(&at, format!("d6 {} is used twice", trauma.d6));
            }
            self.ruling_switch(&at, trauma.ruling.as_ref());
        }

        let mut d66 = BTreeSet::new();
        for injury in &rules.critical_injuries {
            let at = format!("critical-injuries/{}", injury.id);
            if !is_d66(injury.d66) {
                self.problem(&at, format!("{} is not a D66 result", injury.d66));
            }
            if !d66.insert(injury.d66) {
                self.problem(&at, format!("d66 {} is used twice", injury.d66));
            }
            if injury.fatal.is_some() && injury.instant_death {
                self.problem(&at, "an injury is either fatal or instant death");
            }
        }

        let mut taken: Vec<(DamageTable, RollRange, &str)> = Vec::new();
        for damage in &rules.ship_damage {
            let at = format!("ship-damage/{}", damage.id);
            self.reference(&at, "ship system", &damage.system);
            for repair in &damage.repair {
                self.reference(&at, "skill", &repair.skill);
                if repair.count == 0 {
                    self.problem(&at, "repair count must be at least 1");
                }
            }
            self.ruling_switch(&at, damage.ruling.as_ref());

            let RollRange { from, to } = damage.roll;
            let Some(to) = to else {
                self.problem(&at, "damage tables have no open ranges");
                continue;
            };
            let valid = match damage.table {
                DamageTable::Minor => is_d66(from) && is_d66(to),
                DamageTable::Major => (2..=12).contains(&from) && (2..=12).contains(&to),
            };
            if !valid || from > to {
                self.problem(&at, format!("invalid roll range {from}–{to}"));
            }
            for (table, other, other_id) in &taken {
                let other_to = other.to.unwrap_or(u8::MAX);
                if *table == damage.table && from <= other_to && other.from <= to {
                    self.problem(&at, format!("roll range overlaps {other_id}"));
                }
            }
            taken.push((damage.table, damage.roll, &damage.id));
        }

        for ruling in &rules.rulings {
            let at = format!("rulings/{}", ruling.id);
            let options: BTreeSet<&str> = ruling.options.iter().map(String::as_str).collect();
            if options.is_empty() {
                self.problem(&at, "a ruling needs at least one option");
            }
            if options.len() != ruling.options.len() {
                self.problem(&at, "options must be unique");
            }
            if !options.contains(ruling.default.as_str()) {
                self.problem(
                    &at,
                    format!("default \"{}\" is not an option", ruling.default),
                );
            }
            for option in &ruling.options {
                if !is_id(option) {
                    self.problem(
                        &at,
                        format!("option \"{option}\" does not match the pattern"),
                    );
                }
                self.require_key(&at, &format!("{}.{option}", ruling.id));
            }
        }
    }

    /// A career: its key values, its three talents and its suggestions
    /// [p. 38–55].
    fn career(&mut self, career: &crate::data::model::Career) {
        let rules = &self.data.rules;
        let at = format!("careers/{}", career.id);
        self.reference(&at, "attribute", &career.key_attribute);
        if career.key_skills.len() != 3 {
            self.problem(&at, "a career has three key skills");
        }
        for skill in &career.key_skills {
            self.reference(&at, "skill", skill);
        }
        if career.talents.len() != 3 {
            self.problem(&at, "a career has three career talents");
        }
        for id in &career.talents {
            match rules.talents.iter().find(|t| t.id == *id) {
                Some(t) if t.career.as_deref() == Some(career.id.as_str()) => {}
                Some(_) => self.problem(&at, format!("talent \"{id}\" belongs to another career")),
                None => self.problem(&at, format!("unknown talent \"{id}\"")),
            }
        }
        if career.cash.dice == 0 || career.cash.times == 0 {
            self.problem(&at, "cash needs dice and a factor");
        }
        if career.names.is_empty() {
            self.problem(&at, "no suggested names");
        }
        for (kind, ids) in [
            ("look", &career.looks),
            ("item", &career.items),
            ("agenda", &career.agendas),
        ] {
            if ids.is_empty() {
                self.problem(&at, format!("no suggestions of kind {kind}"));
            }
            for id in ids {
                self.require_key(&at, &format!("{}.{kind}.{id}", career.id));
            }
        }
    }

    /// The panic table starts at 1, has no gaps and ends with an open
    /// range [p. 105].
    fn panic_table(&mut self) {
        let data = self.data;
        let mut rows: Vec<_> = data
            .rules
            .panic
            .iter()
            .filter_map(|r| r.roll.map(|roll| (r, roll)))
            .collect();
        rows.sort_by_key(|(_, roll)| roll.from);
        let mut next = 1;
        let last = rows.len().saturating_sub(1);
        for (i, (row, roll)) in rows.iter().enumerate() {
            let at = format!("panic/{}", row.id);
            if roll.from != next {
                self.problem(&at, format!("expected the range to start at {next}"));
            }
            match roll.to {
                Some(to) if i == last => {
                    self.problem(&at, format!("the last range must be open, not end at {to}"))
                }
                Some(to) if to < roll.from => self.problem(&at, "range ends before it starts"),
                Some(to) => next = to + 1,
                None if i != last => self.problem(&at, "only the last range may be open"),
                None => {}
            }
        }
        for id in self.off_table_panic.clone() {
            if !self.used_panic_effects.contains(id) {
                self.problem(
                    format!("panic/{id}"),
                    "entry outside the table is never used",
                );
            }
        }
    }

    /// Every option of the ruling except the default needs alternative
    /// effects; the default uses the entry's own effects.
    fn ruling_switch(&mut self, at: &str, switch: Option<&RulingSwitch>) {
        let Some(switch) = switch else { return };
        self.reference(at, "ruling", &switch.id);
        let Some(&ruling) = self.rulings.get(switch.id.as_str()) else {
            return;
        };
        let expected: BTreeSet<&str> = ruling
            .options
            .iter()
            .map(String::as_str)
            .filter(|o| *o != ruling.default)
            .collect();
        let given: BTreeSet<&str> = switch.alternatives.keys().map(String::as_str).collect();
        if expected != given {
            self.problem(
                at,
                format!(
                    "alternatives must be {expected:?} for {}, found {given:?}",
                    ruling.id
                ),
            );
        }
    }

    // ---- translations ----------------------------------------------------

    fn translations(&mut self) {
        let data = self.data;
        let Some(reference) = data.i18n.get(LANGUAGES[0]) else {
            self.problem("i18n", format!("missing language {}", LANGUAGES[0]));
            return;
        };
        let reference_keys: BTreeSet<&String> = reference.keys().collect();

        for (lang, texts) in &data.i18n {
            let at = format!("i18n/{lang}");
            if !LANGUAGES.contains(&lang.as_str()) {
                self.problem(&at, "unknown language");
            }
            let keys: BTreeSet<&String> = texts.keys().collect();
            for key in keys.symmetric_difference(&reference_keys) {
                self.problem(&at, format!("key \"{key}\" is not in every language"));
            }
            for (key, text) in texts {
                if text.trim().is_empty() {
                    self.problem(&at, format!("empty text for \"{key}\""));
                }
                if !self.used_keys.contains(key) {
                    self.problem(&at, format!("key \"{key}\" is not used by any entry"));
                }
            }
        }
    }
}

//! Rule data: types, loading, validation and JSON Schemas.
//!
//! The Rust types in [`model`] and [`effects`] define the data format. The
//! JSON Schemas in `data/schema` are generated from them (see
//! [`schemas`]); a test makes sure the files are up to date. The format is
//! described in `docs/datenformat.md`.

pub mod effects;
mod load;
pub mod model;
mod validate;

use std::path::Path;

use effects::{Effect, Op};
use model::*;

pub use load::{LoadError, Translations};
pub use validate::{Problem, validate};

/// Identifier of an entry, unique across all rule files.
pub type Id = String;
/// Page in the core rulebook (1st edition, 2019).
pub type Page = u16;

/// Languages in `data/i18n`.
pub const LANGUAGES: [&str; 2] = ["de", "en"];

/// Common view on entries of all rule files, used by the validation.
pub trait Entry {
    fn id(&self) -> &str;
    fn pages(&self) -> Vec<Page>;
    /// All continuous effects, including those of ruling alternatives.
    fn effects(&self) -> Vec<&Effect> {
        Vec::new()
    }
    /// Operations that run once when the entry is applied.
    fn on_apply(&self) -> &[Op] {
        &[]
    }
}

macro_rules! simple_entry {
    ($($ty:ty),*) => {$(
        impl Entry for $ty {
            fn id(&self) -> &str { &self.id }
            fn pages(&self) -> Vec<Page> { vec![self.page] }
        }
    )*};
}
simple_entry!(Attribute, Career, Skill, Action);

macro_rules! effect_entry {
    ($($ty:ty),*) => {$(
        impl Entry for $ty {
            fn id(&self) -> &str { &self.id }
            fn pages(&self) -> Vec<Page> { vec![self.page] }
            fn effects(&self) -> Vec<&Effect> { self.effects.iter().collect() }
        }
    )*};
}
effect_entry!(Condition, Talent, ShipSystem);

/// Effects of an entry plus those of its ruling alternatives.
fn with_alternatives<'a>(
    effects: &'a [Effect],
    ruling: &'a Option<RulingSwitch>,
) -> Vec<&'a Effect> {
    let mut all: Vec<&Effect> = effects.iter().collect();
    if let Some(switch) = ruling {
        all.extend(switch.alternatives.values().flatten());
    }
    all
}

impl Entry for Trauma {
    fn id(&self) -> &str {
        &self.id
    }
    fn pages(&self) -> Vec<Page> {
        vec![self.page]
    }
    fn effects(&self) -> Vec<&Effect> {
        with_alternatives(&self.effects, &self.ruling)
    }
}

impl Entry for PanicResult {
    fn id(&self) -> &str {
        &self.id
    }
    fn pages(&self) -> Vec<Page> {
        vec![self.page]
    }
    fn effects(&self) -> Vec<&Effect> {
        self.effects.iter().collect()
    }
    fn on_apply(&self) -> &[Op] {
        &self.on_apply
    }
}

impl Entry for CriticalInjury {
    fn id(&self) -> &str {
        &self.id
    }
    fn pages(&self) -> Vec<Page> {
        vec![self.page]
    }
    fn effects(&self) -> Vec<&Effect> {
        self.effects.iter().collect()
    }
    fn on_apply(&self) -> &[Op] {
        &self.on_apply
    }
}

impl Entry for ShipDamage {
    fn id(&self) -> &str {
        &self.id
    }
    fn pages(&self) -> Vec<Page> {
        vec![self.page]
    }
    fn effects(&self) -> Vec<&Effect> {
        with_alternatives(&self.effects, &self.ruling)
    }
    fn on_apply(&self) -> &[Op] {
        &self.on_apply
    }
}

impl Entry for Ruling {
    fn id(&self) -> &str {
        &self.id
    }
    fn pages(&self) -> Vec<Page> {
        self.pages.clone()
    }
}

/// Declares all files in `data/rules` once: the [`RuleData`] struct, the
/// loader, the iteration over entries and the schema list follow from it.
macro_rules! rule_files {
    ($($field:ident: $ty:ty => $file:literal),* $(,)?) => {
        /// All rule data, one field per file in `data/rules`.
        #[derive(Debug, Clone, Default, PartialEq, Eq)]
        pub struct RuleData {
            $(pub $field: Vec<$ty>,)*
        }

        /// File names in `data/rules` (without `.json`).
        pub const RULE_FILES: &[&str] = &[$($file),*];

        impl RuleData {
            /// Reads all files from `dir` (usually `data/rules`).
            pub fn load(dir: &Path) -> Result<Self, LoadError> {
                Ok(Self {
                    $($field: load::read_rule_file(dir, $file)?,)*
                })
            }

            /// Calls `f` with the file name and each entry.
            pub fn for_each_entry<'s>(&'s self, mut f: impl FnMut(&'static str, &'s dyn Entry)) {
                $(for entry in &self.$field { f($file, entry); })*
            }
        }

        /// JSON Schema of every rule file, by file name.
        pub fn schemas() -> Vec<(&'static str, schemars::Schema)> {
            vec![$(($file, schemars::schema_for!(DataFile<$ty>)),)*]
        }
    };
}

rule_files! {
    attributes: Attribute => "attributes",
    careers: Career => "careers",
    skills: Skill => "skills",
    actions: Action => "actions",
    panic: PanicResult => "panic",
    traumas: Trauma => "traumas",
    critical_injuries: CriticalInjury => "critical-injuries",
    conditions: Condition => "conditions",
    talents: Talent => "talents",
    ship_systems: ShipSystem => "ship-systems",
    ship_damage: ShipDamage => "ship-damage",
    rulings: Ruling => "rulings",
}

/// Rule data plus translations, as stored in `data/`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Data {
    pub rules: RuleData,
    /// Translations by language code.
    pub i18n: std::collections::BTreeMap<String, Translations>,
}

impl Data {
    /// Reads `rules/*.json` and `i18n/*.json` below `root` (usually `data`).
    pub fn load(root: &Path) -> Result<Self, LoadError> {
        let rules = RuleData::load(&root.join("rules"))?;
        let mut i18n = std::collections::BTreeMap::new();
        for lang in LANGUAGES {
            i18n.insert(lang.to_owned(), load::read_translations(root, lang)?);
        }
        Ok(Self { rules, i18n })
    }
}

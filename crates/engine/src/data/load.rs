//! Reading the JSON files in `data/`.

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

use super::model::DataFile;

/// Translation keys and texts of one language.
pub type Translations = BTreeMap<String, String>;

/// A data file could not be read.
#[derive(Debug)]
pub struct LoadError {
    pub path: PathBuf,
    pub message: String,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.message)
    }
}

impl std::error::Error for LoadError {}

fn read<T: DeserializeOwned>(path: &Path) -> Result<T, LoadError> {
    let error = |message: String| LoadError {
        path: path.to_owned(),
        message,
    };
    let text = std::fs::read_to_string(path).map_err(|e| error(e.to_string()))?;
    serde_json::from_str(&text).map_err(|e| error(e.to_string()))
}

/// Reads `<dir>/<name>.json` and checks its `$schema` reference.
pub(super) fn read_rule_file<T: DeserializeOwned>(
    dir: &Path,
    name: &str,
) -> Result<Vec<T>, LoadError> {
    let path = dir.join(format!("{name}.json"));
    let file: DataFile<T> = read(&path)?;
    let expected = format!("../schema/{name}.schema.json");
    match file.schema {
        Some(schema) if schema == expected => Ok(file.entries),
        other => Err(LoadError {
            path,
            message: format!("\"$schema\" must be \"{expected}\", found {other:?}"),
        }),
    }
}

/// Reads `<root>/i18n/<lang>.json`.
pub(super) fn read_translations(root: &Path, lang: &str) -> Result<Translations, LoadError> {
    read(&root.join("i18n").join(format!("{lang}.json")))
}

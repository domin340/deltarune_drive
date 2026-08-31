use crate::conf::default_app_path;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::io;

pub type Date = chrono::DateTime<Utc>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisteredBkp {
    pub name: String,
    pub desc: String,
    pub creation_date: Date,
    pub update_date: Option<Date>,
}

impl RegisteredBkp {
    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

impl Default for RegisteredBkp {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            desc: "".to_string(),
            creation_date: Utc::now(),
            update_date: None,
        }
    }
}

/// Backups not listed inside `settings.toml` but listed in the backups directory
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnregisteredBkp {
    pub name: String,
}

impl From<UnregisteredBkp> for RegisteredBkp {
    fn from(value: UnregisteredBkp) -> Self {
        Self::with_name(value.name)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Bkp {
    Registered(RegisteredBkp),
    Unregistered(UnregisteredBkp),
}

/// returns an iterator to listable files that looks like backups:
/// - is a file
/// - has "zip" extension
pub fn enlistable_bkp_files() -> io::Result<impl Iterator<Item = std::fs::DirEntry>> {
    std::fs::read_dir(default_app_path()).map(|dir| {
        dir.into_iter().filter_map(|e| e.ok()).filter(|e| {
            let path = e.path();
            path.is_file() && path.extension().is_some_and(|s| s == "zip")
        })
    })
}

use chrono::Utc;
use serde::{Deserialize, Serialize};

pub type Date = chrono::DateTime<Utc>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisteredBkp {
    pub name: String,
    pub desc: String,
    pub creation_date: Date,
    pub update_date: Option<Date>,
}

impl RegisteredBkp {
    pub fn new() -> Self {
        Self::default()
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
        Self {
            name: value.name,
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Bkp {
    Registered(RegisteredBkp),
    Unregistered(UnregisteredBkp),
}

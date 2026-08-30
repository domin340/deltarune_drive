use chrono::Utc;

pub type Date = chrono::DateTime<Utc>;

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

pub enum Bkp {
    Registered(RegisteredBkp),
    /// Backups not listed inside `settings.toml` but listed in the backups directory
    Unregistered {
        path: String,
    },
}

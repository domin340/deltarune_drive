use serde::{Deserialize, Serialize};

use crate::backups::Bkp;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeltarunePath {
    Steam,
    Custom(String),
}

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub deltarune_path: DeltarunePath,
    pub bkps_path: String,
    pub bkps: Vec<Bkp>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            bkps: vec![],
            bkps_path: String::from("./backups"),
            deltarune_path: DeltarunePath::Steam,
        }
    }
}

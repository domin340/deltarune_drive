use crate::backups::Bkp;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

pub const DATA_DIR_SUF: &str = "deltarune_drive_settings";
pub const CONF_FILE: &str = "settings.json";

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeltarunePath {
    Steam(PathBuf),
    Custom(PathBuf),
    None,
}

pub fn default_app_path() -> PathBuf {
    dirs::data_local_dir()
        .expect("User's operating system does not have a local data directory. Cannot proceed.")
        .join(DATA_DIR_SUF)
}

#[derive(Serialize, Deserialize)]
pub struct Conf {
    pub deltarune_path: DeltarunePath,
    pub app_path: PathBuf,
    pub bkps: Vec<Bkp>,
}

impl Conf {
    fn is_app_path_dir(&self) -> bool {
        self.app_path.is_dir()
    }

    fn ensure_initialized(&self) -> io::Result<()> {
        if self.is_app_path_dir() {
            Ok(())
        } else {
            std::fs::create_dir(&self.app_path)
        }
    }

    pub fn save(&self) -> Result<(), ConfFileError> {
        self.ensure_initialized()?;

        let json = serde_json::to_string_pretty(self)?;

        let conf_path = self.app_path.join(CONF_FILE);
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(conf_path)?
            .write_all(json.as_bytes())
            .map_err(ConfFileError::Io)
    }
}

impl Default for Conf {
    fn default() -> Self {
        Self {
            app_path: default_app_path(),
            deltarune_path: DeltarunePath::None,
            bkps: vec![],
        }
    }
}

#[derive(Debug)]
pub enum ConfFileError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl From<io::Error> for ConfFileError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for ConfFileError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

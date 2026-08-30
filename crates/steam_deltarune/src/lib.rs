pub mod sufs;

use dirs::home_dir;
use std::env::consts::OS;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeltarunePathError {
    /// could not retrieve the home directory
    HomeDirNotPresent,
    /// none of the suffixes in [`sufs`] match deltarune local data location
    DeltaruneNotPresent,
    /// operating system is not supported by the function
    OsNotSupported,
}

/// finds the appropriate deltarune path by:
///     - the operating system
///     - possible steam local data directory paths for the dedicated operating system
pub fn deltarune_path() -> Result<PathBuf, DeltarunePathError> {
    #[inline]
    fn if_is_dir(path: PathBuf) -> Result<PathBuf, DeltarunePathError> {
        if path.is_dir() {
            Ok(path)
        } else {
            Err(DeltarunePathError::DeltaruneNotPresent)
        }
    }

    let home = home_dir().ok_or(DeltarunePathError::HomeDirNotPresent)?;

    match OS {
        "windows" => if_is_dir(home.join(sufs::WINDOWS)),
        "macos" => if_is_dir(home.join(sufs::MACOS)),
        "linux" => [sufs::LINUX_NATIVE, sufs::LINUX_COMPAT, sufs::LINUX_FLATPAK]
            .into_iter()
            .map(|suf| home.join(suf))
            .find(|suf| suf.is_dir())
            .ok_or(DeltarunePathError::DeltaruneNotPresent),
        _ => Err(DeltarunePathError::OsNotSupported),
    }
}

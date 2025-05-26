pub mod config;

use std::path::PathBuf;

/// Returns the path to the directory where the application is running.
pub fn dir_rel() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        let mut base = PathBuf::from(".");
        base.push(".run");
        return base;
    }

    #[cfg(not(debug_assertions))]
    {
        return PathBuf::from(".");
    }
}

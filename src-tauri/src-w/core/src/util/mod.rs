pub mod compress;
pub mod config;
pub mod path_ext;

use std::path::PathBuf;
use std::process::Command;

/// Returns the path to the directory where the application is running.
pub fn dir_rel() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        let mut base = PathBuf::from(".");
        base.push(".run");
        base
    }

    #[cfg(not(debug_assertions))]
    {
        PathBuf::from(".")
    }
}

pub fn create_hidden_command(cmd: &str) -> Command {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let mut command = Command::new(cmd);
        command.creation_flags(CREATE_NO_WINDOW);
        command
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new(cmd)
    }
}

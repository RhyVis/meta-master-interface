mod command;

use command::*;
use m_core::data::library::lib_init;
use m_core::util::config::config_init;
use tauri::{generate_context, generate_handler};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            config_init();
            lib_init();

            Ok(())
        })
        .invoke_handler(generate_handler![
            metadata_get_all,
            metadata_get,
            metadata_update,
            metadata_remove
        ])
        .run(generate_context!())
        .expect("Failed to start application");
}

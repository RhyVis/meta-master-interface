mod command;

use command::*;
use m_core::data::library::lib_init;
use m_core::util::config::config_init;
use tauri::{generate_context, generate_handler};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_pinia::init())
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
            metadata_remove,
            metadata_deploy,
            metadata_deploy_off,
            library_clear,
            library_export,
            library_import,
            util_resolve_root,
            api_fetch_dl_site_maniax
        ])
        .run(generate_context!())
        .expect("Failed to start application");
}

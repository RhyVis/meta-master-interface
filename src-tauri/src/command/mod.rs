use m_core::data::library::*;
use m_core::data::metadata::{Metadata, MetadataOptional};
use tauri::command;

type CommandResult<T> = Result<T, String>;

#[command]
pub fn metadata_get_all() -> CommandResult<Vec<Metadata>> {
    lib_get_all().map_err(|e| e.to_string())
}

#[command]
pub fn metadata_get(key: &str) -> CommandResult<Metadata> {
    lib_get(key).map_err(|e| e.to_string())
}

#[command]
pub fn metadata_update(opt: MetadataOptional) -> CommandResult<String> {
    lib_update(opt).map_err(|e| e.to_string())
}

#[command]
pub fn metadata_remove(key: &str) -> CommandResult<bool> {
    lib_remove(key).map_err(|e| e.to_string())
}

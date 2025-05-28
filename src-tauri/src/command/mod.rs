use m_core::data::library::*;
use m_core::data::metadata::{Metadata, MetadataOptional};
use m_core::util::ToStringErr;
use std::env::current_dir;
use std::path::PathBuf;
use tauri::command;

type CommandResult<T> = Result<T, String>;

#[command]
pub fn metadata_get_all() -> CommandResult<Vec<Metadata>> {
    lib_get_all().string_err()
}

#[command]
pub fn metadata_get(key: &str) -> CommandResult<Metadata> {
    lib_get(key).string_err()
}

#[command]
pub fn metadata_update(opt: MetadataOptional) -> CommandResult<String> {
    lib_update(opt).string_err()
}

#[command]
pub fn metadata_remove(key: &str) -> CommandResult<bool> {
    lib_remove(key).string_err()
}

#[command]
pub fn library_export() -> CommandResult<()> {
    lib_export().string_err()
}

#[command]
pub fn library_import() -> CommandResult<()> {
    lib_import().string_err()
}

#[command]
pub fn util_resolve_absolute(path: &str) -> CommandResult<String> {
    current_dir()
        .unwrap_or(PathBuf::from("."))
        .join(path)
        .canonicalize()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| format!("Unable to resolve path: {e}"))
}

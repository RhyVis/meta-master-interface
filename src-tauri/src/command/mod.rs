use log::info;
use m_api::http::dl_site::{DLSiteInfo, Language, fetch_dl_site_maniax};
use m_common::ToStringErr;
use m_core::data::library::*;
use m_core::data::metadata::{Metadata, MetadataOptional};
use m_core::util::config::config_get;
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
pub fn metadata_deploy(key: &str, target: &str) -> CommandResult<()> {
    lib_deploy(key, target).string_err()
}

#[command]
pub fn metadata_deploy_off(key: &str) -> CommandResult<()> {
    lib_deploy_off(key).string_err()
}

#[command]
pub fn library_clear() -> CommandResult<()> {
    lib_clear().string_err()
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
pub fn util_resolve_root(path: &str, abs: bool) -> CommandResult<String> {
    let mut root = if abs {
        config_get().get_root_absolute()
    } else {
        config_get().get_root()
    };

    if !path.is_empty() {
        let path_buf = PathBuf::from(path);
        if path_buf.is_absolute() {
            root = path_buf;
        } else {
            root.push(path_buf);
        }
    }

    if root.exists() {
        Ok(root.display().to_string())
    } else {
        Err(format!("Path does not exist: {}", root.display()))
    }
}

#[command]
pub async fn api_fetch_dl_site_maniax(id: &str) -> CommandResult<DLSiteInfo> {
    info!("Requesting DL Site Maniax info for ID: {id}");
    fetch_dl_site_maniax(id, Language::ZhCn).await.string_err()
}

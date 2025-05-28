use crate::data::metadata::{Metadata, MetadataError, MetadataOptional};
use crate::util::config::config_get;
use chrono::Local;
use const_format::concatcp;
use log::{error, info, warn};
use redb::{Database, ReadableTable, TableDefinition};
use std::sync::OnceLock;
use std::{error, fs};
use thiserror::Error;

const LIB_FILE_STEM: &str = "library";
const LIB_FILE_EXT: &str = "bin";
const LIB_EXPORT_EXT: &str = "json";
const LIB_FILE_NAME: &str = concatcp!(LIB_FILE_STEM, ".", LIB_FILE_EXT);
const LIB_EXPORT_FILE_NAME: &str = concatcp!(LIB_FILE_STEM, ".", LIB_EXPORT_EXT);

const BACKUP_DIR: &str = "backup";

const LIB_TABLE: TableDefinition<&str, Vec<u8>> = TableDefinition::new("LIBRARY");

fn internal_lib() -> &'static Database {
    static DB: OnceLock<Database> = OnceLock::new();

    fn init_db() -> Database {
        let root_path = config_get().get_root();
        if !root_path.exists() {
            fs::create_dir_all(&root_path).expect("Failed to create application data directory");
        }

        let backup_dir = root_path.join(BACKUP_DIR);
        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir).expect("Failed to create backup directory");
        }

        let bin_path = root_path.join(LIB_FILE_NAME);
        if bin_path.exists() {
            let timestamp = Local::now().format("%Y%m%d%H%M%S");
            let backup_file =
                backup_dir.join(format!("{LIB_FILE_STEM}.{timestamp}.{LIB_FILE_EXT}"));
            fs::copy(&bin_path, &backup_file).expect("Failed to copy backup binary");

            let mut backups = fs::read_dir(&backup_dir)
                .expect("Failed to read backup directory")
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
                .collect::<Vec<_>>();
            backups.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());

            if backups.len() > 5 {
                for entry in &backups[..backups.len() - 5] {
                    let _ = fs::remove_file(entry.path());
                }
            }
        }

        Database::create(bin_path).expect("Unable to open library")
    }

    fn init_table(db: &Database) -> Result<(), Box<dyn error::Error>> {
        let write = db.begin_write()?;
        {
            let table = write.open_table(LIB_TABLE)?;
            table.get("SOME_KEY")?;
        }
        write.commit()?;
        Ok(())
    }

    DB.get_or_init(|| {
        let db = init_db();
        init_table(&db).expect("Unable to init library table");
        db
    })
}

fn internal_get(key: &str) -> LibraryResult<Metadata> {
    let read = internal_lib().begin_read()?;
    let table = read.open_table(LIB_TABLE)?;
    let raw = match table.get(key)? {
        Some(raw) => raw,
        None => return Err(LibraryError::NotFound(key.to_string())),
    };
    match bson::from_slice(&raw.value()) {
        Ok(v) => Ok(v),
        Err(e) => {
            error!("Failed to deserialize Metadata: {}", e);
            Err(LibraryError::DeserializationError(e))
        }
    }
}

fn internal_set(key: &str, value: Metadata) -> LibraryResult<()> {
    let write = internal_lib().begin_write()?;
    {
        let mut table = write.open_table(LIB_TABLE)?;
        let raw = bson::to_vec(&value)?;
        table.insert(key, raw)?;
    }
    write.commit()?;
    Ok(())
}

pub fn lib_init() {
    internal_lib();
}

pub fn lib_get(key: &str) -> LibraryResult<Metadata> {
    internal_get(key)
}

pub fn lib_get_all() -> LibraryResult<Vec<Metadata>> {
    let read = internal_lib().begin_read()?;
    let table = read.open_table(LIB_TABLE)?;
    let mut result = Vec::new();
    for entry in table.iter()? {
        let (_, raw) = entry?;
        match bson::from_slice(&raw.value()) {
            Ok(v) => result.push(v),
            Err(e) => {
                error!("Failed to deserialize Metadata: {}", e);
                return Err(LibraryError::DeserializationError(e));
            }
        }
    }
    Ok(result)
}

pub fn lib_update(mut opt: MetadataOptional) -> LibraryResult<String> {
    if let Some(id) = opt.id.clone() {
        info!("Updating metadata with id: {}", id);
        match internal_get(id.as_str()) {
            Ok(mut metadata) => {
                metadata = metadata.patch(opt)?;
                internal_set(id.as_str(), metadata)?;
                Ok(id)
            }
            Err(LibraryError::NotFound(_)) => {
                warn!(
                    "Unexpected update for non-existing metadata with id: {}",
                    id
                );
                opt.id = None;
                let metadata = Metadata::init(opt)?;
                let id = metadata.id.clone();
                internal_set(id.as_str(), metadata)?;
                Ok(id)
            }
            Err(e) => {
                error!("Error updating metadata with id {}: {}", id, e);
                Err(e)
            }
        }
    } else {
        let metadata = Metadata::init(opt)?;
        let id = metadata.id.clone();
        info!("Creating new metadata with id: {}", id);
        internal_set(id.as_str(), metadata)?;
        Ok(id)
    }
}

pub fn lib_remove(key: &str) -> LibraryResult<bool> {
    let write = internal_lib().begin_write()?;
    let removed = {
        let mut table = write.open_table(LIB_TABLE)?;
        table.remove(key)?.is_some()
    };
    write.commit()?;
    Ok(removed)
}

pub fn lib_deploy(key: &str, target: &str) -> LibraryResult<()> {
    let mut metadata = internal_get(key)?;
    if metadata.deploy(target)? {
        info!("Successfully deployed metadata with key: {}", key);
        internal_set(key, metadata)?;
        Ok(())
    } else {
        error!("Failed to deploy metadata with key: {}", key);
        internal_set(key, metadata)?;
        Err(LibraryError::DeployError)
    }
}

pub fn lib_deploy_off(key: &str) -> LibraryResult<()> {
    let mut metadata = internal_get(key)?;
    if metadata.deploy_off()? {
        info!("Successfully removed deployed metadata with key: {}", key);
        internal_set(key, metadata)?;
        Ok(())
    } else {
        error!("Failed to remove deployed metadata with key: {}", key);
        internal_set(key, metadata)?;
        Err(LibraryError::DeployError)
    }
}

pub fn lib_export() -> LibraryResult<()> {
    let all_metadata = lib_get_all()?;
    let export_path = config_get().get_root().join(LIB_EXPORT_FILE_NAME);
    fs::write(&export_path, serde_json::to_string_pretty(&all_metadata)?)?;
    info!("Library exported to {}", export_path.display());
    Ok(())
}

pub fn lib_import() -> LibraryResult<()> {
    let import_path = config_get().get_root().join(LIB_EXPORT_FILE_NAME);
    if !import_path.exists() {
        return Err(LibraryError::FileSystemError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Import file not found",
        )));
    }

    let data = fs::read_to_string(&import_path)?;
    let metadata_list = serde_json::from_str::<Vec<Metadata>>(&data)?;

    let write = internal_lib().begin_write()?;
    {
        let mut table = write.open_table(LIB_TABLE)?;
        for metadata in metadata_list {
            let raw = bson::to_vec(&metadata)?;
            table.insert(metadata.id.as_str(), raw)?;
        }
    }
    write.commit()?;

    info!("Library imported from {}", import_path.display());
    Ok(())
}

type LibraryResult<T> = Result<T, LibraryError>;

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("Metadata with key {0} not found")]
    NotFound(String),

    #[error("Failed to deploy metadata due to missing or invalid info")]
    DeployError,

    #[error("Metadata internal error: {0}")]
    MetadataError(#[from] MetadataError),

    #[error("Database transaction error: {0}")]
    TransactionError(#[from] redb::TransactionError),

    #[error("Database table error: {0}")]
    TableError(#[from] redb::TableError),

    #[error("Database storage error: {0}")]
    StorageError(#[from] redb::StorageError),

    #[error("Database commit error: {0}")]
    CommitError(#[from] redb::CommitError),

    #[error("BSON serialization error: {0}")]
    SerializationError(#[from] bson::ser::Error),

    #[error("BSON deserialization error: {0}")]
    DeserializationError(#[from] bson::de::Error),

    #[error("Export/Import with json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("File system error: {0}")]
    FileSystemError(#[from] std::io::Error),
}

use crate::data::metadata::{Metadata, MetadataError, MetadataOptional};
use crate::util::config::config_get;
use const_format::concatcp;
use log::{error, info, warn};
use redb::{Database, ReadableTable, TableDefinition};
use std::error;
use std::sync::OnceLock;
use thiserror::Error;

const LIB_FILE_STEM: &str = "library";
const LIB_FILE_EXT: &str = "bin";
const LIB_FILE_NAME: &str = concatcp!(LIB_FILE_STEM, ".", LIB_FILE_EXT);

const LIB_TABLE: TableDefinition<&str, Vec<u8>> = TableDefinition::new("LIBRARY");

fn internal_lib() -> &'static Database {
    static DB: OnceLock<Database> = OnceLock::new();

    fn init_db() -> Database {
        let root_path = config_get().get_root();
        if !root_path.exists() {
            std::fs::create_dir_all(&root_path)
                .expect("Failed to create application data directory");
        }

        Database::create(root_path.join(LIB_FILE_NAME)).expect("Unable to open library")
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

pub fn lib_new(opt: MetadataOptional) -> LibraryResult<String> {
    let metadata = Metadata::init_mapped(opt)?;
    let id = metadata.id.clone();
    internal_set(id.as_str(), metadata)?;
    Ok(id)
}

pub fn lib_patch(key: &str, opt: MetadataOptional) -> LibraryResult<()> {
    let mut metadata = internal_get(key)?;
    metadata = metadata.patch(opt)?;
    internal_set(key, metadata)?;
    Ok(())
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
                let metadata = Metadata::init_mapped(opt)?;
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
        let metadata = Metadata::init_mapped(opt)?;
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

type LibraryResult<T> = Result<T, LibraryError>;

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("Metadata with key {0} not found")]
    NotFound(String),

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

    #[error("File system error: {0}")]
    FileSystemError(std::io::Error),
}

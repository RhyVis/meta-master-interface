use crate::Or;
use crate::Or::{That, This};
use crate::util::compress::{compress, decompress};
use crate::util::config::config_get;
use crate::util::path_ext::PathExt;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use fs_extra::dir;
use fs_extra::dir::CopyOptions;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub enum DistributionPlatform {
    #[default]
    Unknown,
    Steam {
        id: String,
    },
    DLSite {
        id: String,
    },
    Other {
        name: String,
        id: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub enum ArchiveInfo {
    #[default]
    Unset,
    ArchiveFile {
        path: String,
        password: Option<String>,
    },
    CommonFile {
        path: String,
    },
    Directory {
        path: String,
    },
}

impl ArchiveInfo {
    pub fn try_resolve(&self) -> Or<PathBuf, ArchiveInfo> {
        match self {
            ArchiveInfo::Unset => {
                warn!("ArchiveInfo is unset, cannot resolve path.");
                That(ArchiveInfo::Unset)
            }
            ArchiveInfo::ArchiveFile { path, .. } => {
                let path = Path::new(path);
                if path.exists() {
                    This(path.to_path_buf())
                } else {
                    warn!(
                        "The specified archive file path does not exist: {}",
                        path.display()
                    );
                    That(ArchiveInfo::Unset)
                }
            }
            ArchiveInfo::CommonFile { path } => {
                let path = Path::new(path);
                if path.exists() {
                    This(path.to_path_buf())
                } else {
                    warn!(
                        "The specified common file path does not exist: {}",
                        path.display()
                    );
                    That(ArchiveInfo::Unset)
                }
            }
            ArchiveInfo::Directory { path } => {
                let path = Path::new(path);
                if path.exists() {
                    This(path.to_path_buf())
                } else {
                    warn!(
                        "The specified directory path does not exist: {}",
                        path.display()
                    );
                    That(ArchiveInfo::Unset)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub enum DeployInfo {
    #[default]
    Unset,
    Directory {
        path: String,
    },
    File {
        path: String,
    },
}

impl DeployInfo {
    pub fn try_resolve(&self) -> Or<PathBuf, DeployInfo> {
        match self {
            DeployInfo::Unset => {
                warn!("DeployInfo is unset, cannot resolve path.");
                That(DeployInfo::Unset)
            }
            DeployInfo::File { path } => {
                let path = Path::new(path);
                if path.exists() {
                    This(path.to_path_buf())
                } else {
                    warn!("The specified file path does not exist: {}", path.display());
                    That(DeployInfo::Unset)
                }
            }
            DeployInfo::Directory { path } => {
                let path = Path::new(path);
                if path.exists() {
                    This(path.to_path_buf())
                } else {
                    warn!(
                        "The specified directory path does not exist: {}",
                        path.display()
                    );
                    That(DeployInfo::Unset)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct Metadata {
    #[serde(default = "default_id")]
    #[builder(default = "default_id()")]
    pub id: String,

    #[serde(default)]
    pub title: String,

    #[serde(default)]
    #[builder(default)]
    pub alias: Vec<String>,

    #[serde(default)]
    #[builder(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    #[builder(default)]
    pub platform: DistributionPlatform,

    #[serde(default)]
    #[builder(default)]
    pub description: Option<String>,

    #[serde(default)]
    #[builder(default)]
    pub developer: Option<String>,

    #[serde(default)]
    #[builder(default)]
    pub publisher: Option<String>,

    #[serde(default = "default_version")]
    #[builder(default = "default_version()")]
    pub version: String,

    #[serde(default)]
    #[builder(default)]
    pub archive_info: ArchiveInfo,

    #[serde(default)]
    #[builder(default)]
    pub deploy_info: DeployInfo,

    #[serde(default = "Utc::now")]
    #[builder(default = "Utc::now()")]
    pub time_created: DateTime<Utc>,

    #[serde(default = "Utc::now")]
    #[builder(default = "Utc::now()")]
    pub time_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetadataOptional {
    pub id: Option<String>,
    pub title: Option<String>,
    pub alias: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub platform: Option<DistributionPlatform>,
    pub description: Option<String>,
    pub developer: Option<String>,
    pub publisher: Option<String>,
    pub version: Option<String>,
    pub archive_info: Option<ArchiveInfo>,
    pub deploy_info: Option<DeployInfo>,
    pub time_created: Option<DateTime<Utc>>,
    pub time_updated: Option<DateTime<Utc>>,

    #[serde(default)]
    pub flag_create_archive: bool,
}

fn default_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn default_version() -> String {
    "1.0.0".to_string()
}

impl Metadata {
    fn map_builder(
        mut builder: MetadataBuilder,
        opt: MetadataOptional,
    ) -> MetadataResult<MetadataBuilder> {
        if let Some(title) = opt.title {
            builder = builder.title(title);
        }
        if let Some(alias) = opt.alias {
            builder = builder.alias(alias);
        }
        if let Some(tags) = opt.tags {
            builder = builder.tags(tags);
        }
        if let Some(platform) = opt.platform.clone() {
            builder = builder.platform(platform);
        }
        if let Some(description) = opt.description {
            builder = builder.description(description);
        }
        if let Some(developer) = opt.developer {
            builder = builder.developer(developer);
        }
        if let Some(publisher) = opt.publisher {
            builder = builder.publisher(publisher);
        }
        if let Some(version) = opt.version {
            builder = builder.version(version);
        }
        if let Some(deploy_info) = opt.deploy_info {
            builder = builder.deploy_info(deploy_info);
        }
        if let Some(time_created) = opt.time_created {
            builder = builder.time_created(time_created);
        }
        if let Some(time_updated) = opt.time_updated {
            builder = builder.time_updated(time_updated);
        }
        builder = Self::map_archive_info(
            builder,
            opt.archive_info,
            opt.platform,
            opt.flag_create_archive,
        )?;
        Ok(builder)
    }

    fn map_archive_info(
        mut builder: MetadataBuilder,
        archive_info: Option<ArchiveInfo>,
        platform_info: Option<DistributionPlatform>,
        flag: bool,
    ) -> MetadataResult<MetadataBuilder> {
        if flag {
            info!("Creating new archive info for metadata with compression flag enabled.");
            if let Some(ArchiveInfo::ArchiveFile { path, password }) = archive_info {
                let path = Path::new(&path);
                if !path.exists() {
                    warn!(
                        "The specified directory path does not exist: {}",
                        path.display()
                    );
                    return Err(MetadataError::MapConversionError);
                }

                let (dir, filename) = match platform_info {
                    Some(DistributionPlatform::Steam { id }) => ("Steam", id.to_string()),
                    Some(DistributionPlatform::DLSite { id }) => ("DLSite", id.to_string()),
                    Some(DistributionPlatform::Other { name, id }) => (
                        "Other",
                        format!(
                            "{name}-{}",
                            id.unwrap_or_else(|| Utc::now()
                                .format("%Y-%m-%d-%H-%M-%S")
                                .to_string())
                        ),
                    ),
                    _ => (
                        "Unknown",
                        format!(
                            "Unknown-{}",
                            Utc::now().format("%Y-%m-%d-%H-%M-%S").to_string()
                        ),
                    ),
                };
                let store_dir = config_get().get_archive_dir().join(dir);
                if !store_dir.exists() {
                    fs::create_dir_all(&store_dir)?;
                }
                let store_path = store_dir.join(format!("{filename}.7z"));
                info!("Archive going to be created at: {}", store_path.display());

                match compress(path, &store_path, password.as_deref(), None) {
                    Ok(_) => {
                        info!("Successfully created archive for metadata as: {}", filename);
                        builder = builder.archive_info(ArchiveInfo::ArchiveFile {
                            path: store_path.to_string_lossy().to_string(),
                            password,
                        });
                    }
                    Err(err) => {
                        warn!("Failed to create archive for {filename}: {err}");
                        builder = builder.archive_info(ArchiveInfo::Unset)
                    }
                }
            } else {
                warn!("The flag_create_archive is set, but no valid archive info provided.");
                builder = builder.archive_info(ArchiveInfo::Unset);
            }
            Ok(builder)
        } else {
            info!("Using provided archive info without compression.");
            builder = builder.archive_info(archive_info.unwrap_or_default());
            Ok(builder)
        }
    }

    pub fn init(opt: MetadataOptional) -> MetadataResult<Self> {
        let mut builder = MetadataBuilder::default();
        builder = Self::map_builder(builder, opt)?;
        builder.build().map_err(|err| {
            error!("Failed to build Metadata from map: {}", err);
            MetadataError::MapConversionError
        })
    }

    pub fn patch(self, opt: MetadataOptional) -> MetadataResult<Metadata> {
        let mut builder: MetadataBuilder = self.into();
        builder = Self::map_builder(builder, opt)?;
        builder = builder.time_updated(Utc::now());
        builder.build().map_err(|err| {
            error!("Failed to build Metadata from map: {}", err);
            MetadataError::MapConversionError
        })
    }

    pub fn deploy(&mut self, target_path: &str) -> MetadataResult<bool> {
        let target_path = Path::new(target_path);
        if !target_path.exists() || !target_path.is_dir() {
            warn!("Target path not valid: {}", target_path.display());
            return Err(MetadataError::TargetPathError(
                target_path.display().to_string(),
            ));
        }
        match self.archive_info.try_resolve() {
            This(source_path) => match &self.archive_info {
                ArchiveInfo::CommonFile { .. } => {
                    let target_path =
                        target_path.join(source_path.file_name().unwrap_or(OsStr::new("Why")));
                    info!(
                        "Deploying file by copying from {} to {}",
                        source_path.display(),
                        target_path.display()
                    );

                    fs::copy(source_path, &target_path)?;

                    self.deploy_info = DeployInfo::File {
                        path: target_path.to_string_lossy().to_string(),
                    };

                    info!("File deployed successfully to {}", target_path.display());
                    Ok(true)
                }
                ArchiveInfo::ArchiveFile { password, .. } => {
                    if !target_path.is_dir_empty() {
                        error!(
                            "Target directory {} is not empty, cannot deploy archive.",
                            target_path.display()
                        );
                        return Err(MetadataError::TargetPathError(
                            target_path.display().to_string(),
                        ));
                    }

                    info!(
                        "Deploying archive by extracting from {} to {}",
                        source_path.display(),
                        target_path.display()
                    );

                    decompress(source_path, target_path, password.as_deref())?;

                    self.deploy_info = DeployInfo::Directory {
                        path: target_path.to_string_lossy().to_string(),
                    };

                    info!("Archive deployed successfully to {}", target_path.display());

                    Ok(true)
                }
                ArchiveInfo::Directory { .. } => {
                    if !target_path.is_dir_empty() {
                        error!(
                            "Target directory {} is not empty, cannot deploy directory.",
                            target_path.display()
                        );
                        return Err(MetadataError::TargetPathError(
                            target_path.display().to_string(),
                        ));
                    }

                    info!(
                        "Deploying directory by copying from {} to {}",
                        source_path.display(),
                        target_path.display()
                    );

                    dir::copy(
                        source_path,
                        target_path,
                        &CopyOptions::new().copy_inside(true).overwrite(true),
                    )?;

                    self.deploy_info = DeployInfo::Directory {
                        path: target_path.to_string_lossy().to_string(),
                    };

                    info!(
                        "Directory deployed successfully to {}",
                        target_path.display()
                    );

                    Ok(true)
                }
                ArchiveInfo::Unset => {
                    warn!("Archive info is unset, cannot deploy.");
                    Err(MetadataError::InvalidDeployOperation)
                }
            },
            That(update) => {
                error!("Archive info is invalid or missing");
                self.archive_info = update;
                Ok(false)
            }
        }
    }

    pub fn deploy_off(&mut self) -> MetadataResult<bool> {
        let path = match self.deploy_info.try_resolve() {
            This(path) => path,
            That(update) => {
                error!("Deploy info is invalid or missing");
                self.deploy_info = update;
                return Ok(false);
            }
        };
        match self.deploy_info {
            DeployInfo::Unset => {
                warn!("Deploy info not exists, cannot deploy off.");
                Err(MetadataError::InvalidDeployOperation)
            }
            DeployInfo::File { .. } => {
                info!("Removing deployed file at: {}", path.display());
                fs::remove_file(&path)?;
                self.deploy_info = DeployInfo::Unset;
                info!("File deployed off successfully.");
                Ok(true)
            }
            DeployInfo::Directory { .. } => {
                info!("Removing deployed directory at: {}", path.display());
                path.clear_dir()?;
                self.deploy_info = DeployInfo::Unset;
                info!("Directory deployed off successfully.");
                Ok(true)
            }
        }
    }
}

impl From<Metadata> for MetadataBuilder {
    fn from(metadata: Metadata) -> Self {
        MetadataBuilder::create_empty()
            .id(metadata.id)
            .title(metadata.title)
            .alias(metadata.alias)
            .tags(metadata.tags)
            .platform(metadata.platform)
            .description(metadata.description)
            .developer(metadata.developer)
            .publisher(metadata.publisher)
            .version(metadata.version)
            .archive_info(metadata.archive_info)
            .deploy_info(metadata.deploy_info)
            .time_created(metadata.time_created)
            .time_updated(metadata.time_updated)
    }
}

type MetadataResult<T> = Result<T, MetadataError>;

#[derive(Debug, Error)]
pub enum MetadataError {
    #[error("Failed with mapping builder values")]
    MapConversionError,

    #[error("Failed with file system operation: {0}")]
    FileSystemError(#[from] std::io::Error),

    #[error("Failed with fs_extra operation: {0}")]
    FileSystemErrorEx(#[from] fs_extra::error::Error),

    #[error("Target path is not a valid directory or does not exist")]
    TargetPathError(String),

    #[error("Deploy related info is missing or invalid")]
    InvalidDeployOperation,
}

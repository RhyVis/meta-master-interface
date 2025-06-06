use crate::Whether::{That, This};
use crate::util::compress::{compress, decompress};
use crate::util::config::config_get;
use crate::util::path_ext::PathExt;
use crate::{DIR_ARCHIVE, Whether};
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
pub enum ContentType {
    #[default]
    Other,
    Game,
    Novel,
    Comic,
    Anime,
    Music,
    Movie,
    Software,
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
    pub fn try_resolve(&self) -> Whether<PathBuf, ArchiveInfo> {
        match self {
            ArchiveInfo::Unset => {
                warn!("ArchiveInfo is unset, cannot resolve path.");
                That(ArchiveInfo::Unset)
            }
            ArchiveInfo::ArchiveFile { path, .. } => {
                let path_seg = Path::new(path);
                // Handle relative paths by resolving them to the root directory
                let path = config_get().resolve_to_root(path_seg);
                if path.exists() {
                    This(path.into())
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
    pub fn calculate_size(&self) -> u64 {
        match self.try_resolve() {
            This(path) => path.calculate_size().unwrap_or_default(),
            That(_) => 0,
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
    pub fn try_resolve(&self) -> Whether<PathBuf, DeployInfo> {
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
    pub content_type: ContentType,

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
    pub archive_size: u64,

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
    pub content_type: Option<ContentType>,
    pub platform: Option<DistributionPlatform>,
    pub description: Option<String>,
    pub developer: Option<String>,
    pub publisher: Option<String>,
    pub version: Option<String>,
    pub archive_info: Option<ArchiveInfo>,
    pub archive_size: Option<u64>,
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
        if let Some(content_type) = opt.content_type {
            builder = builder.content_type(content_type);
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
                    Some(DistributionPlatform::Steam { id }) => ("Steam", format!("{id}.7z")),
                    Some(DistributionPlatform::DLSite { id }) => ("DLSite", format!("{id}.7z")),
                    Some(DistributionPlatform::Other { name, id }) => (
                        "Other",
                        format!(
                            "{name}-{}.7z",
                            id.unwrap_or_else(|| Utc::now()
                                .format("%Y-%m-%d-%H-%M-%S")
                                .to_string())
                        ),
                    ),
                    _ => (
                        "Unknown",
                        format!("Unknown-{}.7z", Utc::now().format("%Y-%m-%d-%H-%M-%S")),
                    ),
                };

                let archive_seg_parent = Path::new(DIR_ARCHIVE).join(dir);
                if !archive_seg_parent.exists() {
                    fs::create_dir_all(&archive_seg_parent)?;
                }

                let archive_seg = archive_seg_parent.join(&filename);
                let archive_path = config_get().resolve_to_root(&archive_seg);

                info!("Archive going to be created at: {}", archive_path.display());

                match compress(path, &archive_path, password.as_deref(), None) {
                    Ok(_) => {
                        info!("Successfully created archive for metadata as: {}", filename);
                        builder =
                            builder.archive_size(archive_path.calculate_size().unwrap_or_default());
                        builder = builder.archive_info(ArchiveInfo::ArchiveFile {
                            path: archive_seg.to_string_lossy().to_string(),
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
                builder = builder.archive_size(0u64);
            }
            Ok(builder)
        } else {
            info!("Using provided archive info without compression.");
            builder = builder.archive_size(
                archive_info
                    .as_ref()
                    .map_or(0, |info| info.calculate_size()),
            );
            builder = builder.archive_info(archive_info.unwrap_or_default());
            Ok(builder)
        }
    }

    pub fn mark_update(&mut self) {
        self.time_updated = Utc::now();
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
                    self.mark_update();

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
                    self.mark_update();

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
                    self.mark_update();

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
                self.mark_update();
                info!("File deployed off successfully.");
                Ok(true)
            }
            DeployInfo::Directory { .. } => {
                info!("Removing deployed directory at: {}", path.display());
                path.clear_dir()?;
                self.deploy_info = DeployInfo::Unset;
                self.mark_update();
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
            .content_type(metadata.content_type)
            .description(metadata.description)
            .developer(metadata.developer)
            .publisher(metadata.publisher)
            .version(metadata.version)
            .archive_info(metadata.archive_info)
            .archive_size(metadata.archive_size)
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

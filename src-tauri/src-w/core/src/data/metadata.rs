use chrono::{DateTime, Utc};
use derive_builder::Builder;
use log::error;
use serde::{Deserialize, Serialize};
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
    pub time_created: Option<DateTime<Utc>>,
    pub time_updated: Option<DateTime<Utc>>,
}

fn default_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn default_version() -> String {
    "1.0.0".to_string()
}

impl Metadata {
    fn map_builder(mut builder: MetadataBuilder, opt: MetadataOptional) -> MetadataBuilder {
        if let Some(title) = opt.title {
            builder = builder.title(title);
        }
        if let Some(alias) = opt.alias {
            builder = builder.alias(alias);
        }
        if let Some(tags) = opt.tags {
            builder = builder.tags(tags);
        }
        if let Some(platform) = opt.platform {
            builder = builder.platform(platform);
        }
        if let Some(description) = opt.description {
            builder = builder.description(description);
        }
        if let Some(developer) = opt.developer {
            builder = builder.developer(developer);
        }
        if let Some(version) = opt.version {
            builder = builder.version(version);
        }
        if let Some(publisher) = opt.publisher {
            builder = builder.publisher(publisher);
        }
        builder
    }

    pub fn init_mapped(opt: MetadataOptional) -> MetadataResult<Self> {
        let mut builder = MetadataBuilder::default();
        builder = Self::map_builder(builder, opt);
        builder.build().map_err(|err| {
            error!("Failed to build Metadata from map: {}", err);
            MetadataError::MapConversionError
        })
    }

    pub fn patch(self, opt: MetadataOptional) -> MetadataResult<Metadata> {
        let mut builder: MetadataBuilder = self.into();
        builder = Self::map_builder(builder, opt);
        builder = builder.time_updated(Utc::now());
        builder.build().map_err(|err| {
            error!("Failed to build Metadata from map: {}", err);
            MetadataError::MapConversionError
        })
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
            .time_created(metadata.time_created)
            .time_updated(metadata.time_updated)
    }
}

type MetadataResult<T> = Result<T, MetadataError>;

#[derive(Debug, Error)]
pub enum MetadataError {
    #[error("Failed with mapping builder values")]
    MapConversionError,
}

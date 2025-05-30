use crate::util::dir_rel;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigRaw {
    pub root: String,
}

impl Default for ConfigRaw {
    fn default() -> Self {
        Self {
            root: String::from("data"),
        }
    }
}

impl From<ConfigRaw> for Config {
    fn from(raw: ConfigRaw) -> Self {
        let path = Path::new(&raw.root);
        if !path.exists() {
            if let Err(err) = fs::create_dir_all(path) {
                let msg = format!("Failed to create root directory '{}': {}", raw.root, err);
                error!("{msg}");
                panic!("{msg}");
            }
        }

        if path.is_absolute() {
            info!("Data root directory set to: {} (absolute)", path.display());
        } else {
            info!(
                "Data root directory set to: {} (resolved from application directory)",
                path.canonicalize()
                    .expect("Failed to canonicalize path")
                    .display()
            );
        }

        Self {
            root_path: if path.is_absolute() {
                path.to_path_buf()
            } else {
                dir_rel().join(path)
            },
        }
    }
}

#[derive(Debug)]
pub struct Config {
    root_path: PathBuf,
}

impl Config {
    pub fn get_root(&self) -> PathBuf {
        self.root_path.clone()
    }

    pub fn get_root_absolute(&self) -> PathBuf {
        self.root_path
            .canonicalize()
            .expect("Failed to get absolute path")
    }

    pub fn resolve_to_root(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.get_root().join(path)
        }
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

const CONFIG_FILE_NAME: &str = "config.toml";

pub fn config_init() {
    fn init() {
        let application_dir = dir_rel();
        if !application_dir.exists() {
            fs::create_dir_all(&application_dir).expect("Failed to create application directory");
        }
        let config_path = application_dir.join(CONFIG_FILE_NAME);
        let config_raw = match fs::read_to_string(&config_path) {
            Ok(content) => match toml::from_str::<ConfigRaw>(content.as_str()) {
                Ok(config) => config,
                Err(err) => {
                    let msg = format!("Failed to parse config file: {}", err);
                    error!("{msg}");
                    panic!("{msg}");
                }
            },
            Err(err) => match err.kind() {
                ErrorKind::NotFound => {
                    info!(
                        "Config file {} not exists, creating the default one",
                        config_path.display()
                    );
                    let default = ConfigRaw::default();
                    fs::write(
                        &config_path,
                        toml::to_string(&default).expect("Failed to serialize default config"),
                    )
                    .expect("Failed to create default config");
                    default
                }
                _ => {
                    let msg = format!("Failed to read config file: {}", err);
                    error!("{msg}");
                    panic!("{msg}");
                }
            },
        };

        CONFIG
            .set(config_raw.into())
            .expect("Failed to init static config");
    }

    init()
}

pub fn config_get() -> &'static Config {
    CONFIG.get().expect("Config is not initialized")
}

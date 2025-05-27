use crate::util::dir_rel;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    root: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root: String::from("data"),
        }
    }
}

impl Config {
    pub fn get_root(&self) -> PathBuf {
        dir_rel().join(&self.root)
    }

    pub fn get_archive_dir(&self) -> PathBuf {
        const ARCHIVE_DIR: &str = "archive";
        self.get_root().join(ARCHIVE_DIR)
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
        let config = match fs::read_to_string(&config_path) {
            Ok(content) => match toml::from_str::<Config>(content.as_str()) {
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
                    let default = Config::default();
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

        CONFIG.set(config).expect("Failed to init static config");
    }

    init()
}

pub fn config_get() -> &'static Config {
    CONFIG.get().expect("Config is not initialized")
}

use config::{File, FileFormat};
use serde::Deserialize;
use thiserror::Error;

use crate::dirs::DIRS;

const CONFIG_NAMES: &[&str] = &["sfd.yaml", "sfd.yml"];

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to parse config: {0}")]
    Parse(#[from] config::ConfigError),
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub sample: String,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let user_conf = CONFIG_NAMES.iter().map(|name| DIRS.config_dir().join(name));

        let cwd = std::env::current_dir().expect("failed to get CWD");
        let proj_conf = cwd
            .ancestors()
            .flat_map(|dir| ["sfd.yaml", "sfd.yml"].iter().map(|name| dir.join(name)));

        let settings = user_conf
            .chain(proj_conf)
            .fold(config::Config::builder(), |builder, path| {
                builder
                    .add_source(File::new(path.to_str().unwrap(), FileFormat::Yaml).required(false))
            })
            .build()?
            .try_deserialize()?;

        Ok(settings)
    }
}

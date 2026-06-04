use schematic::ConfigLoader;
use thiserror::Error;

use crate::dirs::DIRS;

const CONFIG_NAMES: &[&str] = &["sfd.yaml", "sfd.yml"];

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to load config: {0}")]
    Load(#[from] schematic::ConfigError),
}

#[derive(Debug, schematic::Config)]
pub struct Config {
    #[setting(default = String::new())]
    pub sample: String,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let user_paths = CONFIG_NAMES.iter().map(|name| DIRS.config_dir().join(name));

        let cwd = std::env::current_dir().expect("failed to get CWD");
        let proj_paths = cwd
            .ancestors()
            .flat_map(|dir| CONFIG_NAMES.iter().map(|name| dir.join(name)));

        let mut loader = ConfigLoader::new();
        for path in user_paths.chain(proj_paths) {
            loader.file_optional(path)?;
        }

        let config = loader.load()?.config;

        Ok(config)
    }
}

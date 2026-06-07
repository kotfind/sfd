use std::fs;
use std::path::{Path, PathBuf};

use schematic::ConfigLoader;
use thiserror::Error;

use crate::dirs::DIRS;
use crate::extract::config::{ExtractConfig, PartialExtractConfig};

const CONFIG_NAMES: &[&str] = &["sfd.yaml", "sfd.yml"];

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to load config: {0}")]
    Load(#[from] schematic::ConfigError),
}

#[derive(Debug, schematic::Config)]
pub struct Config {
    #[setting(nested)]
    pub extract: ExtractConfig,
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

    fn get_files() -> Result<Self, ConfigError> {
        let cwd = std::env::current_dir().expect("failed to get CWD");

        let user_cfg = get_first_existing([DIRS.config_dir()], CONFIG_NAMES);
        let proj_cfg = get_first_existing(cwd.ancestors(), CONFIG_NAMES);

        let mut loader = ConfigLoader::new();
        for file in user_cfg.iter().chain(proj_cfg.iter()) {
            loader.file_optional(file)?;
        }
        let cfg = loader.load()?.config;

        Ok(cfg)
    }
}

fn get_first_existing(
    dirs: impl IntoIterator<Item = impl AsRef<Path>>,
    files: impl IntoIterator<Item = impl AsRef<Path>> + Clone,
) -> Option<PathBuf> {
    for dir in dirs.into_iter() {
        for file in files.clone().into_iter() {
            let path = dir.as_ref().join(file);
            let path = fs::canonicalize(path).expect("failed to canonicalize path");

            if fs::exists(&path).expect("failed to check path's existence") {
                return Some(path);
            }
        }
    }

    None
}

use std::{
    fs,
    path::{Path, PathBuf},
};

use schematic::ConfigLoader;
use thiserror::Error;

use crate::dirs::DIRS;
use crate::extract::config::{ExtractConfig, PartialExtractConfig};

const CONFIG_NAMES: &[&str] = &["sfd.yaml", "sfd.yml"];

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to load config: {0}")]
    Load(#[from] schematic::ConfigError),

    #[error("user config file not found: sfd.yaml or sfd.yml expected in {0}")]
    UserConfigNotFound(PathBuf),
}

#[derive(Debug, schematic::Config)]
pub struct Config {
    #[setting(nested)]
    pub extract: ExtractConfig,

    #[setting(skip)]
    pub root_path: Option<PathBuf>,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let cwd = std::env::current_dir().expect("failed to get CWD");

        let config_dir = DIRS.config_dir().to_path_buf();
        let user_cfg = get_first_existing([&config_dir], CONFIG_NAMES)
            .ok_or_else(|| ConfigError::UserConfigNotFound(config_dir.clone()))?;
        let proj_cfg = get_first_existing(cwd.ancestors(), CONFIG_NAMES);

        let mut loader = ConfigLoader::new();
        loader.file(&user_cfg)?;
        if let Some(ref proj_cfg) = proj_cfg {
            loader.file_optional(proj_cfg)?;
        }
        let mut cfg: Config = loader.load()?.config;
        cfg.root_path = Some(config_dir);

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

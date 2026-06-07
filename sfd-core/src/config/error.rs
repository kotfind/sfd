use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to load config: {0}")]
    Load(#[from] schematic::ConfigError),

    #[error("user config file not found: sfd.yaml or sfd.yml expected in {0}")]
    UserConfigNotFound(PathBuf),
}

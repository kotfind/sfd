use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to load config: {0}")]
    Load(#[from] schematic::ConfigError),

    #[error("project config not found: sfd.yaml or sfd.yml expected")]
    ProjConfigNotFound,
}

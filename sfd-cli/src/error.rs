#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("config error: {0}")]
    Config(#[from] sfd_core::error::ConfigError),

    #[error("core error: {0}")]
    Core(#[from] sfd_core::Error),
}

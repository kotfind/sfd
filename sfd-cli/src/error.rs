#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("config error: {0}")]
    Config(#[from] sfd_core::error::ConfigError),

    #[error("core error: {0}")]
    Core(#[from] sfd_core::Error),

    #[error("schema error: {0}")]
    Schema(#[from] schematic::ConfigError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("schema generation error: {0}")]
    SchemaRender(miette::Report),
}

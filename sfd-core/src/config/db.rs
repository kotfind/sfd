/// Database config.
#[derive(Debug, schematic::Config)]
#[config(rename_all = "snake_case")]
pub struct DbConfig {
    /// Busy timeout for SQLite connections, in seconds.
    #[setting(default = 10)]
    pub busy_timeout: u64,
}

/// Scanning config.
#[derive(Debug, schematic::Config)]
#[config(rename_all = "snake_case")]
pub struct ScanConfig {
    /// Patterns to exclude.
    pub exclude: Vec<String>,

    /// Respect `.gitignore` file.
    #[setting(default = true)]
    pub ignore_git: bool,

    /// Respect `.ignore` file.
    #[setting(default = true)]
    pub ignore_ignore: bool,

    /// Ignore hidden files.
    #[setting(default = true)]
    pub ignore_hidden: bool,
}

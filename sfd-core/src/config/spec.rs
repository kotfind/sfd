use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, schematic::Config)]
pub struct Config {
    #[setting(nested)]
    pub langs: HashMap<String, LangConfig>,

    #[setting(nested)]
    pub scan: ScanConfig,

    #[setting(skip)]
    pub root_path: Option<PathBuf>,
}

#[derive(Debug, schematic::Config)]
pub struct ScanConfig {
    pub exclude: Vec<String>,

    pub ignore_git: bool,

    pub ignore_ignore: bool,

    pub ignore_hidden: bool,
}

#[derive(Debug, schematic::Config)]
pub struct LangConfig {
    #[setting(validate = "crate::util::validate_absolute")]
    pub parser: PathBuf,

    pub exts: Vec<String>,

    pub query: String,
}

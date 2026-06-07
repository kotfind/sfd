use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, schematic::Config)]
pub struct Config {
    #[setting(nested)]
    pub langs: HashMap<String, LangConfig>,

    #[setting(skip)]
    pub root_path: Option<PathBuf>,
}

#[derive(Debug, schematic::Config)]
pub struct LangConfig {
    #[setting(validate = "crate::util::validate_absolute")]
    pub parser: PathBuf,

    pub exts: Vec<String>,

    pub query: String,
}

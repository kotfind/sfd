use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, schematic::Config)]
pub struct ExtractConfig {
    #[setting(nested)]
    pub langs: HashMap<String, LangConfig>,
}

#[derive(Debug, schematic::Config)]
pub struct LangConfig {
    #[setting(validate = "crate::util::validate_absolute")]
    pub parser: PathBuf,

    pub exts: Vec<String>,

    pub query: String,
}

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, schematic::Config)]
pub struct Config {
    #[setting(nested)]
    pub langs: HashMap<String, LangConfig>,

    #[setting(nested)]
    pub scan: ScanConfig,

    #[setting(nested)]
    pub vect: VectConfig,

    #[setting(skip)]
    pub root_path: Option<PathBuf>,
}

impl Config {
    pub fn root(&self) -> &Path {
        self.root_path
            .as_deref()
            .expect("root path is always specified")
    }
}

#[derive(Debug, schematic::Config)]
pub struct VectConfig {
    #[setting(default = 256)]
    pub max_len: usize,

    #[setting(nested)]
    pub ollama: OllamaConfig,
}

#[derive(Debug, schematic::Config)]
pub struct OllamaConfig {
    #[setting(default = "http://localhost:11434")]
    pub url: String,

    #[setting(default = "nomic-embed-text")]
    pub model: String,

    #[setting(default = 30)]
    pub timeout: u64,
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

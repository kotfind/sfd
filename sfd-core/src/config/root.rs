use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::models::lang_name::LangName;

#[allow(unused_imports)]
use super::ollama::PartialOllamaConfig;
use super::{
    lang::LangConfig,
    scan::{PartialScanConfig, ScanConfig},
    vect::{PartialVectConfig, VectConfig},
};

/// App config.
#[derive(Debug, schematic::Config)]
pub struct Config {
    #[setting(nested)]
    pub langs: HashMap<LangName, LangConfig>,

    #[setting(nested)]
    pub scan: ScanConfig,

    #[setting(nested)]
    pub vect: VectConfig,

    /// A root path of the project.
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

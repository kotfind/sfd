use std::{
    fs,
    path::{Path, PathBuf},
};

use schematic::ConfigLoader;

use crate::dirs::DIRS;

use super::error::Error;
use super::spec::Config;

const CONFIG_NAMES: &[&str] = &["sfd.yaml", "sfd.yml"];

impl Config {
    pub fn load() -> Result<Self, Error> {
        let cwd = std::env::current_dir().expect("failed to get CWD");

        let config_dir = DIRS.config_dir().to_path_buf();
        let user_cfg = get_first_existing([&config_dir], CONFIG_NAMES)
            .ok_or_else(|| Error::UserConfigNotFound(config_dir.clone()))?;
        let proj_cfg = get_first_existing(cwd.ancestors(), CONFIG_NAMES);

        let mut loader = ConfigLoader::new();
        loader.file(&user_cfg)?;
        if let Some(ref proj_cfg) = proj_cfg {
            loader.file_optional(proj_cfg)?;
        }
        let mut cfg: Config = loader.load()?.config;
        cfg.root_path = Some(config_dir);

        Ok(cfg)
    }
}

fn get_first_existing(
    dirs: impl IntoIterator<Item = impl AsRef<Path>>,
    files: impl IntoIterator<Item = impl AsRef<Path>> + Clone,
) -> Option<PathBuf> {
    for dir in dirs.into_iter() {
        for file in files.clone().into_iter() {
            let path = dir.as_ref().join(file);
            let path = fs::canonicalize(path).expect("failed to canonicalize path");

            if fs::exists(&path).expect("failed to check path's existence") {
                return Some(path);
            }
        }
    }

    None
}

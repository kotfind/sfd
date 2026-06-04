use std::sync::LazyLock;

use directories::ProjectDirs;

pub static DIRS: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("org", "kotfind", "sfd").expect("failed to init ProjectDirs")
});

#[derive(Debug, schematic::Config)]
pub struct ExtractConfig {
    #[setting(nested)]
    pub langs: LangsConfig,
}

#[derive(Debug, schematic::Config)]
pub struct LangsConfig {
    pub tree_sitter: String,

    pub query: String,
}

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// App config.
#[derive(Debug, schematic::Config)]
pub struct Config {
    #[setting(nested)]
    pub langs: HashMap<String, LangConfig>,

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

/// Vectorization config.
#[derive(Debug, schematic::Config)]
#[config(rename_all = "snake_case")]
pub struct VectConfig {
    /// A maximum length of a text to be sent to a model.
    ///
    /// If the text is longer, it's be truncated.
    #[setting(default = 256)]
    pub max_len: usize,

    #[setting(nested)]
    pub ollama: OllamaConfig,
}

/// Ollama config.
#[derive(Debug, schematic::Config)]
#[config(rename_all = "snake_case")]
pub struct OllamaConfig {
    /// Ollama API base url.
    #[setting(default = "http://localhost:11434")]
    pub url: String,

    /// Embedding model to use.
    #[setting(default = "nomic-embed-text")]
    pub model: String,

    /// Timeout for a single query.
    #[setting(default = 30)]
    pub timeout: u64,
}

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

/// Language config.
#[derive(Debug, schematic::Config)]
#[config(rename_all = "snake_case")]
pub struct LangConfig {
    /// An **absolute** path to a WASM tree-sitter parser.
    #[setting(validate = "crate::util::validate_absolute")]
    pub parser: PathBuf,

    /// A list of extensions, this kind of file, can be identified by.
    pub exts: Vec<String>,

    /// Tree-sitter query for extracting comment.
    ///
    /// A query should be written, so that each match give:
    /// - exactly one `@item` capture
    /// - one or more `@comment` captures
    ///
    /// Other captures are allowed, but are completely ignored.
    ///
    /// A sample query (for C):
    /// ```scm
    /// (
    ///   -- any number of comments
    ///   (comment)+ @comment
    ///   .
    ///   -- and right after that
    ///   [
    ///     -- function definition
    ///     (
    ///       function_definition declarator: (
    ///         function_declarator
    ///           declarator: _ @item
    ///       )
    ///     )
    ///     -- or struct definition
    ///     (struct_specifier name: _ @item)
    ///     -- or enum definition
    ///     (enum_specifier name: _ @item)
    ///   ]
    /// )
    /// ```
    pub query: String,
}

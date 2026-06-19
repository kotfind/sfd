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

    /// Timeout for a single query, in seconds.
    #[setting(default = 30.0)]
    pub timeout: f64,

    /// The maximal amount of Ollama API requests to run in parallel.
    #[setting(default = 1)]
    pub max_parallel: usize,
}

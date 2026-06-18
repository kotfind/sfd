use super::ollama::{OllamaConfig, PartialOllamaConfig};

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

mod lang;
pub mod load;
mod ollama;
mod root;
mod scan;
mod vect;

pub use lang::LangConfig;
pub use ollama::OllamaConfig;
pub use root::Config;
pub use scan::ScanConfig;
pub use vect::VectConfig;

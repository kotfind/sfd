pub mod error;
pub mod ollama;
mod prepare;
pub mod state;

pub use ollama::embed;
pub use state::VectContext;

pub mod client;
pub mod config;
pub mod context;
pub mod dirs;
pub mod error;
pub mod logic;
pub mod models;
pub mod result;
mod util;

pub use client::Client;
pub use error::Error;
pub use result::SearchResult;

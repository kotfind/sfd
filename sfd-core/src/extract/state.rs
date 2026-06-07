use std::collections::HashMap;
use std::fs;

use derive_more::Debug;
use thiserror::Error;
use tree_sitter::{Language, LanguageError, Query, QueryError, WasmError, WasmStore};
use wasmtime::Engine;

use crate::config::Config;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("failed to read wasm file: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to create wasm store: {0}")]
    Wasm(#[from] WasmError),

    #[error("failed to load language: {0}")]
    Language(#[from] LanguageError),

    #[error("failed to compile query: {0}")]
    Query(#[from] QueryError),
}

#[derive(Debug)]
pub struct LangState {
    pub name: String,

    pub exts: Vec<String>,

    pub lang: Language,

    pub query: Query,
}

#[derive(Debug)]
pub struct State {
    pub langs: HashMap<String, LangState>,

    #[debug(skip)]
    pub wasm_engine: Engine,

    #[debug(skip)]
    pub wasm_store: WasmStore,
}

impl State {
    pub fn new(config: &Config) -> Result<Self, StateError> {
        let wasm_engine = Engine::default();
        let mut wasm_store = WasmStore::new(&wasm_engine)?;
        let mut langs = HashMap::new();

        for (name, lang_cfg) in &config.extract.langs {
            let wasm_bytes = fs::read(&lang_cfg.parser)?;
            let lang = wasm_store.load_language(name, &wasm_bytes)?;
            let query = Query::new(&lang, &lang_cfg.query)?;
            langs.insert(name.clone(), LangState {
                name: name.clone(),
                exts: lang_cfg.exts.clone(),
                lang,
                query,
            });
        }

        Ok(Self {
            langs,
            wasm_engine,
            wasm_store,
        })
    }
}

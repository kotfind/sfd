use std::{collections::HashMap, fs, sync::Arc};

use derive_more::Debug;
use tree_sitter::{Language, Query, WasmStore};
use wasmtime::Engine;

use crate::{
    config::Config,
    error::ExtractError,
    logic::extract::{COMMENT_CAPTURE, ITEM_CAPTURE},
    models::lang_name::LangName,
};

#[derive(Debug)]
struct LangContextInner {
    #[allow(dead_code)] // for debug
    name: LangName,

    lang: Language,

    query: Query,
}

/// Language context.
#[derive(Clone, Debug)]
pub struct LangContext {
    inner: Arc<LangContextInner>,
}

impl LangContext {
    fn new(name: LangName, lang: Language, query: Query) -> Self {
        Self {
            inner: Arc::new(LangContextInner { name, lang, query }),
        }
    }

    pub fn lang(&self) -> &Language {
        &self.inner.lang
    }

    pub fn query(&self) -> &Query {
        &self.inner.query
    }
}

#[derive(Debug)]
struct ExtractContextInner {
    langs: HashMap<LangName, LangContext>,

    ext_to_lang: HashMap<String, LangName>,

    #[debug(skip)]
    wasm_engine: Engine,
}

/// Extraction context.
#[derive(Debug, Clone)]
pub struct ExtractContext {
    inner: Arc<ExtractContextInner>,
}

impl ExtractContext {
    pub(crate) fn wasm_engine(&self) -> &Engine {
        &self.inner.wasm_engine
    }

    pub fn get_lang(&self, name: &LangName) -> LangContext {
        self.inner
            .langs
            .get(name)
            .expect("language not found in state")
            .clone()
    }

    pub fn ext_to_lang(&self, ext: &str) -> Option<&LangName> {
        self.inner.ext_to_lang.get(ext)
    }

    pub fn new(config: &Config) -> Result<Self, ExtractError> {
        let wasm_engine = Engine::default();
        let mut wasm_store = WasmStore::new(&wasm_engine)?;
        let mut langs = HashMap::new();
        let mut ext_to_lang = HashMap::new();

        for (name, lang_cfg) in &config.langs {
            let wasm_bytes = fs::read(&lang_cfg.parser)?;
            let lang = wasm_store.load_language(name.as_str(), &wasm_bytes)?;
            let query = Query::new(&lang, &lang_cfg.query)?;

            let capture_names = query.capture_names();
            if !capture_names.contains(&COMMENT_CAPTURE) || !capture_names.contains(&ITEM_CAPTURE) {
                return Err(ExtractError::InvalidQuery);
            }

            for ext in &lang_cfg.exts {
                ext_to_lang.insert(ext.clone(), name.clone());
            }

            langs.insert(name.clone(), LangContext::new(name.clone(), lang, query));
        }

        Ok(Self {
            inner: Arc::new(ExtractContextInner {
                langs,
                ext_to_lang,
                wasm_engine,
            }),
        })
    }
}

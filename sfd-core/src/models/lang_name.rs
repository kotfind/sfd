use std::fmt;

use serde::{Deserialize, Serialize};

/// A name of a language.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LangName(String);

impl LangName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LangName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for LangName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for LangName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

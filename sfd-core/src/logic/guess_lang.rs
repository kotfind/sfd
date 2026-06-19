use std::{collections::HashMap, path::Path};

use crate::models::lang_name::LangName;

/// Guesses the language of a source file by its extension.
pub fn guess_lang(path: &Path, ext_to_lang: &HashMap<String, LangName>) -> Option<LangName> {
    let ext = path.extension()?.to_str()?;
    ext_to_lang.get(ext).cloned()
}

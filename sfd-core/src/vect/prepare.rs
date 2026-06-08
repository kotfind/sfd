use std::sync::LazyLock;

use regex::Regex;

static ALLOWED_CHAR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[^[:alnum:][:space:][:punct:]]").expect("valid regex"));

static SPACE_SEQ_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").expect("valid regex"));

use crate::config::spec::Config;

pub fn prepare(text: &str, config: &Config) -> String {
    let text = ALLOWED_CHAR_RE.replace_all(text, "");
    let text = SPACE_SEQ_RE.replace_all(text.trim(), " ");
    let mut text = text.to_string();
    text.truncate(config.vect.max_len);
    text
}

use std::sync::LazyLock;

use regex::Regex;

use crate::context::VectContext;

/// A list of chars NOT to filter out.
static ALLOWED_CHAR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[^[:alnum:][:space:][:punct:]]").expect("valid regex"));

/// Any sequence of whitespace chars.
static SPACE_SEQ_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").expect("valid regex"));

/// Prepares a text before sending to Ollama.
pub fn prepare(text: &str, ctx: VectContext) -> String {
    let text = ALLOWED_CHAR_RE.replace_all(text, "");
    let text = SPACE_SEQ_RE.replace_all(text.trim(), " ");
    let mut text = text.to_string();
    text.truncate(ctx.max_len());
    text
}

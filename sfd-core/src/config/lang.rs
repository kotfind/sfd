use std::path::PathBuf;

/// Language config.
#[derive(Debug, schematic::Config)]
#[config(rename_all = "snake_case")]
pub struct LangConfig {
    /// An **absolute** path to a WASM tree-sitter parser.
    #[setting(validate = "crate::util::validate_absolute")]
    pub parser: PathBuf,

    /// A list of extensions, this kind of file, can be identified by.
    pub exts: Vec<String>,

    /// Tree-sitter query for extracting comment.
    ///
    /// A query should be written, so that each match give:
    /// - exactly one `@item` capture
    /// - one or more `@comment` captures
    ///
    /// Other captures are allowed, but are completely ignored.
    ///
    /// A sample query (for C):
    /// ```scm
    /// (
    ///   -- any number of comments
    ///   (comment)+ @comment
    ///   .
    ///   -- and right after that
    ///   [
    ///     -- function definition
    ///     (
    ///       function_definition declarator: (
    ///         function_declarator
    ///           declarator: _ @item
    ///       )
    ///     )
    ///     -- or struct definition
    ///     (struct_specifier name: _ @item)
    ///     -- or enum definition
    ///     (enum_specifier name: _ @item)
    ///   ]
    /// )
    /// ```
    pub query: String,
}

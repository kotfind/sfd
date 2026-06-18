use std::ops::RangeInclusive;

use tree_sitter::{
    Node, Parser, QueryCapture, QueryCursor, QueryMatch, StreamingIterator, Tree, WasmStore,
};

use crate::{
    context::{ExtractContext, LangContext},
    error::ExtractError,
    models::{
        comment::Comment, ident::Ident, item::Item, source::Source, source_items::SourceItems,
        span::Span,
    },
};

pub const COMMENT_CAPTURE: &str = "comment";
pub const ITEM_CAPTURE: &str = "item";

/// Extracts all the [Item]s from a [Source].
pub fn extract(src: Source, ctx: &ExtractContext) -> Result<SourceItems, ExtractError> {
    let lang = ctx.get_lang(src.lang().ok_or_else(|| ExtractError::NoLang {
        path: src.path().to_path_buf(),
    })?);
    let tree = parse(src.clone(), &lang, ctx)?;

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(lang.query(), tree.root_node(), src.content().as_bytes());

    let capture_names = lang.query().capture_names();

    let mut items = Vec::new();
    while let Some(m) = matches.next() {
        items.push(match_to_item(m, capture_names, src.clone())?);
    }

    Ok(SourceItems::new(src, items))
}

/// Parses a [Source].
fn parse(src: Source, lang: &LangContext, ctx: &ExtractContext) -> Result<Tree, ExtractError> {
    let mut parser = Parser::new();
    let wasm_store = WasmStore::new(ctx.wasm_engine())?;
    parser.set_wasm_store(wasm_store)?;
    parser.set_language(lang.lang())?;

    let tree = parser
        .parse(src.content(), None)
        .expect("the language should've been provided");

    if tree.root_node().has_error() {
        return Err(ExtractError::SyntaxError);
    }

    Ok(tree)
}

/// Converts a match to an [Item].
fn match_to_item(
    m: &QueryMatch,
    capture_names: &[&str],
    src: Source,
) -> Result<Item, ExtractError> {
    let comment_nodes =
        get_named_captures(COMMENT_CAPTURE, 1..=usize::MAX, m.captures, capture_names)?;
    let comment = Comment::new(concat_node_text(&comment_nodes, src.clone())?);

    let item_node = &get_named_captures(ITEM_CAPTURE, 1..=1, m.captures, capture_names)?[0];
    let ident = Ident::new(get_node_text(item_node, src.clone())?);

    let offset = item_node.start_byte();
    let pos = item_node.start_position();
    let span = Span::new(src, offset, pos.row, pos.column);

    Ok(Item::new(comment, ident, span))
}

/// Gets all the captures with a `name` and asserts their amount is in a `range`.
fn get_named_captures<'tree>(
    name: &str,
    range: RangeInclusive<usize>,
    captures: &[QueryCapture<'tree>],
    capture_names: &[&str],
) -> Result<Vec<Node<'tree>>, ExtractError> {
    let nodes: Vec<Node<'tree>> = captures
        .iter()
        .filter(|c| capture_names[c.index as usize] == name)
        .map(|c| c.node)
        .collect();

    let count = nodes.len();
    if !range.contains(&count) {
        return Err(ExtractError::UnexpectedCaptureCount {
            name: name.to_owned(),
            expected: range,
            actual: count,
        });
    }

    Ok(nodes)
}

/// Gets [Node]'s text.
fn get_node_text<'tree>(node: &Node<'tree>, src: Source) -> Result<String, ExtractError> {
    node.utf8_text(src.content().as_bytes())
        .map_err(|_| ExtractError::NonUtf8)
        .map(|s| s.to_owned())
}

/// Gets and concatenates text from all the given [Node]s.
fn concat_node_text<'tree>(nodes: &[Node<'tree>], src: Source) -> Result<String, ExtractError> {
    nodes
        .iter()
        .map(|node| get_node_text(node, src.clone()))
        .collect::<Result<Vec<_>, _>>()
        .map(|texts| texts.join("\n"))
}

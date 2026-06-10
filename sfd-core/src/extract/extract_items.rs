use tree_sitter::{Node, Parser, QueryCapture, QueryCursor, QueryMatch, StreamingIterator, Tree};

use crate::{
    extract::{
        error::Error,
        state::{LangState, State},
    },
    models::{
        comment::Comment, ident::Ident, item::Item, source::Source, source_items::SourceItems,
        span::Span,
    },
};

pub const COMMENT_CAPTURE: &str = "comment";
pub const ITEM_CAPTURE: &str = "item";

pub fn extract(src: Source, state: &State) -> Result<SourceItems, Error> {
    let lang = state.get_lang(src.lang().ok_or(Error::NoLang)?);
    let tree = parse(src.clone(), &lang)?;

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(
        lang.query(),
        tree.root_node(),
        src.content().as_bytes(),
    );

    let capture_names = lang.query().capture_names();

    let mut items = Vec::new();
    while let Some(m) = matches.next() {
        items.push(match_to_item(m, capture_names, src.clone())?);
    }

    Ok(SourceItems::new(src, items))
}

fn parse(src: Source, lang: &LangState) -> Result<Tree, Error> {
    let mut parser = Parser::new();
    parser.set_language(lang.lang())?;

    let tree = parser
        .parse(src.content(), None)
        .expect("the language should've been provided");

    if tree.root_node().has_error() {
        return Err(Error::SyntaxError);
    }

    Ok(tree)
}

fn match_to_item(m: &QueryMatch, capture_names: &[&str], src: Source) -> Result<Item, Error> {
    let comment_node = get_single_capture_node(COMMENT_CAPTURE, m.captures, capture_names)?;
    let item_node = get_single_capture_node(ITEM_CAPTURE, m.captures, capture_names)?;

    let comment = Comment::new(get_node_text(&comment_node, src.clone())?);
    let ident = Ident::new(get_node_text(&item_node, src.clone())?);

    let offset = item_node.start_byte();
    let pos = item_node.start_position();
    let span = Span::new(src, offset, pos.row, pos.column);

    Ok(Item::new(comment, ident, span))
}

fn get_single_capture_node<'tree>(
    name: &str,
    captures: &[QueryCapture<'tree>],
    capture_names: &[&str],
) -> Result<Node<'tree>, Error> {
    let candidates: Vec<_> = captures
        .iter()
        .filter(|c| capture_names[c.index as usize] == name)
        .collect();

    let capture = match candidates.len() {
        0 => return Err(Error::MissingCapture(name.to_owned())),
        1 => candidates[0],
        2.. => return Err(Error::MultipleCaptures(name.to_owned())),
    };

    Ok(capture.node)
}

fn get_node_text<'tree>(node: &Node<'tree>, src: Source) -> Result<String, Error> {
    node.utf8_text(src.content().as_bytes())
        .map_err(|_| Error::NonUtf8)
        .map(|s| s.to_owned())
}

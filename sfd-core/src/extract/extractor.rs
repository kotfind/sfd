use tree_sitter::{Parser, QueryCursor, StreamingIterator};

use crate::{
    extract::{error::Error, state::LangState},
    models::{comment::Comment, ident::Ident, item::Item, source::Source, span::Span},
};

pub const COMMENT_CAPTURE: &str = "comment";
pub const ITEM_CAPTURE: &str = "item";

pub fn extract_file(src: &Source, lang: &LangState) -> Result<Vec<Item>, Error> {
    let mut parser = Parser::new();
    parser.set_language(&lang.inner.lang)?;

    let content = src.content();
    let tree = parser.parse(content, None).ok_or(Error::LangNotFound)?;

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&lang.inner.query, tree.root_node(), content.as_bytes());

    let mut items = Vec::new();
    while let Some(m) = matches.next() {
        let mut comment_node = None;
        let mut item_node = None;
        for c in m.captures {
            match lang.inner.query.capture_names()[c.index as usize] {
                COMMENT_CAPTURE => comment_node = Some(c.node),
                ITEM_CAPTURE => item_node = Some(c.node),
                _ => {}
            }
        }
        let comment_node = comment_node.ok_or(Error::MissingCapture(COMMENT_CAPTURE))?;
        let item_node = item_node.ok_or(Error::MissingCapture(ITEM_CAPTURE))?;

        let comment_text = comment_node
            .utf8_text(content.as_bytes())
            .map_err(|_| Error::NonUtf8)?;
        let item_text = item_node
            .utf8_text(content.as_bytes())
            .map_err(|_| Error::NonUtf8)?;

        let comment = Comment::new(comment_text);
        let ident = Ident::new(item_text);

        let offset = item_node.start_byte();
        let start = item_node.start_position();
        let span = Span::new(src.clone(), offset, start.row, start.column);

        items.push(Item::new(comment, ident, span));
    }

    Ok(items)
}

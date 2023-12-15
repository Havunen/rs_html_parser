use rs_html_parser_tokenizer_tokens::QuoteType;
use std::borrow::Cow;
use unicase_collections::unicase_btree_map::UniCaseBTreeMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Text = 1,

    OpenTag,
    CloseTag,

    ProcessingInstruction,

    Comment,
    CommentEnd, // Attribute,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub data: Cow<'a, str>,
    pub attrs: Option<UniCaseBTreeMap<Option<(Cow<'a, str>, QuoteType)>>>,
    pub kind: TokenKind,
    pub is_implied: bool,
}

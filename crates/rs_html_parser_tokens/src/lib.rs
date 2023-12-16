use unicase_collections::unicase_btree_map::UniCaseBTreeMap;
use rs_html_parser_tokenizer_tokens::QuoteType;

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
pub struct Token {
    pub data: Box<str>,
    pub attrs: Option<UniCaseBTreeMap<Option<(Box<str>, QuoteType)>>>,
    pub kind: TokenKind,
    pub is_implied: bool,
}

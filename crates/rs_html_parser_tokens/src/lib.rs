use rs_html_parser_tokenizer_tokens::QuoteType;
use std::collections::BTreeMap;
use unicase::UniCase;

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
    pub attrs: Option<BTreeMap<UniCase<Box<str>>, Option<(Box<str>, QuoteType)>>>,
    pub kind: TokenKind,
    pub is_implied: bool,
}

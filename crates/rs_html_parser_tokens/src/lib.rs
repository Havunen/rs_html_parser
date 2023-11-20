use rs_html_parser_tokenizer_tokens::QuoteType;
use std::collections::BTreeMap;

#[derive(Debug)]
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
    pub data: String,
    pub attrs: Option<BTreeMap<String, Option<(String, QuoteType)>>>,
    pub kind: TokenKind,
    pub is_implied: bool,
}

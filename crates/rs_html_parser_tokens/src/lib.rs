use std::collections::HashMap;
use rs_html_parser_tokenizer_tokens::QuoteType;

#[derive(Debug)]
pub enum TokenKind {
    Text = 1,

    OpenTag,
    CloseTag,

    ProcessingInstruction,

    Comment,
    CommentEnd
    // Attribute,
}

#[derive(Debug)]
pub struct Token {
    pub data: String,
    pub attrs: Option<HashMap<String, Option<(String, QuoteType)>>>,
    pub kind: TokenKind,
    pub is_implied: bool
}

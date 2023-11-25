use rs_html_parser_tokenizer_tokens::QuoteType;
use std::borrow::Cow;
use std::collections::BTreeMap;
use unicase::UniCase;

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
pub struct Token<'a> {
    pub data: Cow<'a, str>,
    pub attrs: Option<BTreeMap<UniCase<&'a str>, Option<(Cow<'a, str>, QuoteType)>>>,
    pub kind: TokenKind,
    pub is_implied: bool,
}

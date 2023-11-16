use std::collections::HashMap;

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
pub struct Token<'a> {
    pub data: &'a str,
    pub attrs: Option<&'a HashMap<&'a str, &'a str>>,
    pub kind: TokenKind,
    pub is_implied: bool
}

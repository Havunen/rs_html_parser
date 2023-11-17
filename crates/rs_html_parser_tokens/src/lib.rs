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
pub struct Token {
    pub data: String,
    pub attrs: Option<HashMap<String, String>>,
    pub kind: TokenKind,
    pub is_implied: bool
}

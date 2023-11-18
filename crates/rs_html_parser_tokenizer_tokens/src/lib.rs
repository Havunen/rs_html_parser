#[derive(Debug, Copy, Clone)]
pub enum QuoteType {
    NoValue = 0,
    Unquoted = 1,
    Single = 2,
    Double = 3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenizerTokenLocation {
    AttrData = 1,
    AttrEntity,
    AttrEnd,
    AttrName,

    CData,

    CloseTag,
    Comment,
    Declaration,
    OpenTagEnd,
    OpenTagName,

    ProcessingInstruction,
    SelfClosingTag,
    Text,
    TextEntity,

    End,
}

#[derive(Debug)]
pub struct TokenizerToken {
    pub start: usize,
    pub end: usize,
    pub location: TokenizerTokenLocation,
    pub code: u32,
    pub quote: QuoteType,
}

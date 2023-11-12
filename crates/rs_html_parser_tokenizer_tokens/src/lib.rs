#[derive(Debug)]
pub enum QuoteType {
    NoValue = 0,
    Unquoted = 1,
    Single = 2,
    Double = 3,
}

#[derive(Debug)]
pub enum TokenLocation {
    AttrData = 1,
    AttrEntity,
    AttrEnd,
    AttrName,

    CData,

    CloseTag,
    Comment,
    Declaration,
    End,
    OpenTagEnd,
    OpenTagName,

    ProcessingInstruction,
    SelfClosingTag,
    Text,
    TextEntity,
}

#[derive(Debug)]
pub struct Token {
    pub start: i32,
    pub end: i32,
    pub offset: i32,
    pub location: TokenLocation,
    pub code: u32,
    pub quote: QuoteType,
}

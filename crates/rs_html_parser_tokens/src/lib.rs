#[derive(Debug)]
pub enum TokenLocation {

}

#[derive(Debug)]
pub struct AttrToken {

}

#[derive(Debug)]
pub struct Token {
    pub tag: Box<str>,
    pub attrs: Option<Vec<AttrToken>>,
    pub start: i32,
    pub end: i32,
    pub offset: i32,
    pub location: TokenLocation,
    pub code: u32,
}

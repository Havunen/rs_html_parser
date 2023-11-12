#[derive(Debug)]
pub enum AstKind {

}

#[derive(Debug)]
pub struct AstAttrNode {

}

#[derive(Debug)]
pub struct AstNode {
    pub tag: str,
    pub attrs: Option<Vec<AstAttrNode>>,
    pub start: i32,
    pub end: i32,
    pub offset: i32,
    pub kind: AstKind,
    pub code: u32,
}

#[derive(Debug, PartialEq)]
pub struct Literal {
    pub kind: LiteralKind,
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    Bool(bool),
    Char(char),
    Integer(i64),
    Float(f64),
    String(String),
}

impl Literal {
    pub const fn new(kind: LiteralKind) -> Self {
        Self { kind }
    }
}

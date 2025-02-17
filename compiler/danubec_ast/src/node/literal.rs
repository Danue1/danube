pub struct Literal {
    pub kind: LiteralKind,
}

pub enum LiteralKind {
    Boolean(bool),
    Char(char),
    Float(f64),
    Integer(i64),
    String(String),
}

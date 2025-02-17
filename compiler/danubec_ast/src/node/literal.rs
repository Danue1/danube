use danubec_syntax::SyntaxNode;

pub struct Literal {
    pub syntax: SyntaxNode,
    pub kind: LiteralKind,
}

pub enum LiteralKind {
    Array(Vec<Literal>),
    Boolean(bool),
    Char(char),
    Float(f64),
    Integer(i64),
    String(String),
}

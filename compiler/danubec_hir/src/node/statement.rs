#[derive(Debug)]
pub struct Statement {
    pub kind: StatementKind,
}

#[derive(Debug)]
pub enum StatementKind {
    // Def(DefStatement),
    // Let(LetStatement),
    // Expr(ExprStatement),
    // Semi(SemiStatement),
    Empty,
}

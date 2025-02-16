pub struct Statement {
    pub kind: StatementKind,
}

pub enum StatementKind {
    // Def(DefStatement),
    // Let(LetStatement),
    // Expr(ExprStatement),
    // Semi(SemiStatement),
    Empty,
}

use super::{Literal, Path};

pub struct Pattern {
    pub kind: PatternKind,
}

pub enum PatternKind {
    Never,
    Placeholder,
    Rest,
    Path(Path),
    Tuple(Vec<Pattern>),
    Array(Vec<Pattern>),
    Literal(Literal),
    Or(Box<Pattern>, Box<Pattern>),
    Named(Path, Vec<(Path, Pattern)>),
    Unnamed(Path, Vec<Pattern>),
}

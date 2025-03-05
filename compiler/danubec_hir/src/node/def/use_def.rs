use crate::Path;
use danubec_symbol::Symbol;

#[derive(Debug)]
pub struct UseDef {
    // pub visibility: Visibility,
    pub tree: UseTree,
}

#[derive(Debug)]
pub struct UseTree {
    pub path: Path,
    pub kind: Option<UseTreeKind>,
}

#[derive(Debug)]
pub enum UseTreeKind {
    Barrel,
    Ident(UseTreeIdent),
    Nested(UseTreeNested),
}

#[derive(Debug)]
pub struct UseTreeIdent {
    pub alias: Symbol,
}

#[derive(Debug)]
pub struct UseTreeNested {
    pub trees: Vec<UseTree>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Import {
    pub path: Path,
    pub kind: Option<ImportKind>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImportKind {
    Alias(Symbol),
    Barrel,
}

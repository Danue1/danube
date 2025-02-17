use crate::Ident;

#[derive(Debug)]
pub struct StructDef {
    pub ident: Ident,
    pub kind: Option<StructKind>,
}

#[derive(Debug)]
pub enum StructKind {
    Named,
    Unnamed,
}

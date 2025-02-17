use danubec_symbol::Symbol;

#[derive(Debug)]
pub struct StructDef {
    pub ident: Symbol,
    pub kind: Option<StructKind>,
}

#[derive(Debug)]
pub enum StructKind {
    Named,
    Unnamed,
}

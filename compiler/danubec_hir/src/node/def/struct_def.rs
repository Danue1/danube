#[derive(Debug)]
pub struct StructDef {
    pub kind: Option<StructKind>,
}

#[derive(Debug)]
pub enum StructKind {
    Struct,
    Enum,
}

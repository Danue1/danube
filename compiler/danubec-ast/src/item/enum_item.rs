use crate::{Ident, NamedField, Ty, UnnamedField};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EnumItem {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EnumVariant {
    pub name: Ident,
    pub ty: Option<Ty>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EnumVariantKind {
    Unnamed(Vec<UnnamedField>),
    Named(Vec<NamedField>),
}

impl EnumItem {
    pub const fn new(name: Ident, variants: Vec<EnumVariant>) -> Self {
        Self { name, variants }
    }
}

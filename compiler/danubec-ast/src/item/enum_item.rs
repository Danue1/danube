use crate::{Ident, NamedField, UnnamedField};

#[derive(Debug, PartialEq)]
pub struct EnumItem {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, PartialEq)]
pub struct EnumVariant {
    pub name: Ident,
    pub kind: Option<EnumVariantKind>,
}

#[derive(Debug, PartialEq)]
pub enum EnumVariantKind {
    Unnamed(Vec<UnnamedField>),
    Named(Vec<NamedField>),
}

impl EnumItem {
    pub const fn new(name: Ident, variants: Vec<EnumVariant>) -> Self {
        Self { name, variants }
    }
}

impl EnumVariant {
    pub const fn new(name: Ident, kind: Option<EnumVariantKind>) -> Self {
        Self { name, kind }
    }
}

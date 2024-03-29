use crate::{Ident, NamedField, UnnamedField};

#[derive(Debug, PartialEq)]
pub struct StructItem {
    pub name: Ident,
    pub fields: Option<StructFields>,
}

#[derive(Debug, PartialEq)]
pub enum StructFields {
    Named(Vec<NamedField>),
    Unnamed(Vec<UnnamedField>),
}

impl StructItem {
    pub const fn new(name: Ident, fields: Option<StructFields>) -> Self {
        Self { name, fields }
    }
}

use crate::{NamedField, Path};

#[derive(Debug, PartialEq)]
pub struct Ty {
    pub kind: TypeKind,
}

#[derive(Debug, PartialEq)]
pub enum TypeKind {
    Unnamed(UnnamedTypeKind),
    Named(NamedTypeKind),
    Path(PathTypeKind),
}

#[derive(Debug, PartialEq)]
pub struct UnnamedTypeKind {
    pub fields: Vec<Ty>,
}

#[derive(Debug, PartialEq)]
pub struct NamedTypeKind {
    pub fields: Vec<NamedField>,
}

#[derive(Debug, PartialEq)]
pub struct PathTypeKind {
    pub path: Path,
}

impl Ty {
    pub const fn new(kind: TypeKind) -> Self {
        Self { kind }
    }
}

impl UnnamedTypeKind {
    pub const fn new(fields: Vec<Ty>) -> Self {
        Self { fields }
    }
}

impl NamedTypeKind {
    pub const fn new(fields: Vec<NamedField>) -> Self {
        Self { fields }
    }
}

impl PathTypeKind {
    pub const fn new(path: Path) -> Self {
        Self { path }
    }
}

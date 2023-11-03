pub mod const_item;
pub mod enum_item;
pub mod struct_item;

use crate::{Ident, Ty, Visibility};
pub use const_item::*;
pub use enum_item::*;
pub use struct_item::*;

#[derive(Debug, PartialEq)]
pub enum Item {
    Struct(StructItem),
    Enum(EnumItem),
    Const(ConstItem),
}

#[derive(Debug, PartialEq)]
pub struct NamedField {
    pub visibility: Option<Visibility>,
    pub name: Ident,
    pub ty: Ty,
}

#[derive(Debug, PartialEq)]
pub struct UnnamedField {
    pub visibility: Option<Visibility>,
    pub ty: Ty,
}

impl NamedField {
    pub const fn new(visibility: Option<Visibility>, name: Ident, ty: Ty) -> Self {
        Self {
            visibility,
            name,
            ty,
        }
    }
}

impl UnnamedField {
    pub const fn new(visibility: Option<Visibility>, ty: Ty) -> Self {
        Self { visibility, ty }
    }
}

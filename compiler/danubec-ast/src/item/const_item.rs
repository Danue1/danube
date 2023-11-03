use crate::{Expression, Ident, Ty};

#[derive(Debug, PartialEq)]
pub struct ConstItem {
    pub name: Ident,
    pub ty: Ty,
    pub value: Expression,
}

impl ConstItem {
    pub const fn new(name: Ident, ty: Ty, value: Expression) -> Self {
        Self { name, ty, value }
    }
}

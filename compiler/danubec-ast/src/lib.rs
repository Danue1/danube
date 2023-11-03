pub mod expression;
pub mod ident;
pub mod item;
pub mod literal;
pub mod path;
pub mod ty;
pub mod visibility;

pub use expression::*;
pub use ident::*;
pub use item::*;
pub use literal::*;
pub use path::*;
pub use ty::*;
pub use visibility::*;

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub items: Vec<Item>,
}

impl Ast {
    pub const fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}

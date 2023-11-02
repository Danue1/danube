pub mod ident;
pub mod item;
pub mod path;
pub mod ty;
pub mod visibility;

pub use ident::*;
pub use item::*;
pub use path::*;
pub use ty::*;
pub use visibility::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Ast {
    pub items: Vec<Item>,
}

impl Ast {
    pub const fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}

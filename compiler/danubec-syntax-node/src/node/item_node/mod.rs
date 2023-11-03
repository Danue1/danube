pub mod enum_item_node;
pub mod struct_item_node;

use super::SyntaxNode;
use danubec_ast::Item;
pub use enum_item_node::*;
pub use struct_item_node::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ItemNode {
    Struct(StructItemNode),
    Enum(EnumItemNode),
}

impl ItemNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(item) = StructItemNode::cast(node.clone()) {
            Some(Self::Struct(item))
        } else if let Some(item) = EnumItemNode::cast(node) {
            Some(Self::Enum(item))
        } else {
            None
        }
    }

    pub fn lower(self) -> Item {
        match self {
            Self::Struct(item) => Item::Struct(item.lower()),
            Self::Enum(item) => Item::Enum(item.lower()),
        }
    }
}

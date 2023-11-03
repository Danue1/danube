pub mod const_item_node;
pub mod enum_item_node;
pub mod struct_item_node;

use super::SyntaxNode;
pub use const_item_node::*;
use danubec_ast::Item;
pub use enum_item_node::*;
pub use struct_item_node::*;

#[derive(Debug, PartialEq)]
pub enum ItemNode {
    Struct(StructItemNode),
    Enum(EnumItemNode),
    Const(ConstItemNode),
}

impl ItemNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(item) = StructItemNode::cast(node.clone()) {
            Some(Self::Struct(item))
        } else if let Some(item) = EnumItemNode::cast(node.clone()) {
            Some(Self::Enum(item))
        } else if let Some(item) = ConstItemNode::cast(node) {
            Some(Self::Const(item))
        } else {
            None
        }
    }

    pub fn lower(self) -> Item {
        match self {
            Self::Struct(item) => Item::Struct(item.lower()),
            Self::Enum(item) => Item::Enum(item.lower()),
            Self::Const(item) => Item::Const(item.lower()),
        }
    }
}

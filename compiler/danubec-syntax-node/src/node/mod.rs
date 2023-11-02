pub mod ident_node;
pub mod item_node;
pub mod path_node;
pub mod type_node;
pub mod visibility_node;

use crate::SyntaxError;
use danubec_ast::Ast;
use danubec_syntax_kind::SyntaxKind;
pub use ident_node::*;
pub use item_node::*;
pub use path_node::*;
pub use rowan::GreenNode;
pub use type_node::*;
pub use visibility_node::*;

pub type SyntaxNode = rowan::SyntaxNode<danubec_syntax_kind::SyntaxKind>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AstNode(SyntaxNode);

macro_rules! impl_cast {
    ($node:ident, $kind:ident) => {
        impl $node {
            pub fn cast(node: SyntaxNode) -> Option<Self> {
                if node.kind() == SyntaxKind::$kind {
                    Some(Self(node))
                } else {
                    None
                }
            }
        }
    };
}

impl_cast!(AstNode, AST_NODE);
impl_cast!(StructItemNode, STRUCT_ITEM_NODE);
impl_cast!(NamedStructFieldNode, NAMED_STRUCT_FIELD_NODE);
impl_cast!(UnnamedStructFieldNode, UNNAMED_STRUCT_FIELD_NODE);
impl_cast!(TypeNode, TYPE_NODE);
impl_cast!(UnnamedTypeKindNode, UNNAMED_TYPE_KIND_NODE);
impl_cast!(PathTypeKindNode, PATH_TYPE_KIND_NODE);
impl_cast!(PathNode, PATH_NODE);
impl_cast!(IdentNode, IDENT_NODE);
impl_cast!(VisibilityNode, VISIBILITY_NODE);

impl AstNode {
    pub fn items(&self) -> impl Iterator<Item = ItemNode> + '_ {
        self.0.children().filter_map(ItemNode::cast)
    }

    pub fn lower(node: GreenNode) -> Result<Ast, SyntaxError> {
        let node = match Self::cast(SyntaxNode::new_root(node)) {
            Some(node) => node,
            None => {
                return Err(SyntaxError {
                    message: "Root node must be AST_NODE".to_string(),
                    line: 0,
                    column: 0,
                })
            }
        };
        dbg!(&node);

        Ok(Ast::new(node.items().map(ItemNode::lower).collect()))
    }
}

use crate::{ExpressionNode, IdentNode, SyntaxNode, TypeNode};
use danubec_ast::ConstItem;

#[derive(Debug, PartialEq)]
pub struct ConstItemNode(pub SyntaxNode);

impl ConstItemNode {
    pub fn name(&self) -> Option<IdentNode> {
        self.0.children().find_map(IdentNode::cast)
    }

    pub fn ty(&self) -> Option<TypeNode> {
        self.0.children().find_map(TypeNode::cast)
    }

    pub fn value(&self) -> Option<ExpressionNode> {
        self.0.children().find_map(ExpressionNode::cast)
    }

    pub fn lower(self) -> ConstItem {
        let name = match self.name() {
            Some(name) => name.lower(),
            None => panic!("ConstItemNode must have name"),
        };
        let ty = match self.ty() {
            Some(ty) => ty.lower(),
            None => panic!("ConstItemNode must have type"),
        };
        let value = match self.value() {
            Some(value) => value.lower(),
            None => panic!("ConstItemNode must have value"),
        };
        ConstItem::new(name, ty, value)
    }
}

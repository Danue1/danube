use crate::{LiteralNode, SyntaxNode};
use danubec_ast::{Expression, ExpressionKind};
use danubec_syntax_kind::SyntaxKind;

#[derive(Debug, PartialEq)]
pub enum ExpressionNode {
    Literal(SyntaxNode),
}

impl ExpressionNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::EXPRESSION_LITERAL_BOOL_NODE
            | SyntaxKind::EXPRESSION_LITERAL_CHAR_NODE
            | SyntaxKind::EXPRESSION_LITERAL_INTEGER_NODE
            | SyntaxKind::EXPRESSION_LITERAL_FLOAT_NODE
            | SyntaxKind::EXPRESSION_LITERAL_STRING_NODE => Some(Self::Literal(node)),
            _ => None,
        }
    }

    pub fn lower(self) -> Expression {
        match self {
            Self::Literal(node) => {
                let literal = match LiteralNode::cast(node) {
                    Some(literal) => literal,
                    None => panic!("ExpressionNode::Literal must have LiteralNode"),
                };
                Expression::new(ExpressionKind::Literal(literal.lower()))
            }
        }
    }
}

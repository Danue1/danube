use crate::SyntaxNode;
use danubec_ast::{Literal, LiteralKind};
use danubec_syntax_kind::SyntaxKind;

#[derive(Debug, PartialEq)]
pub enum LiteralNode {
    Bool(SyntaxNode),
    Char(SyntaxNode),
    Integer(SyntaxNode),
    Float(SyntaxNode),
    String(SyntaxNode),
}

impl LiteralNode {
    pub fn cast(node: SyntaxNode) -> Option<LiteralNode> {
        match node.kind() {
            SyntaxKind::EXPRESSION_LITERAL_BOOL_NODE => Some(Self::Bool(node)),
            SyntaxKind::EXPRESSION_LITERAL_CHAR_NODE => Some(Self::Char(node)),
            SyntaxKind::EXPRESSION_LITERAL_INTEGER_NODE => Some(Self::Integer(node)),
            SyntaxKind::EXPRESSION_LITERAL_FLOAT_NODE => Some(Self::Float(node)),
            SyntaxKind::EXPRESSION_LITERAL_STRING_NODE => Some(Self::String(node)),
            _ => None,
        }
    }

    pub fn lower(self) -> Literal {
        match self {
            Self::Bool(node) => {
                let value = match node.text().to_string().as_str() {
                    "true" => true,
                    "false" => false,
                    _ => panic!("LiteralNode::Bool must have value"),
                };
                Literal::new(LiteralKind::Bool(value))
            }
            Self::Char(node) => {
                let value = node.text().to_string();
                let len = value.len();
                if len != 3 {
                    panic!("LiteralNode::Char must have value");
                }
                let value = match value[1..len - 1].parse() {
                    Ok(value) => value,
                    Err(_) => panic!("LiteralNode::Char must have value"),
                };
                Literal::new(LiteralKind::Char(value))
            }
            Self::Integer(node) => {
                let value = node.text().to_string();
                let value = match value.parse() {
                    Ok(value) => value,
                    Err(_) => panic!("LiteralNode::Integer must have value"),
                };
                Literal::new(LiteralKind::Integer(value))
            }
            Self::Float(node) => {
                let value = node.text().to_string();
                let value = match value.parse() {
                    Ok(value) => value,
                    Err(_) => panic!("LiteralNode::Float must have value"),
                };
                Literal::new(LiteralKind::Float(value))
            }
            Self::String(node) => {
                let value = node.text().to_string();
                let value = value[1..value.len() - 1].to_string();
                Literal::new(LiteralKind::String(value))
            }
        }
    }
}

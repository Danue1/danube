use super::SyntaxNode;
use danubec_ast::Visibility;
use danubec_syntax_kind::SyntaxKind;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VisibilityNode(pub SyntaxNode);

impl VisibilityNode {
    pub fn lower(self) -> Visibility {
        match self.0.children().next() {
            Some(node) => match node.kind() {
                SyntaxKind::PUB_KEYWORD => Visibility::Public,
                _ => panic!("Invalid visibility"),
            },
            None => panic!("Invalid visibility"),
        }
    }
}

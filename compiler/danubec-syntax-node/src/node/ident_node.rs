use super::SyntaxNode;
use danubec_ast::Ident;
use danubec_syntax_kind::SyntaxKind;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct IdentNode(pub SyntaxNode);

impl IdentNode {
    pub fn name(&self) -> Option<String> {
        match self.0.kind() {
            SyntaxKind::IDENT_NODE => Some(self.0.text().to_string()),
            _ => None,
        }
    }

    pub fn lower(self) -> Ident {
        let name = match self.name() {
            Some(name) => name,
            None => panic!("IdentNode must have name"),
        };
        Ident::new(name)
    }
}

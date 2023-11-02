use super::SyntaxNode;
use crate::IdentNode;
use danubec_ast::Path;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PathNode(pub SyntaxNode);

impl PathNode {
    pub fn segments(&self) -> impl Iterator<Item = IdentNode> {
        self.0.children().filter_map(IdentNode::cast)
    }

    pub fn lower(self) -> Path {
        Path::new(self.segments().map(IdentNode::lower).collect())
    }
}

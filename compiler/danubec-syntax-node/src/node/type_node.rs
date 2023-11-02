use super::SyntaxNode;
use crate::PathNode;
use danubec_ast::{PathTypeKind, Ty, TypeKind, UnnamedTypeKind};
use danubec_syntax_kind::SyntaxKind;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TypeNode(pub SyntaxNode);

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TypeKindNode {
    Unnamed(Vec<SyntaxNode>),
    Path(SyntaxNode),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UnnamedTypeKindNode(pub SyntaxNode);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PathTypeKindNode(pub SyntaxNode);

impl TypeNode {
    pub fn kind(self) -> Option<TypeKindNode> {
        self.0.children().find_map(TypeKindNode::cast)
    }

    pub fn lower(self) -> Ty {
        let kind = match self.kind().map(TypeKindNode::lower) {
            Some(kind) => kind,
            None => panic!("TypeNode must have a kind"),
        };
        Ty::new(kind)
    }
}

impl TypeKindNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::UNNAMED_TYPE_KIND_NODE => Some(Self::Unnamed(
                node.children()
                    .filter(|node| node.kind() != SyntaxKind::TYPE_NODE)
                    .collect(),
            )),
            SyntaxKind::PATH_TYPE_KIND_NODE => Some(Self::Path(node)),
            _ => None,
        }
    }

    pub fn lower(self) -> TypeKind {
        match self {
            Self::Unnamed(nodes) => TypeKind::Unnamed(UnnamedTypeKind::new(
                nodes
                    .iter()
                    .flat_map(|node| {
                        node.children()
                            .filter_map(TypeNode::cast)
                            .map(TypeNode::lower)
                    })
                    .collect(),
            )),
            Self::Path(node) => {
                let path = match PathTypeKindNode::cast(node) {
                    Some(path) => path.lower(),
                    None => panic!("PathTypeKindNode must have a path"),
                };
                TypeKind::Path(path)
            }
        }
    }
}

impl PathTypeKindNode {
    pub fn path(&self) -> Option<PathNode> {
        self.0.children().find_map(PathNode::cast)
    }

    pub fn lower(self) -> PathTypeKind {
        let path = match self.path() {
            Some(path) => path.lower(),
            None => panic!("PathTypeKindNode must have a path"),
        };
        PathTypeKind::new(path)
    }
}

impl UnnamedTypeKindNode {
    pub fn lower(self) -> UnnamedTypeKind {
        UnnamedTypeKind::new(
            self.0
                .children()
                .filter_map(TypeNode::cast)
                .map(TypeNode::lower)
                .collect(),
        )
    }
}

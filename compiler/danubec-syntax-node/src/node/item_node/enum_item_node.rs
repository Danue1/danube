use crate::{IdentNode, NamedStructFieldNode, SyntaxNode, UnnamedStructFieldNode};
use danubec_ast::{EnumItem, EnumVariant, EnumVariantKind};
use danubec_syntax_kind::SyntaxKind;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EnumItemNode(pub SyntaxNode);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EnumVariantNode(pub SyntaxNode);

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EnumVariantKindNode {
    Unnamed(Vec<SyntaxNode>),
    Named(Vec<SyntaxNode>),
}

impl EnumItemNode {
    pub fn name(&self) -> Option<IdentNode> {
        self.0.children().find_map(IdentNode::cast)
    }

    pub fn variants(&self) -> impl Iterator<Item = EnumVariantNode> {
        self.0.children().filter_map(EnumVariantNode::cast)
    }

    pub fn lower(self) -> EnumItem {
        let name = match self.name() {
            Some(ident) => ident.lower(),
            None => panic!("EnumItemNode must have name"),
        };
        let variants = self.variants().map(EnumVariantNode::lower).collect();
        EnumItem::new(name, variants)
    }
}

impl EnumVariantNode {
    pub fn name(&self) -> Option<IdentNode> {
        self.0.children().find_map(IdentNode::cast)
    }

    pub fn kind(&self) -> Option<EnumVariantKindNode> {
        self.0.children().find_map(EnumVariantKindNode::cast)
    }

    pub fn lower(self) -> EnumVariant {
        let name = match self.name() {
            Some(ident) => ident.lower(),
            None => panic!("EnumVariantNode must have name"),
        };
        let kind = self.kind().map(EnumVariantKindNode::lower);
        EnumVariant::new(name, kind)
    }
}

impl EnumVariantKindNode {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            // { a: i32, b: i32 }
            SyntaxKind::NAMED_FIELDS_NODE => Some(Self::Named(
                node.children()
                    .filter(|node| node.kind() == SyntaxKind::NAMED_FIELD_NODE)
                    .collect(),
            )),
            // (i32, i32)
            SyntaxKind::UNNAMED_FIELDS_NODE => Some(Self::Unnamed(
                node.children()
                    .filter(|node| node.kind() == SyntaxKind::UNNAMED_FIELD_NODE)
                    .collect(),
            )),
            _ => None,
        }
    }

    pub fn lower(self) -> EnumVariantKind {
        match self {
            Self::Unnamed(nodes) => EnumVariantKind::Unnamed(
                nodes
                    .into_iter()
                    .filter_map(UnnamedStructFieldNode::cast)
                    .map(UnnamedStructFieldNode::lower)
                    .collect(),
            ),
            Self::Named(nodes) => EnumVariantKind::Named(
                nodes
                    .into_iter()
                    .filter_map(NamedStructFieldNode::cast)
                    .map(NamedStructFieldNode::lower)
                    .collect(),
            ),
        }
    }
}

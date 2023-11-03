use crate::{IdentNode, SyntaxNode, TypeNode, VisibilityNode};
use danubec_ast::{NamedField, StructFields, StructItem, UnnamedField};
use danubec_syntax_kind::SyntaxKind;

#[derive(Debug, PartialEq)]
pub struct StructItemNode(pub SyntaxNode);

#[derive(Debug, PartialEq)]
pub enum StructFieldsNode {
    Named(Vec<SyntaxNode>),
    Unnamed(Vec<SyntaxNode>),
}

#[derive(Debug, PartialEq)]
pub struct NamedStructFieldNode(pub SyntaxNode);

#[derive(Debug, PartialEq)]
pub struct UnnamedStructFieldNode(pub SyntaxNode);

impl StructItemNode {
    pub fn name(&self) -> Option<IdentNode> {
        self.0.children().find_map(IdentNode::cast)
    }

    pub fn fields(&self) -> Option<StructFieldsNode> {
        self.0.children().find_map(StructFieldsNode::cast)
    }

    pub fn lower(self) -> StructItem {
        let name = match self.name() {
            Some(ident) => ident.lower(),
            None => panic!("StructItemNode must have name"),
        };
        let fields = self.fields().map(StructFieldsNode::lower);
        StructItem::new(name, fields)
    }
}

impl StructFieldsNode {
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

    pub fn lower(self) -> StructFields {
        match self {
            Self::Named(nodes) => StructFields::Named(
                nodes
                    .into_iter()
                    .filter_map(NamedStructFieldNode::cast)
                    .map(NamedStructFieldNode::lower)
                    .collect(),
            ),
            Self::Unnamed(nodes) => StructFields::Unnamed(
                nodes
                    .into_iter()
                    .filter_map(UnnamedStructFieldNode::cast)
                    .map(UnnamedStructFieldNode::lower)
                    .collect(),
            ),
        }
    }
}

impl NamedStructFieldNode {
    pub fn visibility(&self) -> Option<VisibilityNode> {
        self.0.children().find_map(VisibilityNode::cast)
    }

    pub fn name(&self) -> Option<IdentNode> {
        self.0.children().find_map(IdentNode::cast)
    }

    pub fn ty(&self) -> Option<TypeNode> {
        self.0.children().find_map(TypeNode::cast)
    }

    pub fn lower(self) -> NamedField {
        let visibility = self.visibility().map(VisibilityNode::lower);
        let name = match self.name() {
            Some(ident) => ident.lower(),
            None => panic!("NamedFieldNode must have name"),
        };
        let ty = match self.ty() {
            Some(ty) => ty.lower(),
            None => panic!("NamedFieldNode must have type"),
        };
        NamedField::new(visibility, name, ty)
    }
}

impl UnnamedStructFieldNode {
    pub fn visibility(&self) -> Option<VisibilityNode> {
        self.0.children().find_map(VisibilityNode::cast)
    }

    pub fn ty(&self) -> Option<TypeNode> {
        self.0.children().find_map(TypeNode::cast)
    }

    pub fn lower(self) -> UnnamedField {
        let visibility = self.visibility().map(VisibilityNode::lower);
        let ty = match self.ty() {
            Some(ty) => ty.lower(),
            None => panic!("UnnamedFieldNode must have type"),
        };
        UnnamedField::new(visibility, ty)
    }
}

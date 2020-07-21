use crate::*;

#[derive(Debug, PartialEq)]
pub struct IdentNode {
  pub raw: String,
}

#[derive(Debug, PartialEq)]
pub struct ModuleNode {
  pub ident: Option<Positioned<IdentNode>>,
  pub item_list: Vec<Positioned<ItemNode>>,
}

#[derive(Debug, PartialEq)]
pub enum ItemNode {
  Struct(StructNode),
  Enum(EnumNode),
}

#[derive(Debug, PartialEq)]
pub struct StructNode {
  pub ident: Positioned<IdentNode>,
  pub fields: StructFieldsNode,
}

#[derive(Debug, PartialEq)]
pub struct EnumNode {
  pub ident: Positioned<IdentNode>,
  pub variant_list: Vec<Positioned<EnumVariantNode>>,
}

#[derive(Debug, PartialEq)]
pub enum StructFieldsNode {
  Unnamed(StructUnnamedFieldsNode),
  Named(StructNamedFieldsNode),
}

#[derive(Debug, PartialEq)]
pub struct StructUnnamedFieldsNode {
  pub node_list: Vec<Positioned<TypeNode>>,
}

#[derive(Debug, PartialEq)]
pub struct StructNamedFieldsNode {
  pub node_list: Vec<(Positioned<IdentNode>, Positioned<TypeNode>)>,
}

#[derive(Debug, PartialEq)]
pub struct EnumVariantNode {
  pub ident: Positioned<IdentNode>,
  pub ty: Option<Positioned<TypeNode>>,
}

#[derive(Debug, PartialEq)]
pub enum TypeNode {
  Array(Box<TypeArrayNode>),
  Ident(IdentNode),
}

#[derive(Debug, PartialEq)]
pub struct TypeArrayNode {
  pub ty: Positioned<TypeNode>,
  pub size: usize,
}

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
  Function(FunctionNode),
  TypeAlias(TypeAliasNode),
  TraitNode(TraitNode),
  Constant(ConstantNode),
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
pub struct FunctionNode {
  pub ident: Positioned<IdentNode>,
  pub return_type: Option<Positioned<TypeNode>>,
  pub argument_list: Vec<Positioned<FunctionArgumentNode>>,
  pub body: Positioned<String>, // TODO(Danuel): implement ExpressionNode
}

#[derive(Debug, PartialEq)]
pub struct TypeAliasNode {
  pub ident: Positioned<IdentNode>,
  pub ty: Positioned<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub struct TraitNode {
  pub ident: Positioned<IdentNode>,
  pub item_list: Vec<Positioned<TraitItemNode>>,
}

#[derive(Debug, PartialEq)]
pub struct ConstantNode {
  pub ident: Positioned<IdentNode>,
  pub ty: Positioned<TypeNode>,
  pub value: Positioned<ValueNode>, // TODO(Danuel): implement ExpressionNode
}

#[derive(Debug, PartialEq)]
pub enum ValueNode {
  Bool(bool),
  Char(char),
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
pub struct FunctionArgumentNode {
  pub ident: Positioned<IdentNode>,
  pub ty: Option<Positioned<TypeNode>>,
}

#[derive(Debug, PartialEq)]
pub enum TraitItemNode {
  Constant(TraitItemConstantNode),
  Function(TraitItemFunctionNode),
}

#[derive(Debug, PartialEq)]
pub struct TraitItemConstantNode {
  pub ident: Positioned<IdentNode>,
  pub ty: Positioned<TypeNode>,
  pub default_value: Option<Positioned<ValueNode>>,
}

#[derive(Debug, PartialEq)]
pub struct TraitItemFunctionNode {
  pub ident: Positioned<IdentNode>,
  pub return_type: Option<Positioned<TypeNode>>,
  pub argument_list: Vec<Positioned<FunctionArgumentNode>>,
  pub body: Option<Positioned<String>>, // TODO(Danuel): implement ExpressionNode
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

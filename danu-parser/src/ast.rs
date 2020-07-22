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
  Static(StaticNode),
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
  pub body: Vec<Positioned<StatementNode>>,
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
  pub value: Positioned<ValueNode>,
}

#[derive(Debug, PartialEq)]
pub struct StaticNode {
  pub ident: Positioned<IdentNode>,
  pub ty: Positioned<TypeNode>,
  pub value: Positioned<ValueNode>,
}

#[derive(Debug, PartialEq)]
pub enum ValueNode {
  Bool(bool),
  Char(char),
  Int(i128),
  Float(f64),
  String(String),
  Array(Vec<ValueNode>),
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
  pub body: Option<Vec<Positioned<StatementNode>>>,
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

#[derive(Debug, PartialEq)]
pub enum StatementNode {
  Let(LetNode),
  LetMut(LetMutNode),
  Expression(ExpressionNode),
  Conditional(StatementConditionalNode),
}

#[derive(Debug, PartialEq)]
pub struct LetNode {
  pub ident: Positioned<IdentNode>,
  pub ty: Option<Positioned<TypeNode>>,
  pub value: Positioned<ExpressionNode>,
}

#[derive(Debug, PartialEq)]
pub struct LetMutNode {
  pub ident: Positioned<IdentNode>,
  pub ty: Option<Positioned<TypeNode>>,
  pub value: Positioned<ExpressionNode>,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionNode {
  Value(ValueNode),
  Conditional(ExpressionConditionalNode),
}

type IfElse = (Positioned<ExpressionNode>, Vec<Positioned<StatementNode>>);

#[derive(Debug, PartialEq)]
pub struct StatementConditionalNode {
  pub condition: Box<Positioned<ExpressionNode>>,
  pub if_true: Vec<Positioned<StatementNode>>,
  pub if_else_if: Option<Vec<IfElse>>,
  pub if_false: Option<Vec<Positioned<StatementNode>>>,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionConditionalNode {
  pub condition: Box<Positioned<ExpressionNode>>,
  pub if_true: Vec<Positioned<StatementNode>>,
  pub if_else_if: Option<Vec<IfElse>>,
  pub if_false: Vec<Positioned<StatementNode>>,
}

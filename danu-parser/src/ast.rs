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
  Implement(ImplementNode),
  ImplementTrait(ImplementTraitNode),
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
  pub value: Positioned<LiteralValueNode>,
}

#[derive(Debug, PartialEq)]
pub struct StaticNode {
  pub ident: Positioned<IdentNode>,
  pub ty: Positioned<TypeNode>,
  pub value: Positioned<LiteralValueNode>,
}

#[derive(Debug, PartialEq)]
pub struct ImplementNode {
  pub target: Positioned<IdentNode>,
  pub item_list: Vec<Positioned<ImplementItemNode>>,
}

#[derive(Debug, PartialEq)]
pub struct ImplementTraitNode {
  pub target: Positioned<IdentNode>,
  pub trait_ident: Positioned<IdentNode>,
  pub item_list: Vec<Positioned<ImplementItemNode>>,
}

#[derive(Debug, PartialEq)]
pub enum LiteralValueNode {
  Bool(bool),
  Char(char),
  Int(i128),
  Float(f64),
  String(String),
  Array(Vec<LiteralValueNode>),
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
  pub default_value: Option<Positioned<LiteralValueNode>>,
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
  Loop(LoopNode),
  While(WhileNode),
  PatternMatch(PatternMatchNode),
}

#[derive(Debug, PartialEq)]
pub struct LetNode {
  pub pattern: Positioned<PatternNode>,
  pub ty: Option<Positioned<TypeNode>>,
  pub value: Positioned<ExpressionNode>,
}

#[derive(Debug, PartialEq)]
pub struct LetMutNode {
  pub pattern: Positioned<PatternNode>,
  pub ty: Option<Positioned<TypeNode>>,
  pub value: Positioned<ExpressionNode>,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionNode {
  Literal(LiteralValueNode),
  Conditional(ExpressionConditionalNode),
  Loop(LoopNode),
  While(WhileNode),
  PatternMatch(PatternMatchNode),
}

type ConditionalBranch = (Positioned<ExpressionNode>, Vec<Positioned<StatementNode>>);

#[derive(Debug, PartialEq)]
pub struct StatementConditionalNode {
  pub main_branch: Box<ConditionalBranch>,
  pub branch_list: Option<Vec<ConditionalBranch>>,
  pub other: Option<Vec<Positioned<StatementNode>>>,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionConditionalNode {
  pub main_branch: Box<ConditionalBranch>,
  pub branch_list: Option<Vec<ConditionalBranch>>,
  pub other: Vec<Positioned<StatementNode>>,
}

#[derive(Debug, PartialEq)]
pub struct LoopNode {
  pub body: Vec<Positioned<StatementNode>>,
}

#[derive(Debug, PartialEq)]
pub struct WhileNode {
  pub condition: Box<Positioned<ExpressionNode>>,
  pub body: Vec<Positioned<StatementNode>>,
}

type PatternBranch = (Vec<Positioned<PatternNode>>, Vec<Positioned<StatementNode>>);

#[derive(Debug, PartialEq)]
pub struct PatternMatchNode {
  pub condition: Box<Positioned<ExpressionNode>>,
  pub branch_list: Vec<PatternBranch>,
}

#[derive(Debug, PartialEq)]
pub enum PatternNode {
  Literal(LiteralValueNode),
  Path(PathNode),
  UnnamedStruct(UnnamedStructNode),
}

#[derive(Debug, PartialEq)]
pub enum ImplementItemNode {
  Constant(ConstantNode),
  Function(FunctionNode),
}

#[derive(Debug, PartialEq)]
pub struct PathNode {
  pub ident_list: Vec<Positioned<IdentNode>>,
}

#[derive(Debug, PartialEq)]
pub struct UnnamedStructNode {
  pub path: PathNode,
  pub field_list: Vec<Positioned<PatternNode>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentNode {
  pub raw: String,
}

#[derive(Debug, PartialEq)]
pub struct ModuleNode {
  pub ident: Option<IdentNode>,
  pub item_list: Vec<ItemNode>,
}

#[derive(Debug, PartialEq)]
pub enum ItemNode {
  Use(UseNode),
  Struct(StructNode),
  Enum(EnumNode),
  Function(FunctionNode),
  TypeAlias(TypeAliasNode),
  Trait(TraitNode),
  Constant(ConstantNode),
  Static(StaticNode),
  Implement(ImplementNode),
  ImplementTrait(ImplementTraitNode),
}

#[derive(Debug, PartialEq)]
pub struct UseNode {
  pub kind: UseKind<UseRootNode>,
}

#[derive(Debug, PartialEq)]
pub enum UseKind<T: Sized> {
  Unnested(T),
  Nested(Vec<T>),
}

#[derive(Debug, PartialEq)]
pub struct UseRootNode {
  pub ident: UseRootIdent,
  pub extra: UseKind<UseExtra>,
}

#[derive(Debug, PartialEq)]
pub enum UseRootIdent {
  Current,
  Super,
  Module,
  Ident(IdentNode),
}

#[derive(Debug, PartialEq)]
pub enum UseExtra {
  All,
  Ident(IdentNode, Option<IdentNode>),
  Extra(IdentNode, Box<UseKind<UseExtra>>),
}

#[derive(Debug, PartialEq)]
pub struct StructNode {
  pub ident: IdentNode,
  pub fields: StructFieldsNode,
}

#[derive(Debug, PartialEq)]
pub struct EnumNode {
  pub ident: IdentNode,
  pub variant_list: Vec<EnumVariantNode>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionNode {
  pub ident: IdentNode,
  pub argument_list: Vec<FunctionArgumentNode>,
  pub return_type: Option<TypeNode>,
  pub body: Vec<StatementNode>,
}

#[derive(Debug, PartialEq)]
pub struct TypeAliasNode {
  pub ident: IdentNode,
  pub ty: TypeNode,
}

#[derive(Debug, PartialEq)]
pub struct TraitNode {
  pub ident: IdentNode,
  pub item_list: Vec<TraitItemNode>,
}

#[derive(Debug, PartialEq)]
pub struct ConstantNode {
  pub ident: IdentNode,
  pub ty: TypeNode,
  pub value: ExpressionNode,
}

#[derive(Debug, PartialEq)]
pub struct StaticNode {
  pub ident: IdentNode,
  pub ty: TypeNode,
  pub value: LiteralValueNode,
}

#[derive(Debug, PartialEq)]
pub struct ImplementNode {
  pub target: IdentNode,
  pub item_list: Vec<ImplementItemNode>,
}

#[derive(Debug, PartialEq)]
pub struct ImplementTraitNode {
  pub target: IdentNode,
  pub trait_ident: IdentNode,
  pub item_list: Vec<ImplementItemNode>,
}

#[derive(Debug, PartialEq)]
pub enum LiteralValueNode {
  Bool(bool),
  Char(char),
  Int(i64),
  Float(f64),
  String(String),
}

#[derive(Debug, PartialEq)]
pub enum StructFieldsNode {
  Unnamed(StructUnnamedFieldsNode),
  Named(StructNamedFieldsNode),
}

#[derive(Debug, PartialEq)]
pub struct StructUnnamedFieldsNode {
  pub node_list: Vec<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub struct StructNamedFieldsNode {
  pub node_list: Vec<(IdentNode, TypeNode)>,
}

#[derive(Debug, PartialEq)]
pub struct EnumVariantNode {
  pub ident: IdentNode,
  pub ty: Option<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionArgumentNode {
  pub ident: IdentNode,
  pub ty: TypeNode,
}

#[derive(Debug, PartialEq)]
pub enum TraitItemNode {
  Constant(TraitItemConstantNode),
  Function(TraitItemFunctionNode),
}

#[derive(Debug, PartialEq)]
pub struct TraitItemConstantNode {
  pub ident: IdentNode,
  pub ty: TypeNode,
  pub default_value: Option<LiteralValueNode>,
}

#[derive(Debug, PartialEq)]
pub struct TraitItemFunctionNode {
  pub ident: IdentNode,
  pub return_type: Option<TypeNode>,
  pub argument_list: Vec<FunctionArgumentNode>,
  pub body: Option<Vec<StatementNode>>,
}

#[derive(Debug, PartialEq)]
pub enum TypeNode {
  Array(Box<TypeArrayNode>),
  Ident(IdentNode),
}

#[derive(Debug, PartialEq)]
pub struct TypeArrayNode {
  pub ty: TypeNode,
  pub size: usize,
}

#[derive(Debug, PartialEq)]
pub enum StatementNode {
  Constant(ConstantNode),
  Static(StaticNode),
  Let(LetNode),
  LetMut(LetMutNode),
  Conditional(StatementConditionalNode),
  Loop(LoopNode),
  While(WhileNode),
  PatternMatch(PatternMatchNode),
  Expression(ExpressionNode),
}

#[derive(Debug, PartialEq)]
pub struct LetNode {
  pub pattern: PatternNode,
  pub ty: Option<TypeNode>,
  pub value: ExpressionNode,
}

#[derive(Debug, PartialEq)]
pub struct LetMutNode {
  pub pattern: PatternNode,
  pub ty: Option<TypeNode>,
  pub value: ExpressionNode,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionNode {
  Path(PathNode),
  Conditional(ExpressionConditionalNode),
  Loop(LoopNode),
  While(WhileNode),
  PatternMatch(PatternMatchNode),
  Break,
  Continue,
  Return(ReturnNode),
  Literal(LiteralValueNode),
  Array(Vec<ExpressionNode>),
  FunctionCall(FunctionCallNode),
  Index(IndexNode),
  BinaryOperator(BinaryOperatorNode),
  UnaryOperator(UnaryOperatorNode),
  Field(ExpressionFieldNode),
}

pub type ConditionalBranch = (ExpressionNode, Vec<StatementNode>);

#[derive(Debug, PartialEq)]
pub struct StatementConditionalNode {
  pub main_branch: Box<ConditionalBranch>,
  pub branch_list: Vec<ConditionalBranch>,
  pub other: Option<Vec<StatementNode>>,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionConditionalNode {
  pub main_branch: Box<ConditionalBranch>,
  pub branch_list: Vec<ConditionalBranch>,
  pub other: Vec<StatementNode>,
}

#[derive(Debug, PartialEq)]
pub struct LoopNode {
  pub body: Vec<StatementNode>,
}

#[derive(Debug, PartialEq)]
pub struct WhileNode {
  pub condition: Box<ExpressionNode>,
  pub body: Vec<StatementNode>,
}

pub type PatternBranch = (Vec<PatternNode>, Vec<StatementNode>);

#[derive(Debug, PartialEq)]
pub struct PatternMatchNode {
  pub condition: Box<ExpressionNode>,
  pub branch_list: Vec<PatternBranch>,
}

#[derive(Debug, PartialEq)]
pub struct ReturnNode {
  pub value: Option<Box<ExpressionNode>>,
}

#[derive(Debug, PartialEq)]
pub enum PatternNode {
  UnnamedStruct(UnnamedStructNode),
  NamedStruct(NamedStructNode),
  Literal(LiteralValueNode),
  Path(PathNode),
}

#[derive(Debug, PartialEq)]
pub struct FunctionCallNode {
  pub argument_list: Vec<ExpressionNode>,
}

#[derive(Debug, PartialEq)]
pub struct IndexNode {
  pub array: Box<ExpressionNode>,
  pub index: Box<ExpressionNode>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryOperatorNode {
  pub kind: BinaryOperatorKind,
  pub left: Box<ExpressionNode>,
  pub right: Box<ExpressionNode>,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperatorKind {
  Add,                // +
  Sub,                // -
  Mul,                // *
  Div,                // /
  Mod,                // %
  And,                // &&
  Or,                 // ||
  BitXor,             // ^
  BitAnd,             // &
  BitOr,              // |
  BitLeft,            // <<
  BitRight,           // >>
  Equal,              // ==
  NotEqual,           // !=
  LessThan,           // <
  LessThanOrEqual,    // <=
  GreaterThan,        // >
  GreaterThanOrEqual, // >=
}

#[derive(Debug, PartialEq)]
pub struct UnaryOperatorNode {
  pub kind: UnaryOperatorKind,
  pub value: Box<ExpressionNode>,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperatorKind {
  Not,
  Negative,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionFieldNode {
  pub left: Box<ExpressionNode>,
  pub right: Box<IdentNode>,
}

#[derive(Debug, PartialEq)]
pub enum ImplementItemNode {
  Constant(ConstantNode),
  Function(FunctionNode),
}

#[derive(Debug, PartialEq)]
pub struct PathNode {
  pub ident_list: Vec<IdentNode>,
}

#[derive(Debug, PartialEq)]
pub struct UnnamedStructNode {
  pub path: Option<PathNode>,
  pub field_list: Vec<PatternNode>,
}

#[derive(Debug, PartialEq)]
pub struct NamedStructNode {
  pub path: Option<PathNode>,
  pub field_list: Vec<FieldNode>,
}

#[derive(Debug, PartialEq)]
pub struct FieldNode {
  pub ident: IdentNode,
  pub pattern: Option<PatternNode>,
}

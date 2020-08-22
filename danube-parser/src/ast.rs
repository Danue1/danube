#[derive(Debug, PartialEq, Clone)]
pub enum Visibility {
  Public,
  Super,
  Module,
  Restricted(PathNode),
}

#[derive(Debug, PartialEq, Clone)]
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
  pub visibility: Option<Visibility>,
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
  pub visibility: Option<Visibility>,
  pub ident: IdentNode,
  pub generic: Option<GenericNode>,
  pub fields: StructFieldsNode,
}

#[derive(Debug, PartialEq)]
pub struct GenericNode {
  pub path: PathNode,
  pub trait_list: Vec<PathNode>,
}

#[derive(Debug, PartialEq)]
pub struct EnumNode {
  pub visibility: Option<Visibility>,
  pub ident: IdentNode,
  pub generic: Option<GenericNode>,
  pub variant_list: Vec<EnumVariantNode>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionNode {
  pub visibility: Option<Visibility>,
  pub is_async: bool,
  pub ident: IdentNode,
  pub generic: Option<GenericNode>,
  pub argument_list: Vec<FunctionArgumentNode>,
  pub return_type: Option<TypeNode>,
  pub body: Vec<StatementNode>,
}

#[derive(Debug, PartialEq)]
pub struct TypeAliasNode {
  pub visibility: Option<Visibility>,
  pub ident: IdentNode,
  pub ty: TypeNode,
}

#[derive(Debug, PartialEq)]
pub struct TraitNode {
  pub visibility: Option<Visibility>,
  pub ident: IdentNode,
  pub generic: Option<GenericNode>,
  pub item_list: Vec<TraitItemNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantNode {
  pub visibility: Option<Visibility>,
  pub ident: IdentNode,
  pub ty: TypeNode,
  pub value: ExpressionNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StaticNode {
  pub visibility: Option<Visibility>,
  pub ident: IdentNode,
  pub ty: TypeNode,
  pub value: LiteralValueNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignSugarNode {
  pub kind: AssignSugarKind,
  pub ident: IdentNode,
  pub value: ExpressionNode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssignSugarKind {
  AddAssign, // +=
  SubAssign, // -=
  ExpAssign, // **=
  MulAssign, // *=
  DivAssign, // /=
  ModAssign, // %=
  AndAssign, // &&=
  OrAssign,  // ||=

  BitAndAssign,   // &=
  BitOrAssign,    // |=
  BitXorAssign,   // ^=
  BitLeftAssign,  // <<=
  BitRightAssign, // >>=
}

#[derive(Debug, PartialEq)]
pub struct ImplementNode {
  pub visibility: Option<Visibility>,
  pub generic: Option<GenericNode>,
  pub target: PathNode,
  pub item_list: Vec<ImplementItemNode>,
}

#[derive(Debug, PartialEq)]
pub struct ImplementTraitNode {
  pub visibility: Option<Visibility>,
  pub target: PathNode,
  pub target_generic: Option<GenericNode>,
  pub trait_ident: PathNode,
  pub generic: Option<GenericNode>,
  pub item_list: Vec<ImplementItemNode>,
}

#[derive(Debug, PartialEq, Clone)]
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
  pub immutablity: Immutablity,
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
  pub is_async: bool,
  pub ident: IdentNode,
  pub generic: Option<GenericNode>,
  pub argument_list: Vec<FunctionArgumentNode>,
  pub return_type: Option<TypeNode>,
  pub body: Option<Vec<StatementNode>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeNode {
  Array(Immutablity, TypeArrayNode),
  Tuple(Immutablity, Vec<TypeNode>),
  Path(Immutablity, PathNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeArrayNode {
  pub ty: Box<TypeNode>,
  pub size: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Immutablity {
  Yes,
  Nope,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementNode {
  Constant(ConstantNode),
  Static(StaticNode),
  AssignSugar(AssignSugarNode),
  Let(LetNode),
  Expression(ExpressionNode),
  Semicolon,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetNode {
  pub immutablity: Immutablity,
  pub pattern: PatternNode,
  pub ty: Option<TypeNode>,
  pub value: ExpressionNode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionNode {
  Path(PathNode),
  Conditional(ConditionalNode),
  Loop(LoopNode),
  While(WhileNode),
  For(ForNode),
  PatternMatch(PatternMatchNode),
  Break,
  Continue,
  Return(ReturnNode),
  Literal(LiteralValueNode),
  Array(Vec<ExpressionNode>),
  Tuple(TupleNode),
  Index(IndexNode),
  UnaryOperator(UnaryOperatorNode),
  InfixOperator(InfixOperatorNode),
  Await(Box<ExpressionNode>),
  Try(Box<ExpressionNode>),
  Field(ExpressionFieldNode),
  Struct(ExpressionStructNode),
  Block(Vec<StatementNode>),
}

pub type ConditionalBranch = (ExpressionNode, Vec<StatementNode>);

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionalNode {
  pub main_branch: Box<ConditionalBranch>,
  pub branch_list: Vec<ConditionalBranch>,
  pub other: Option<Vec<StatementNode>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoopNode {
  pub body: Vec<StatementNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileNode {
  pub condition: Box<ExpressionNode>,
  pub body: Vec<StatementNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForNode {
  pub immutablity: Immutablity,
  pub pattern: PatternNode,
  pub iteration: Box<ExpressionNode>,
  pub body: Vec<StatementNode>,
}

pub type PatternBranch = (Vec<PatternNode>, Vec<StatementNode>);

#[derive(Debug, PartialEq, Clone)]
pub struct PatternMatchNode {
  pub condition: Box<ExpressionNode>,
  pub branch_list: Vec<PatternBranch>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnNode {
  pub value: Option<Box<ExpressionNode>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternNode {
  Placeholder,
  UnnamedStruct(UnnamedStructNode),
  NamedStruct(NamedStructNode),
  Literal(LiteralValueNode),
  Path(PathNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleNode {
  pub field: Option<Box<ExpressionNode>>,
  pub node_list: Vec<(Option<IdentNode>, ExpressionNode)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexNode {
  pub array: Box<ExpressionNode>,
  pub index: Box<ExpressionNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfixOperatorNode {
  pub kind: InfixOperatorKind,
  pub lhs: Box<ExpressionNode>,
  pub rhs: Box<ExpressionNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InfixOperatorKind {
  Add,      // +
  Sub,      // -
  Mul,      // *
  Div,      // /
  Mod,      // %
  BitAnd,   // &
  BitOr,    // |
  BitXor,   // ^
  BitLeft,  // <<
  BitRight, // >>

  Equal,              // ==
  NotEqual,           // !=
  GreaterThan,        // >
  LessThan,           // <
  GreaterThanOrEqual, // >=
  LessThanOrEqual,    // <=

  And, // &&
  Or,  // ||

  ChainArrow, // |>
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOperatorNode {
  pub kind: UnaryOperatorKind,
  pub value: Box<ExpressionNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperatorKind {
  Not,
  Negative,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionFieldNode {
  pub lhs: Box<ExpressionNode>,
  pub rhs: Box<IdentNode>,
}

pub type ExpressionStructField = (IdentNode, Option<ExpressionNode>);

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStructNode {
  pub path: Option<PathNode>,
  pub field_list: Vec<ExpressionStructField>,
  pub rest: Option<Box<ExpressionNode>>,
}

#[derive(Debug, PartialEq)]
pub enum ImplementItemNode {
  Constant(ConstantNode),
  Function(FunctionNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct PathNode {
  pub ident_list: Vec<IdentNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnnamedStructNode {
  pub path: Option<PathNode>,
  pub field_list: Vec<PatternNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamedStructNode {
  pub path: Option<PathNode>,
  pub field_list: Vec<FieldNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldNode {
  pub ident: IdentNode,
  pub pattern: Option<PatternNode>,
}

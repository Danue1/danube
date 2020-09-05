#[derive(Debug)]
pub struct ProgramNode {
  pub feature_list: Vec<FeatureNode>,
  pub module: ModuleNode,
}

#[derive(Debug, PartialEq)]
pub struct FeatureNode {
  pub name: IdentNode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VisibilityKind {
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

#[derive(Debug, PartialEq, Clone)]
pub struct ItemNode {
  pub attribute_list: Vec<AttributeNode>,
  pub kind: ItemKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AttributeNode {
  pub path: PathNode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ItemKind {
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

#[derive(Debug, PartialEq, Clone)]
pub struct UseNode {
  pub visibility: Option<VisibilityKind>,
  pub kind: UseKind<UseRootNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UseKind<T: Sized> {
  Unnested(T),
  Nested(Vec<T>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct UseRootNode {
  pub ident: UseRootIdentKind,
  pub extra: UseKind<UseExtraKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UseRootIdentKind {
  Current,
  Super,
  Module,
  Ident(IdentNode),
}

#[derive(Debug, PartialEq, Clone)]
pub enum UseExtraKind {
  All,
  Ident(IdentNode, Option<IdentNode>),
  Extra(IdentNode, Box<UseKind<UseExtraKind>>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructNode {
  pub visibility: Option<VisibilityKind>,
  pub ident: IdentNode,
  pub generic: Option<GenericNode>,
  pub fields: StructFieldsKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GenericNode {
  pub path: PathNode,
  pub trait_list: Vec<PathNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumNode {
  pub visibility: Option<VisibilityKind>,
  pub ident: IdentNode,
  pub generic: Option<GenericNode>,
  pub variant_list: Vec<EnumVariantNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
  pub visibility: Option<VisibilityKind>,
  pub is_async: bool,
  pub ident: IdentNode,
  pub generic: Option<GenericNode>,
  pub argument_list: Vec<FunctionArgumentNode>,
  pub return_type: Option<TypeKind>,
  pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAliasNode {
  pub visibility: Option<VisibilityKind>,
  pub ident: IdentNode,
  pub ty: TypeKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitNode {
  pub visibility: Option<VisibilityKind>,
  pub ident: IdentNode,
  pub generic: Option<GenericNode>,
  pub item_list: Vec<TraitItemKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantNode {
  pub visibility: Option<VisibilityKind>,
  pub ident: IdentNode,
  pub ty: TypeKind,
  pub value: ExpressionKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StaticNode {
  pub visibility: Option<VisibilityKind>,
  pub ident: IdentNode,
  pub ty: TypeKind,
  pub value: LiteralValueKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompoundAssignNode {
  pub kind: CompoundAssignKind,
  pub lhs: ExpressionKind,
  pub rhs: ExpressionKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CompoundAssignKind {
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

#[derive(Debug, PartialEq, Clone)]
pub struct ImplementNode {
  pub visibility: Option<VisibilityKind>,
  pub generic: Option<GenericNode>,
  pub target: PathNode,
  pub item_list: Vec<ImplementItemKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImplementTraitNode {
  pub visibility: Option<VisibilityKind>,
  pub target: PathNode,
  pub target_generic: Option<GenericNode>,
  pub trait_ident: PathNode,
  pub generic: Option<GenericNode>,
  pub item_list: Vec<ImplementItemKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValueKind {
  Bool(bool),
  Char(char),
  Int(i64),
  Float(f64),
  String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum StructFieldsKind {
  Unnamed(StructUnnamedFieldsNode),
  Named(StructNamedFieldsNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructUnnamedFieldsNode {
  pub node_list: Vec<TypeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructNamedFieldsNode {
  pub node_list: Vec<(IdentNode, TypeKind)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumVariantNode {
  pub ident: IdentNode,
  pub ty: Option<TypeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionArgumentNode {
  pub ident: IdentNode,
  pub immutablity: ImmutablityKind,
  pub ty: TypeKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TraitItemKind {
  Constant(TraitItemConstantNode),
  Function(TraitItemFunctionNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitItemConstantNode {
  pub ident: IdentNode,
  pub ty: TypeKind,
  pub default_value: Option<LiteralValueKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitItemFunctionNode {
  pub is_async: bool,
  pub ident: IdentNode,
  pub generic: Option<GenericNode>,
  pub argument_list: Vec<FunctionArgumentNode>,
  pub return_type: Option<TypeKind>,
  pub block: Option<BlockNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
  Array(ImmutablityKind, TypeArrayNode),
  Tuple(ImmutablityKind, Vec<TypeKind>),
  Path(ImmutablityKind, PathNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeArrayNode {
  pub ty: Box<TypeKind>,
  pub size: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImmutablityKind {
  Yes,
  Nope,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
  Item(Box<ItemNode>),
  CompoundAssign(Box<CompoundAssignNode>),
  Let(Box<LetNode>),
  ExpressionKind(ExpressionKind),
  Semicolon,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetNode {
  pub immutablity: ImmutablityKind,
  pub pattern: PatternKind,
  pub ty: Option<TypeKind>,
  pub value: Option<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionKind {
  Path(PathNode),
  Conditional(ConditionalNode),
  Loop(LoopNode),
  While(WhileNode),
  For(ForNode),
  PatternMatch(PatternMatchNode),
  Closure(ClosureNode),
  Break,
  Continue,
  Return(ReturnNode),
  Literal(LiteralValueKind),
  Array(Vec<ExpressionKind>),
  Tuple(TupleNode),
  Index(IndexNode),
  UnaryOperator(UnaryOperatorNode),
  InfixOperator(InfixOperatorNode),
  Await(Box<ExpressionKind>),
  Try(Box<ExpressionKind>),
  Field(ExpressionKindFieldNode),
  Struct(ExpressionKindStructNode),
  Block(BlockNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionNode {
  pub pattern: Option<(ImmutablityKind, PatternKind)>,
  pub value: Box<ExpressionKind>,
}

pub type ConditionalBranch = (ConditionNode, BlockNode);

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionalNode {
  pub main_branch: ConditionalBranch,
  pub branch_list: Vec<ConditionalBranch>,
  pub other: Option<BlockNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoopNode {
  pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileNode {
  pub condition: ConditionNode,
  pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForNode {
  pub immutablity: ImmutablityKind,
  pub pattern: PatternKind,
  pub iteration: Box<ExpressionKind>,
  pub block: BlockNode,
}

pub type PatternBranch = (Vec<PatternKind>, BlockNode);

#[derive(Debug, PartialEq, Clone)]
pub struct PatternMatchNode {
  pub condition: Box<ExpressionKind>,
  pub branch_list: Vec<PatternBranch>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClosureNode {
  pub argument_list: Vec<ClosureArgumentNode>,
  pub return_type: Option<TypeKind>,
  pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClosureArgumentNode {
  pub ident: IdentNode,
  pub ty: Option<TypeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnNode {
  pub value: Option<Box<ExpressionKind>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternKind {
  Placeholder,
  UnnamedStruct(UnnamedStructNode),
  NamedStruct(NamedStructNode),
  Literal(LiteralValueKind),
  Path(PathNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleNode {
  pub field: Option<Box<ExpressionKind>>,
  pub argument_list: Vec<TupleArgument>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleArgument {
  pub name: Option<IdentNode>,
  pub value: ExpressionKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexNode {
  pub array: Box<ExpressionKind>,
  pub index: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfixOperatorNode {
  pub kind: InfixOperatorKind,
  pub lhs: Box<ExpressionKind>,
  pub rhs: Box<ExpressionKind>,
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
  pub value: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperatorKind {
  Not,
  Negative,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionKindFieldNode {
  pub lhs: Box<ExpressionKind>,
  pub rhs: Box<IdentNode>,
}

pub type ExpressionKindStructField = (IdentNode, Option<ExpressionKind>);

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionKindStructNode {
  pub path: Option<PathNode>,
  pub field_list: Vec<ExpressionKindStructField>,
  pub rest: Option<Box<ExpressionKind>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockNode {
  pub statement_list: Vec<StatementKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImplementItemKind {
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
  pub field_list: Vec<PatternKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamedStructNode {
  pub path: Option<PathNode>,
  pub field_list: Vec<FieldNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldNode {
  pub ident: IdentNode,
  pub pattern: Option<PatternKind>,
}

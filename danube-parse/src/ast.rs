use std::collections::HashMap;

pub type Program = Attributed<ProgramNode>;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Attributed<T: Sized> {
    pub attribute_list: Vec<AttributeNode>,
    pub node: T,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AttributeNode {
    pub path: PathNode,
    pub args: HashMap<String, Option<LiteralKind>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PathNode {
    pub ident_list: Vec<IdentNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdentNode {
    pub raw: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, PartialEq)]
pub struct ProgramNode {
    pub item_list: Vec<Item>,
}

pub type Item = Attributed<ItemKind>;

#[derive(Debug, PartialEq, Clone)]
pub enum ItemKind {
    Use(UseNode),
    Module(ModuleNode),
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
    pub visibility: VisibilityKind,
    pub kind: UseKind<UseRootNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VisibilityKind {
    TypeSelf,
    Public,
    Super,
    Module,
    Restricted(PathNode),
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
pub struct ModuleNode {
    pub ident: IdentNode,
    pub item_list: Option<Vec<Item>>,
}

pub type GenericNodeList = Vec<GenericNode>;

#[derive(Debug, PartialEq, Clone)]
pub struct StructNode {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub generic_list: GenericNodeList,
    pub fields: StructFieldKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GenericNode {
    pub ident: IdentNode,
    pub trait_list: Vec<PathNode>,
    pub default_trait_list: Vec<PathNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StructFieldKind {
    Unnamed(StructUnnamedFieldNode),
    Named(StructNamedFieldNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructUnnamedFieldNode {
    pub node_list: Vec<(VisibilityKind, TypeKind)>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImmutablityKind {
    Yes,
    Nope,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructNamedFieldNode {
    pub node_list: Vec<(VisibilityKind, IdentNode, TypeKind)>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    TypeSelf(ImmutablityKind),
    Array(ImmutablityKind, TypeArrayNode),
    Tuple(ImmutablityKind, Vec<TypeKind>),
    Path(ImmutablityKind, PathNode),
    Generic(ImmutablityKind, PathNode, Vec<PathNode>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeArrayNode {
    pub ty: Box<TypeKind>,
    pub size: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumNode {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub generic_list: GenericNodeList,
    pub variant_list: Vec<(IdentNode, EnumVariantKind)>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EnumVariantKind {
    Unnamed(EnumUnnamedVariantNode),
    Named(EnumNamedVariantNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumUnnamedVariantNode {
    pub node_list: Vec<TypeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumNamedVariantNode {
    pub node_list: Vec<(IdentNode, TypeKind)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub generic_list: GenericNodeList,
    pub self_type: Option<ImmutablityKind>,
    pub argument_list: Vec<FunctionArgumentNode>,
    pub return_type: TypeKind,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionArgumentNode {
    pub immutablity: ImmutablityKind,
    pub ident: IdentNode,
    pub ty: Option<TypeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockNode {
    pub statement_list: Vec<StatementKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Item(Box<Item>),
    CompoundAssign(Box<CompoundAssignNode>),
    Let(Box<LetNode>),
    Expression(ExpressionKind),
    Semicolon,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompoundAssignNode {
    pub kind: CompoundAssignKind,
    pub lhs: ExpressionKind,
    pub rhs: ExpressionKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CompoundAssignKind {
    Add, // +=
    Sub, // -=
    Exp, // **=
    Mul, // *=
    Div, // /=
    Mod, // %=
    And, // &&=
    Or,  // ||=

    BitAnd,   // &=
    BitOr,    // |=
    BitXor,   // ^=
    BitLeft,  // <<=
    BitRight, // >>=
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetNode {
    pub immutablity: ImmutablityKind,
    pub pattern: PatternKind,
    pub ty: Option<TypeKind>,
    pub value: Option<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternKind {
    Placeholder,
    UnnamedStruct(ExpressionUnnamedStructNode),
    NamedStruct(ExpressionNamedStructNode),
    Literal(LiteralKind),
    Path(PathNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionUnnamedStructNode {
    pub path: Option<PathNode>,
    pub field_list: Vec<PatternKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionNamedStructNode {
    pub path: Option<PathNode>,
    pub field_list: Vec<(IdentNode, Option<PatternKind>)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAliasNode {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub ty: TypeKind,
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
    Literal(LiteralKind),
    Array(Vec<ExpressionKind>),
    Tuple(TupleNode),
    Index(IndexNode),
    Generic(ExpressionGenericNode),
    UnaryOperator(UnaryOperatorNode),
    InfixOperator(InfixOperatorNode),
    Await(Box<ExpressionKind>),
    Try(Box<ExpressionKind>),
    Field(ExpressionKindFieldNode),
    Struct(ExpressionKindStructNode),
    Block(BlockNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionalNode {
    pub main_branch: ConditionalBranch,
    pub branch_list: Vec<ConditionalBranch>,
    pub other: Option<BlockNode>,
}

pub type ConditionalBranch = (ConditionNode, BlockNode);

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionNode {
    pub pattern: Option<(ImmutablityKind, PatternKind)>,
    pub value: Box<ExpressionKind>,
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

#[derive(Debug, PartialEq, Clone)]
pub struct PatternMatchNode {
    pub condition: Box<ExpressionKind>,
    pub branch_list: Vec<PatternBranch>,
}

pub type PatternBranch = (Vec<PatternKind>, BlockNode);

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
pub struct TupleNode {
    pub field: Option<Box<ExpressionKind>>,
    pub argument_list: Vec<TupleArgumentNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleArgumentNode {
    pub name: Option<IdentNode>,
    pub value: ExpressionKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexNode {
    pub array: Box<ExpressionKind>,
    pub index: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionGenericNode {
    pub expression: Box<ExpressionKind>,
    pub generic_list: Vec<ExpressionGenericKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionGenericKind {
    Output(IdentNode, TypeKind),
    Input(TypeKind),
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfixOperatorNode {
    pub kind: InfixOperatorKind,
    pub lhs: Box<ExpressionKind>,
    pub rhs: Box<ExpressionKind>,
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
pub struct TraitNode {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub generic_list: GenericNodeList,
    pub inheritance_list: Vec<(PathNode, Vec<PathNode>)>,
    pub item_list: Vec<TraitItemKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TraitItemKind {
    OutputType(TraitItemOutputTypeNode),
    Constant(TraitItemConstantNode),
    Function(TraitItemFunctionNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitItemOutputTypeNode {
    pub ident: IdentNode,
    pub ty: Option<TypeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitItemConstantNode {
    pub ident: IdentNode,
    pub ty: TypeKind,
    pub default_value: Option<ValueKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueKind {
    Literal(LiteralKind),
    Path(PathNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitItemFunctionNode {
    pub ident: IdentNode,
    pub generic_list: GenericNodeList,
    pub self_type: Option<ImmutablityKind>,
    pub argument_list: Vec<FunctionArgumentNode>,
    pub return_type: Option<TypeKind>,
    pub block: Option<BlockNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantNode {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub ty: TypeKind,
    pub value: ValueKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StaticNode {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub ty: TypeKind,
    pub value: ValueKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImplementNode {
    pub visibility: VisibilityKind,
    pub generic_list: GenericNodeList,
    pub target: PathNode,
    pub item_list: Vec<ImplementItemKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImplementItemKind {
    OutputType(Attributed<ImplementOutputTypeNode>),
    Constant(Attributed<ConstantNode>),
    Function(Attributed<FunctionNode>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImplementOutputTypeNode {
    pub ident: IdentNode,
    pub ty: TypeKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImplementTraitNode {
    pub visibility: VisibilityKind,
    pub target: PathNode,
    pub target_generic_list: GenericNodeList,
    pub trait_ident: PathNode,
    pub generic_list: GenericNodeList,
    pub item_list: Vec<ImplementItemKind>,
}

#![warn(clippy::all)]

use std::collections::HashMap;

pub struct Package {
    pub attribute_list: Vec<AttributeNode>,
    pub item_list: Vec<ItemNode>,
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
    Char(char),
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ItemNode {
    pub id: ItemId,
    pub attribute_list: Vec<AttributeNode>,
    pub kind: ItemKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ItemKind {
    Use(UseNode),
    Module(ModuleNode),
    Enum(EnumNode),
    Function(FunctionNode),
    TypeAlias(TypeAliasNode),
    Trait(TraitNode),
    Constant(ConstantNode),
    Implement(ImplementNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct UseNode {
    pub visibility: VisibilityKind,
    pub node_list: Vec<UseRootNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VisibilityKind {
    Current,
    Public,
    Super,
    Package,
    Restricted(PathNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct UseRootNode {
    pub root: UseRootIdentKind,
    pub extra_list: Vec<UseExtraKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UseRootIdentKind {
    Current,
    Super,
    Package,
    Ident(IdentNode),
}

#[derive(Debug, PartialEq, Clone)]
pub enum UseExtraKind {
    All,
    Ident(IdentNode),
    Nested(IdentNode, Vec<UseExtraKind>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleNode {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub item_list: Option<Vec<ItemNode>>,
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
    Unnamed(Vec<(VisibilityKind, TypeNode)>),
    Named(Vec<(VisibilityKind, IdentNode, TypeNode)>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImmutablityKind {
    Nope,
    Yes,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeNode {
    pub immutablity: ImmutablityKind,
    pub kind: TypeKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    TypeSelf,
    Tuple(Vec<TypeKind>),
    Path(PathNode),
    Generic(PathNode, Vec<PathNode>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumNode {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub generic_list: GenericNodeList,
    pub variant_list: Vec<EnumVariantNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumVariantNode {
    pub ident: IdentNode,
    pub kind: Option<EnumVariantKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EnumVariantKind {
    Unnamed(EnumUnnamedVariantNode),
    Named(EnumNamedVariantNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumUnnamedVariantNode {
    pub node_list: Vec<TypeNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumNamedVariantNode {
    pub node_list: Vec<(IdentNode, TypeNode)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub generic_list: GenericNodeList,
    pub self_type: Option<ImmutablityKind>,
    pub parameter_list: Vec<FunctionParameterNode>,
    pub return_type: Option<TypeNode>,
    pub block: Option<BlockNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParameterNode {
    pub argument_label: IdentNode,
    pub parameter_label: Option<IdentNode>,
    pub ty: TypeNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockNode {
    pub statement_list: Vec<StatementNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatementNode {
    pub id: StatementId,
    pub attribute_list: Vec<AttributeNode>,
    pub kind: StatementKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Break,
    Continue,
    Return(ReturnNode),
    Item(Box<ItemNode>),
    Let(Box<LetNode>),
    Assign(Box<AssignNode>),
    Expression(ExpressionKind),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnNode {
    pub value: Option<Box<ExpressionKind>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignNode {
    pub kind: AssignKind,
    pub lhs: ExpressionKind,
    pub rhs: ExpressionKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssignKind {
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
    BitNot,   // ~=
    BitLeft,  // <<=
    BitRight, // >>=
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetNode {
    pub pattern: PatternNode,
    pub ty: Option<TypeNode>,
    pub value: Option<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PatternNode {
    pub part_list: Vec<PatternPart>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PatternPart {
    pub immutablity: ImmutablityKind,
    pub kind: PatternKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternKind {
    Placeholder,
    Literal(LiteralKind),
    Path(PathNode),
    UnnamedStruct(ExpressionUnnamedStructNode),
    NamedStruct(ExpressionNamedStructNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionUnnamedStructNode {
    pub path: Option<PathNode>,
    pub field_list: Vec<PatternNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionNamedStructNode {
    pub path: Option<PathNode>,
    pub field_list: Vec<(IdentNode, Option<PatternNode>)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAliasNode {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub ty: TypeNode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionKind {
    // Prefix
    Sub(Box<ExpressionKind>),
    Add(Box<ExpressionKind>),
    Not(Box<ExpressionKind>),
    BitNot(Box<ExpressionKind>),

    // Atomic
    Literal(LiteralKind),
    Conditional(ConditionNode),
    Loop(LoopNode),
    While(WhileNode),
    For(ForNode),
    PatternMatch(PatternMatchNode),
    Closure(ClosureNode),
    Block(BlockNode),
    Tuple(TupleNode),
    Array(Vec<ExpressionKind>),

    // Postfix
    Try(Box<ExpressionKind>),
    Await(Box<ExpressionKind>),
    Path(PathExpressioNode),
    Index(IndexNode),
    Field(FieldNode),
    FunctionCall(FunctionCallNode),

    // Binary
    Binary(BinaryExpressionNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionNode {
    pub branch_list: Vec<ConditionBranch>,
    pub other: Option<BlockNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionBranch {
    pub pattern: Option<PatternNode>,
    pub expression: Box<ExpressionKind>,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoopNode {
    pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileNode {
    pub branch: ConditionBranch,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForNode {
    pub pattern: PatternNode,
    pub expression: Box<ExpressionKind>,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PatternMatchNode {
    pub expression: Box<ExpressionKind>,
    pub branch_list: Vec<PatternBranch>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PatternBranch {
    pub pattern: PatternNode,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClosureNode {
    pub parameter_list: Option<ClosureArgumentListKind>,
    pub return_type: Option<TypeNode>,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClosureArgumentListKind {
    Untyped(Vec<IdentNode>),
    Typed(Vec<(IdentNode, TypeNode)>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleNode {
    pub argument_list: Vec<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PathExpressioNode {
    pub expression: Box<ExpressionKind>,
    pub path: PathNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexNode {
    pub expression: Box<ExpressionKind>,
    pub index: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpressionNode {
    pub kind: BinaryOperatorKind,
    pub lhs: Box<ExpressionKind>,
    pub rhs: Box<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldNode {
    pub expression: Box<ExpressionKind>,
    pub field: Box<IdentNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCallNode {
    pub expression: Box<ExpressionKind>,
    pub argument_list: Vec<FunctionArgumentNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionArgumentNode {
    pub ident: Option<IdentNode>,
    pub expression: ExpressionKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperatorKind {
    Add, // +
    Sub, // -
    Mul, // *
    Exp, // **
    Div, // /
    Mod, // %

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
pub struct TraitNode {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub generic_list: GenericNodeList,
    pub inheritance_list: Vec<(PathNode, Vec<PathNode>)>,
    pub item_list: Vec<ImplementItemNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantNode {
    pub visibility: VisibilityKind,
    pub pattern: PatternNode,
    pub ty: TypeNode,
    pub expression: Option<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImplementNode {
    pub generic_list: GenericNodeList,
    pub trait_ident: Option<PathNode>,
    pub target: PathNode,
    pub target_generic_list: GenericNodeList,
    pub item_list: Vec<ImplementItemNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImplementItemNode {
    pub attribute_list: Vec<AttributeNode>,
    pub kind: ImplementItemKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImplementItemKind {
    Type(TypeAliasNode),
    Constant(ConstantNode),
    Function(FunctionNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ItemId(pub Id);

#[derive(Debug, PartialEq, Clone)]
pub struct StatementId(pub Id);

#[derive(Debug, PartialEq, Clone)]
pub struct Id(pub u32);

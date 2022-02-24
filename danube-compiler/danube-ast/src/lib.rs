#![warn(clippy::all)]

use danube_token::{LiteralKind, Symbol};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct PackageNode {
    pub attributes: Vec<AttributeNode>,
    pub items: Vec<ItemNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AttributeNode {
    pub path: PathNode,
    pub args: HashMap<String, Option<LiteralKind>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PathNode {
    pub idents: Vec<IdentNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdentNode {
    pub symbol: Symbol,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ItemNode {
    pub id: ItemId,
    pub attributes: Vec<AttributeNode>,
    pub visibility: VisibilityKind,
    pub kind: ItemKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ItemKind {
    Use(UseNode),
    Enum(EnumNode),
    Function(FunctionNode),
    TypeAlias(TypeAliasNode),
    Trait(TraitNode),
    Constant(ConstantNode),
    Implement(ImplementNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct UseNode {
    pub path: PathNode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VisibilityKind {
    Current,
    Public,
    Restricted(PathNode),
}

pub type GenericNodeList = Vec<GenericNode>;

#[derive(Debug, PartialEq, Clone)]
pub struct StructNode {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub generics: GenericNodeList,
    pub fields: StructFieldKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GenericNode {
    pub ident: IdentNode,
    pub traits: Vec<PathNode>,
    pub default_traits: Vec<PathNode>,
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
    pub ident: IdentNode,
    pub generics: GenericNodeList,
    pub variants: Vec<EnumVariantNode>,
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
    pub nodes: Vec<TypeNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumNamedVariantNode {
    pub nodes: Vec<(IdentNode, TypeNode)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    pub ident: IdentNode,
    pub generics: GenericNodeList,
    pub self_type: Option<ImmutablityKind>,
    pub parameters: Vec<FunctionParameterNode>,
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
    pub statements: Vec<StatementNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatementNode {
    pub id: StatementId,
    pub kind: StatementKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Semicolon,
    Break,
    Continue,
    Return(Option<ExpressionKind>),
    Item(Box<ItemNode>),
    Let(Box<LetNode>),
    Assign(Box<AssignNode>),
    Expression(ExpressionKind),
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
    pub kind: PatternKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternKind {
    /// _
    Wildcard,
    /// ..
    Rest,
    Ident(),
    /// 1, 2.3, 'c', "har"
    Literal(LiteralKind),
    /// foo, foo::bar
    Path(PathNode),
    /// Foo { a, b }
    NamedStruct(Option<PathNode>, Vec<PatternNode>),
    /// Foo(a, b)
    UnnamedStruct(Option<PathNode>, Vec<PatternNode>),
    /// [foo, bar]
    Slice(Vec<PatternNode>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionUnnamedStructNode {
    pub path: Option<PathNode>,
    pub fields: Vec<PatternNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionNamedStructNode {
    pub path: Option<PathNode>,
    pub fields: Vec<(IdentNode, Option<PatternNode>)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAliasNode {
    pub ident: IdentNode,
    pub ty: TypeNode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionKind {
    // Binding
    Let(PatternNode, Box<ExpressionKind>),

    // Prefix
    Negate(Box<ExpressionKind>),
    Not(Box<ExpressionKind>),
    BitNot(Box<ExpressionKind>),

    // Atomic
    Literal(Symbol, LiteralKind),
    Conditional(ConditionNode),
    Loop(LoopNode),
    While(WhileNode),
    For(ForNode),
    Match(MatchNode),
    Path(PathNode),
    FunctionCall(FunctionCallNode),
    Closure(ClosureNode),
    Block(BlockNode),
    Tuple(TupleNode),
    Array(Vec<ExpressionKind>),

    // Postfix
    Try(Box<ExpressionKind>),
    Await(Box<ExpressionKind>),
    Field(FieldNode),
    Index(IndexNode),
    MethodCall(MethodCallNode),

    // Binary
    Binary(BinaryExpressionNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionNode {
    pub branches: Vec<ConditionBranch>,
    pub other: Option<BlockNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionBranch {
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
    pub iter: Box<ExpressionKind>,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MatchNode {
    pub expression: Box<ExpressionKind>,
    pub branches: Vec<MatchBranch>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MatchBranch {
    pub pattern: PatternNode,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClosureNode {
    pub parameters: Vec<(IdentNode, Option<TypeNode>)>,
    pub return_type: Option<TypeNode>,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleNode {
    pub arguments: Vec<ExpressionKind>,
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
    pub field: IdentNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCallNode {
    pub expression: Box<ExpressionKind>,
    pub arguments: Vec<ArgumentNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodCallNode {
    pub ident: IdentNode,
    pub arguments: Vec<ArgumentNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArgumentNode {
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitNode {
    pub ident: IdentNode,
    pub generics: GenericNodeList,
    pub inheritances: Vec<(PathNode, Vec<PathNode>)>,
    pub items: Vec<ImplementItemNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantNode {
    pub pattern: PatternNode,
    pub ty: TypeNode,
    pub expression: Option<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImplementNode {
    pub generics: GenericNodeList,
    pub trait_ident: Option<PathNode>,
    pub target: PathNode,
    pub target_generics: GenericNodeList,
    pub items: Vec<ImplementItemNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImplementItemNode {
    pub attributes: Vec<AttributeNode>,
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

impl From<Id> for ItemId {
    fn from(id: Id) -> ItemId {
        ItemId(id)
    }
}

impl From<Id> for StatementId {
    fn from(id: Id) -> StatementId {
        StatementId(id)
    }
}

#![warn(clippy::all)]

pub use danube_token::LiteralKind;
use danube_token::Symbol;

danube_index::newtype_index! {
    pub struct NodeId(usize);
    pub struct AttributeId(usize);
}

pub const DUMMY_NODE_ID: NodeId = NodeId(0);
pub const DUMMY_ATTRIBUTE_ID: AttributeId = AttributeId(0);

#[derive(Debug, PartialEq, Clone)]
pub struct PackageNode {
    pub id: NodeId,
    pub attributes: Vec<AttributeNode>,
    pub items: Vec<ItemNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AttributeNode {
    pub id: AttributeId,
    pub path: PathNode,
    pub args: Vec<(IdentNode, Option<ExpressionNode>)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PathNode {
    pub segments: Vec<IdentNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdentNode {
    pub id: NodeId,
    pub symbol: Symbol,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ItemNode {
    pub id: NodeId,
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
    pub id: NodeId,
    pub ident: IdentNode,
    pub traits: Vec<PathNode>,
    pub default: Option<PathNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StructFieldKind {
    Unnamed(Vec<(VisibilityKind, TypeNode)>),
    Named(Vec<(VisibilityKind, IdentNode, TypeNode)>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImmutabilityKind {
    Nope,
    Yes,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeNode {
    pub id: NodeId,
    pub immutability: ImmutabilityKind,
    pub kind: TypeKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    Tuple(Vec<TypeKind>),
    Path(PathNode),
    Generic(PathNode, Vec<TypeKind>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumNode {
    pub ident: IdentNode,
    pub generics: GenericNodeList,
    pub variants: Vec<EnumVariantNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumVariantNode {
    pub id: NodeId,
    pub ident: IdentNode,
    pub kind: Option<EnumVariantKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EnumVariantKind {
    Unnamed(Vec<TypeNode>),
    Named(Vec<(IdentNode, TypeNode)>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    pub ident: IdentNode,
    pub generics: GenericNodeList,
    pub self_type: Option<ImmutabilityKind>,
    pub parameters: Vec<FunctionParameterNode>,
    pub return_type: Option<TypeNode>,
    pub block: Option<BlockNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParameterNode {
    pub id: NodeId,
    pub argument_label: IdentNode,
    pub parameter_label: Option<IdentNode>,
    pub ty: TypeNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockNode {
    pub id: NodeId,
    pub statements: Vec<StatementNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatementNode {
    pub id: NodeId,
    pub kind: StatementKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Semicolon,
    Break,
    Continue,
    Return(Option<ExpressionNode>),
    Item(Box<ItemNode>),
    Let(Box<LetNode>),
    Assign(Box<AssignNode>),
    Expression(ExpressionNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignNode {
    pub kind: AssignKind,
    pub lhs: ExpressionNode,
    pub rhs: ExpressionNode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssignKind {
    Assign, // =

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
    pub id: NodeId,
    pub immutability: ImmutabilityKind,
    pub pattern: PatternNode,
    pub ty: Option<TypeNode>,
    pub value: Option<ExpressionNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PatternNode {
    pub id: NodeId,
    pub kind: PatternKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternKind {
    /// _
    Wildcard,
    /// ..
    Rest,
    /// 1, 2.3, 'c', "har"
    Literal(Symbol, LiteralKind),
    /// foo, foo::bar
    Path(PathNode),
    /// Foo { a, b }
    NamedStruct(PathNode, Vec<(PathNode, Option<PatternNode>)>),
    /// Foo(a, b)
    UnnamedStruct(Option<PathNode>, Vec<PatternNode>),
    /// [foo, bar]
    Slice(Vec<PatternNode>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAliasNode {
    pub ident: IdentNode,
    pub ty: Option<TypeNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionNode {
    pub id: NodeId,
    pub kind: ExpressionKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionKind {
    // Binding
    Let(PatternNode, Box<ExpressionNode>),

    // Prefix
    Negate(Box<ExpressionNode>),
    Not(Box<ExpressionNode>),

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
    Array(Vec<ExpressionNode>),

    // Postfix
    Try(Box<ExpressionNode>),
    Await(Box<ExpressionNode>),
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
    pub expression: Box<ExpressionNode>,
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
    pub iter: Box<ExpressionNode>,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MatchNode {
    pub expression: Box<ExpressionNode>,
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
    pub arguments: Vec<ExpressionNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexNode {
    pub expression: Box<ExpressionNode>,
    pub index: Box<ExpressionNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpressionNode {
    pub kind: BinaryOperatorKind,
    pub lhs: Box<ExpressionNode>,
    pub rhs: Box<ExpressionNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldNode {
    pub expression: Box<ExpressionNode>,
    pub field: IdentNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCallNode {
    pub expression: Box<ExpressionNode>,
    pub arguments: Vec<ArgumentNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodCallNode {
    pub ident: IdentNode,
    pub arguments: Vec<ArgumentNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArgumentNode {
    pub id: NodeId,
    pub ident: Option<IdentNode>,
    pub expression: ExpressionNode,
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
    pub inheritances: Vec<PathNode>,
    pub items: Vec<ImplementItemNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantNode {
    pub pattern: PatternNode,
    pub ty: TypeNode,
    pub expression: Option<ExpressionNode>,
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
    pub id: NodeId,
    pub attributes: Vec<AttributeNode>,
    pub kind: ImplementItemKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImplementItemKind {
    Type(TypeAliasNode),
    Constant(ConstantNode),
    Function(FunctionNode),
}

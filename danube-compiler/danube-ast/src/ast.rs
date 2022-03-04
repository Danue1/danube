use crate::{AttributeId, NodeId};
use danube_token::LiteralKind;
use danube_token::Symbol;

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
    pub args: Vec<AttributeArgumentNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AttributeArgumentNode {
    pub ident: IdentNode,
    pub value: Option<ExpressionNode>,
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
    Struct(StructNode),
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

#[derive(Debug, PartialEq, Clone)]
pub struct StructNode {
    pub ident: IdentNode,
    pub generics: Vec<GenericNode>,
    pub fields: Option<StructFieldKind>,
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
    Unnamed(Vec<UnnamedStructField>),
    Named(Vec<NamedStructField>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnnamedStructField {
    pub visibility: VisibilityKind,
    pub ty: TypeNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamedStructField {
    pub visibility: VisibilityKind,
    pub ident: IdentNode,
    pub ty: TypeNode,
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
    Generic(GenericTypeNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct GenericTypeNode {
    pub path: PathNode,
    pub parameters: Vec<TypeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumNode {
    pub ident: IdentNode,
    pub generics: Vec<GenericNode>,
    pub variants: Vec<EnumVariantNode>,
}

/// ident: Ident
/// enum <Ident> {
///    // kind: None
///    <Ident>
///    // kind: Some(EnumVariantKind::Unnamed)
///    <Ident> { <Ident>: <Type> }
///    // kind: Some(EnumVariantKind::Named)
///    <Ident> ( <Type> )
/// }
#[derive(Debug, PartialEq, Clone)]
pub struct EnumVariantNode {
    pub id: NodeId,
    pub ident: IdentNode,
    pub kind: Option<EnumVariantKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EnumVariantKind {
    /// ( (<Type>)* )
    Unnamed(Vec<TypeNode>),
    /// { (<Ident>: <Type>),* }
    Named(Vec<EnumNamedVariantNode>),
}

/// <Ident>: <Type>
#[derive(Debug, PartialEq, Clone)]
pub struct EnumNamedVariantNode {
    pub ident: IdentNode,
    pub ty: TypeNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    pub ident: IdentNode,
    pub generics: Vec<GenericNode>,
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
    Literal(LiteralNode),
    /// foo, foo::bar
    Path(PathNode),
    /// Foo { a, b }
    NamedStruct(PatternNamedStructNode),
    /// Foo(a, b)
    UnnamedStruct(PatternUnnamedStructNode),
    /// [foo, bar]
    Slice(Vec<PatternNode>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LiteralNode {
    pub symbol: Symbol,
    pub kind: LiteralKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PatternNamedStructNode {
    pub path: PathNode,
    pub fields: Vec<PatternNamedStructFieldNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PatternNamedStructFieldNode {
    pub path: PathNode,
    pub pattern: Option<PatternNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PatternUnnamedStructNode {
    pub path: Option<PathNode>,
    pub fields: Vec<PatternNode>,
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
    Let(LetExpressionNode),

    // Prefix
    Negate(Box<ExpressionNode>),
    Not(Box<ExpressionNode>),

    // Atomic
    Literal(LiteralNode),
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
pub struct LetExpressionNode {
    pub pattern: PatternNode,
    pub value: Box<ExpressionNode>,
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
    pub parameters: Vec<ClosureParameterNode>,
    pub return_type: Option<TypeNode>,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClosureParameterNode {
    pub ident: IdentNode,
    pub ty: Option<TypeNode>,
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
    pub generics: Vec<GenericNode>,
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
    pub generics: Vec<GenericNode>,
    pub trait_ident: Option<PathNode>,
    pub target: PathNode,
    pub target_generics: Vec<GenericNode>,
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

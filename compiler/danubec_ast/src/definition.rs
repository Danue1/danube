use danubec_syntax::SyntaxNode;
use std::{collections::HashMap, path::PathBuf};

pub type NodeId = usize;

pub struct Krate {
    pub attributes: Vec<TopLevelAttribute>,
    pub definitions: Vec<Definition>,
    pub children: HashMap<Identifier, Root>,
}

pub struct Root {
    pub definitions: Vec<Definition>,
    pub children: HashMap<Identifier, Root>,
}

pub struct TopLevelAttribute {
    pub path: Path,
    pub arguments: Vec<AttributeArgument>,
}

pub struct Attribute {
    pub path: Path,
    pub arguments: Vec<AttributeArgument>,
}

pub enum AttributeArgument {
    Expression {
        value: Expression,
    },
    KeyValue {
        node_id: NodeId,
        key: Path,
        value: Expression,
    },
    Nested {
        path: Path,
        arguments: Vec<AttributeArgument>,
    },
}

pub struct Definition {
    pub attributes: Vec<Attribute>,
    pub kind: DefinitionKind,
}

pub enum DefinitionKind {
    Function {
        node_id: NodeId,
        visibility: Visibility,
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        parameters: Vec<FunctionParameter>,
        return_type: Option<TypeExpression>,
        type_bounds: Vec<(TypeExpression, TypeBound)>,
        body: Expression,
    },
    Struct {
        node_id: NodeId,
        visibility: Visibility,
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<(TypeExpression, TypeBound)>,
        body: Option<StructBody>,
    },
    Enum {
        node_id: NodeId,
        visibility: Visibility,
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<(TypeExpression, TypeBound)>,
        variants: Vec<EnumVariant>,
    },
    Use {
        node_id: NodeId,
        visibility: Visibility,
        tree: UseTree,
    },
    Module {
        node_id: NodeId,
        visibility: Visibility,
        name: Identifier,
        definitions: Vec<Definition>,
    },
    Trait {
        node_id: NodeId,
        visibility: Visibility,
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<(TypeExpression, TypeBound)>,
        definitions: Vec<AssociatedDefinition>,
    },
    Constant {
        node_id: NodeId,
        visibility: Visibility,
        name: Identifier,
        r#type: TypeExpression,
        value: Expression,
    },
    Static {
        node_id: NodeId,
        visibility: Visibility,
        name: Identifier,
        r#type: TypeExpression,
        value: Expression,
    },
    Type {
        node_id: NodeId,
        visibility: Visibility,
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<(TypeExpression, TypeBound)>,
        expression: TypeExpression,
    },
    Implement {
        type_parameters: Vec<TypeParameter>,
        trait_name: Option<Identifier>,
        for_type: TypeExpression,
        type_bounds: Vec<(TypeExpression, TypeBound)>,
        definitions: Vec<AssociatedDefinition>,
    },
}

pub enum Visibility {
    Public,
    Crate,
    Restricted(NodeId, Identifier),
    Private,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
}

pub struct TypeParameter {
    pub name: Identifier,
    pub bounds: Vec<TypeExpression>,
}

pub struct TypeBound {
    pub r#type: TypeExpression,
    pub constraints: Vec<TypeExpression>,
}

pub struct FunctionParameter {
    pub node_id: NodeId,
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub r#type: TypeExpression,
}

pub enum StructBody {
    Named(Vec<(NodeId, Visibility, Identifier, TypeExpression)>),
    Unnamed(Vec<(Visibility, TypeExpression)>),
}

pub struct EnumVariant {
    pub node_id: NodeId,
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub body: Option<EnumVariantBody>,
}

pub enum EnumVariantBody {
    Named(Vec<(NodeId, Vec<Attribute>, Identifier, TypeExpression)>),
    Unnamed(Vec<(Vec<Attribute>, TypeExpression)>),
}

pub struct UseTree {
    pub node_id: NodeId,
    pub prefix: Path,
    pub kind: UseTreeKind,
}

pub enum UseTreeKind {
    Glob,
    Terminal { alias: Option<(NodeId, Identifier)> },
    Nested { trees: Vec<UseTree> },
}

pub struct Path {
    pub segments: Vec<PathSegment>,
}

pub struct PathSegment {
    pub node_id: NodeId,
    pub name: Identifier,
    pub type_arguments: Vec<TypeExpression>,
}

pub struct AssociatedDefinition {
    pub node_id: NodeId,
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub kind: AssociatedDefinitionKind,
}

pub enum AssociatedDefinitionKind {
    Function {
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        parameters: Vec<FunctionParameter>,
        return_type: Option<TypeExpression>,
        type_bounds: Vec<(TypeExpression, TypeBound)>,
        body: Option<Expression>,
    },
    Constant {
        name: Identifier,
        r#type: TypeExpression,
        value: Option<Expression>,
    },
    Type {
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<(TypeExpression, TypeBound)>,
        expression: Option<TypeExpression>,
    },
}

pub struct Expression {
    pub node_id: NodeId,
    pub kind: ExpressionKind,
}

pub enum ExpressionKind {
    Break,
    Continue,
    Return {
        value: Option<Box<Expression>>,
    },
    For {
        pattern: Pattern,
        iterable: Box<Expression>,
        body: Vec<Statement>,
    },
    While {
        condition: Box<Expression>,
        body: Vec<Statement>,
    },
    Loop {
        body: Vec<Statement>,
    },
    If {
        condition: Box<Expression>,
        then_branch: Vec<Statement>,
        else_branch: Option<Box<Expression>>,
    },
    Match {
        expression: Box<Expression>,
        arms: Vec<(Pattern, Expression)>,
    },
    Let {
        pattern: Pattern,
        type_annotation: Option<TypeExpression>,
        initializer: Option<Box<Expression>>,
    },
    Array {
        elements: Vec<Expression>,
    },
    Block {
        attributes: Vec<Attribute>,
        statements: Vec<Statement>,
    },
    Literal {
        value: Literal,
    },
    Path {
        path: Path,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Assignment {
        target: Box<Expression>,
        operator: AssignmentOperator,
        value: Box<Expression>,
    },
    FunctionCall {
        callee: Box<Expression>,
        type_arguments: Vec<TypeExpression>,
        arguments: Vec<Expression>,
    },
    MethodCall {
        node_id: NodeId,
        receiver: Box<Expression>,
        identifier: Identifier,
        type_arguments: Vec<TypeExpression>,
        arguments: Vec<Expression>,
    },
    Field {
        node_id: NodeId,
        receiver: Box<Expression>,
        field: Identifier,
    },
    Index {
        collection: Box<Expression>,
        index: Box<Expression>,
    },
    Struct {
        path: Path,
        fields: Vec<(NodeId, Identifier, Expression)>,
        rest: Option<Box<Expression>>,
    },
    Await {
        expression: Box<Expression>,
    },
    Range {
        start: Option<Box<Expression>>,
        operator: RangeOperator,
        end: Option<Box<Expression>>,
    },
    Try {
        expression: Box<Expression>,
    },
    Yield {
        expression: Box<Expression>,
    },
}

pub enum Statement {
    Definition {
        definition: Definition,
    },
    Let {
        pattern: Pattern,
        r#type: Option<TypeExpression>,
        expression: Option<Expression>,
    },
    Expression {
        expression: Expression,
    },
    Semicolon,
}

pub enum Pattern {
    Never,
    Placeholder,
    Path {
        path: Path,
    },
    Tuple {
        elements: Vec<Pattern>,
    },
    Array {
        elements: Vec<Pattern>,
    },
    Literal {
        value: Literal,
    },
    Rest {
        pattern: Box<Pattern>,
    },
    Or {
        patterns: Vec<Pattern>,
    },
    Named {
        path: Path,
        fields: Vec<(NodeId, Identifier, Pattern)>,
    },
    Unnamed {
        elements: Vec<(NodeId, Pattern)>,
    },
}

pub enum Literal {
    Boolean { value: bool },
    Character { value: char },
    Float { value: f64 },
    Integer { value: i64 },
    String { value: String },
}

pub enum UnaryOperator {
    Positive,
    Negate,
    WrappingNegate,
    Not,
    BitwiseNot,
}

pub enum BinaryOperator {
    Add,
    SaturatingAdd,
    WrappingAdd,
    Subtract,
    SaturatingSubtract,
    WrappingSubtract,
    Multiply,
    SaturatingMultiply,
    WrappingMultiply,
    Divide,
    Remainder,
    Exponent,
    SaturatingExponent,
    WrappingExponent,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    SaturatingLeftShift,
    RightShift,
    RightShiftUnsigned,
    LogicalAnd,
    LogicalOr,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

pub enum AssignmentOperator {
    Assign,
    Add,
    SaturatingAdd,
    WrappingAdd,
    Subtract,
    SaturatingSubtract,
    WrappingSubtract,
    Multiply,
    SaturatingMultiply,
    WrappingMultiply,
    Divide,
    Remainder,
    Exponent,
    SaturatingExponent,
    WrappingExponent,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    SaturatingLeftShift,
    RightShift,
    RightShiftUnsigned,
    LogicalAnd,
    LogicalOr,
}

pub enum RangeOperator {
    Exclusive,
    Inclusive,
}

pub enum TypeExpression {
    Never,
    Path { path: Path },
    Slice { element: Box<TypeExpression> },
    Tuple { elements: Vec<TypeExpression> },
}

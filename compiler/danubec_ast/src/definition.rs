use danubec_symbol::Symbol;
use danubec_syntax::{Span, SyntaxNode};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Krate {
    pub attributes: Vec<TopLevelAttribute>,
    pub definitions: Vec<Definition>,
    pub children: HashMap<Identifier, Root>,
}

#[derive(Debug)]
pub struct Root {
    pub definitions: Vec<Definition>,
    pub children: HashMap<Identifier, Root>,
}

#[derive(Debug)]
pub struct TopLevelAttribute {
    pub argument: AttributeArgument,
    pub span: Span,
}

#[derive(Debug)]
pub struct Attribute {
    pub argument: AttributeArgument,
    pub span: Span,
}

#[derive(Debug)]
pub struct AttributeArgument {
    pub kind: AttributeArgumentKind,
}

#[derive(Debug)]
pub enum AttributeArgumentKind {
    Expression {
        value: Expression,
    },
    KeyValue {
        key: Path,
        value: Option<Expression>,
    },
    Nested {
        path: Path,
        arguments: Vec<AttributeArgument>,
    },
}

#[derive(Debug)]
pub struct Definition {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub kind: DefinitionKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum DefinitionKind {
    Function {
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        parameters: Vec<FunctionParameter>,
        return_type: Option<TypeExpression>,
        type_bounds: Vec<TypeBound>,
        body: Option<Vec<Statement>>,
    },
    Struct {
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<TypeBound>,
        body: StructBody,
    },
    Enum {
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<TypeBound>,
        variants: Vec<EnumVariant>,
    },
    Use {
        tree: UseTree,
    },
    Module {
        name: Identifier,
        kind: ModuleDefinitionKind,
    },
    Trait {
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<TypeBound>,
        definitions: Vec<AssociatedDefinition>,
    },
    Constant {
        name: Identifier,
        r#type: TypeExpression,
        initializer: Expression,
    },
    Static {
        name: Identifier,
        r#type: TypeExpression,
        initializer: Expression,
    },
    Type {
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<TypeBound>,
        initializer: Option<TypeExpression>,
    },
    Implement {
        type_parameters: Vec<TypeParameter>,
        trait_type: Option<TypeExpression>,
        target_type: TypeExpression,
        type_bounds: Vec<TypeBound>,
        definitions: Vec<AssociatedDefinition>,
    },
}

#[derive(Debug)]
pub enum ModuleDefinitionKind {
    External,
    Inline { definitions: Vec<Definition> },
}

#[derive(Debug)]
pub enum Visibility {
    Krate,
    Super,
    Self_,
    Restricted(Path),
    Private,
}

#[derive(Debug)]
pub struct Identifier {
    pub name: Symbol,
    pub span: Span,
}

#[derive(Debug)]
pub struct TypeParameter {
    pub r#type: TypeExpression,
    pub constraints: Vec<TypeExpression>,
    pub span: Span,
}

#[derive(Debug)]
pub struct TypeBound {
    pub r#type: TypeExpression,
    pub constraints: Vec<TypeExpression>,
    pub span: Span,
}

#[derive(Debug)]
pub struct FunctionParameter {
    pub attributes: Vec<Attribute>,
    pub pattern: Pattern,
    pub r#type: TypeExpression,
    pub span: Span,
}

#[derive(Debug)]
pub enum StructBody {
    Unit,
    Named(Vec<(Visibility, Identifier, TypeExpression)>),
    Unnamed(Vec<(Visibility, TypeExpression)>),
}

#[derive(Debug)]
pub struct EnumVariant {
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub kind: EnumVariantKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum EnumVariantKind {
    Unit,
    Scalar(Expression),
    Named(Vec<(Vec<Attribute>, Identifier, TypeExpression)>),
    Unnamed(Vec<(Vec<Attribute>, TypeExpression)>),
}

#[derive(Debug)]
pub struct UseTree {
    pub root: bool,
    pub kind: UseTreeKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum UseTreeKind {
    List {
        trees: Vec<UseTree>,
    },
    Glob,
    Element {
        path: Path,
        trailing: UseTreeTrailing,
    },
}

#[derive(Debug)]
pub enum UseTreeTrailing {
    Identifier,
    List { trees: Vec<UseTree> },
    Glob,
    Rename { name: Identifier },
}

#[derive(Debug)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub span: Span,
}

#[derive(Debug)]
pub struct PathSegment {
    pub kind: PathSegmentKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum PathSegmentKind {
    Root,
    Self_,
    Super_,
    Krate,
    Identifier(Identifier),
}

#[derive(Debug)]
pub struct AssociatedDefinition {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub kind: AssociatedDefinitionKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum AssociatedDefinitionKind {
    Function {
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        parameters: Vec<FunctionParameter>,
        return_type: Option<TypeExpression>,
        type_bounds: Vec<TypeBound>,
        body: Option<Vec<Statement>>,
    },
    Constant {
        name: Identifier,
        r#type: Option<TypeExpression>,
        initializer: Option<Expression>,
    },
    Type {
        name: Identifier,
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<TypeBound>,
        initializer: Option<TypeExpression>,
    },
}

#[derive(Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

#[derive(Debug)]
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
        r#type: Option<TypeExpression>,
        initializer: Option<Box<Expression>>,
    },
    Array {
        elements: Vec<Expression>,
    },
    Tuple {
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
        left: Box<Expression>,
        operator: AssignmentOperator,
        right: Box<Expression>,
    },
    FunctionCall {
        callee: Box<Expression>,
        type_arguments: Vec<TypeExpression>,
        arguments: Vec<Expression>,
    },
    MethodCall {
        receiver: Box<Expression>,
        method: Identifier,
        type_arguments: Vec<TypeExpression>,
        arguments: Vec<Expression>,
    },
    Field {
        receiver: Box<Expression>,
        field: Identifier,
    },
    Index {
        collection: Box<Expression>,
        index: Box<Expression>,
    },
    Struct {
        path: Path,
        fields: Vec<(Identifier, Expression)>,
    },
    Await {
        expression: Box<Expression>,
    },
    Range {
        range: RangeExpression,
    },
    Try {
        expression: Box<Expression>,
    },
    Yield {
        expression: Option<Box<Expression>>,
    },
}

#[derive(Debug)]
pub enum RangeExpression {
    /// `..`
    Full,
    /// `..end`
    To { end: Box<Expression> },
    /// `..=end`
    ToInclusive { end: Box<Expression> },
    /// `start..`
    From { start: Box<Expression> },
    /// `start..end`
    FromTo {
        start: Box<Expression>,
        end: Box<Expression>,
    },
    /// `start..=end`
    FromToInclusive {
        start: Box<Expression>,
        end: Box<Expression>,
    },
}

#[derive(Debug)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum StatementKind {
    Definition {
        definition: Definition,
    },
    Let {
        pattern: Pattern,
        r#type: Option<TypeExpression>,
        initializer: Option<Expression>,
    },
    Expression {
        value: Expression,
    },
    Semicolon,
}

#[derive(Debug)]
pub struct Pattern {
    pub kind: PatternKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum PatternKind {
    Never,
    Placeholder,
    Path {
        path: Path,
    },
    Mutable {
        pattern: Box<Pattern>,
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
    Range {
        range: RangePattern,
    },
    Rest {
        pattern: Box<Pattern>,
    },
    At {
        name: Identifier,
        pattern: Box<Pattern>,
    },
    Or {
        patterns: Vec<Pattern>,
    },
    Named {
        path: Path,
        fields: Vec<(Identifier, Pattern)>,
    },
    Unnamed {
        elements: Vec<Pattern>,
    },
}

#[derive(Debug)]
pub enum RangePattern {
    /// `start..end`
    FromTo {
        start: Box<Pattern>,
        end: Box<Pattern>,
    },
    /// `start..=end`
    FromToInclusive {
        start: Box<Pattern>,
        end: Box<Pattern>,
    },
    /// `start..`
    From { start: Box<Pattern> },
    /// `..end`
    To { end: Box<Pattern> },
    /// `..=end`
    ToInclusive { end: Box<Pattern> },
}

#[derive(Debug)]
pub struct Literal {
    pub kind: LiteralKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum LiteralKind {
    Boolean { value: bool },
    Character { value: char },
    Float { value: f64 },
    Integer { value: i128 },
    String { segments: Vec<StringSegment> },
}

#[derive(Debug)]
pub enum StringSegment {
    Text { value: String },
    Unicode { value: char },
    Escape { value: char },
    Interpolation { expression: Expression },
}

#[derive(Debug)]
pub enum UnaryOperator {
    /// mut
    Mutable,
    /// +
    Positive,
    /// -
    Negate,
    /// !
    Not,
    /// ~
    BitwiseNot,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum RangeOperator {
    Exclusive,
    Inclusive,
}

#[derive(Debug)]
pub struct TypeExpression {
    pub kind: TypeExpressionKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum TypeExpressionKind {
    Never,
    Mutable { inner: Box<TypeExpression> },
    Path { path: Path },
    Slice { element: Box<TypeExpression> },
    Tuple { elements: Vec<TypeExpression> },
}

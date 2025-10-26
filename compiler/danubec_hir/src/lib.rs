use danubec_symbol::{AttributeId, DefinitionId, LocalId, Symbol};
use danubec_syntax::Span;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Root {
    pub attributes: Vec<TopLevelAttribute>,
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
    pub attributes: Vec<AttributeId>,
    pub visibility: Visibility,
    pub name: Identifier,
    pub kind: DefinitionKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum DefinitionKind {
    Function {
        type_parameters: Vec<TypeParameter>,
        parameters: Vec<FunctionParameter>,
        return_type: Option<TypeExpression>,
        type_bounds: Vec<TypeBound>,
        body: Option<Vec<Statement>>,
    },
    Struct {
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<TypeBound>,
        body: StructBody,
    },
    Enum {
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<TypeBound>,
        variants: Vec<EnumVariant>,
    },
    Module {
        kind: ModuleDefinitionKind,
    },
    Trait {
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<TypeBound>,
        definitions: HashMap<Symbol, Vec<DefinitionId>>,
    },
    Constant {
        r#type: Option<TypeExpression>,
        initializer: Option<Expression>,
    },
    Static {
        r#type: TypeExpression,
        initializer: Expression,
    },
    Type {
        type_parameters: Vec<TypeParameter>,
        type_bounds: Vec<TypeBound>,
        initializer: Option<TypeExpression>,
    },
}

#[derive(Debug)]
pub struct Implement {
    pub attributes: Vec<AttributeId>,
    pub visibility: Visibility,
    pub type_parameters: Vec<TypeParameter>,
    pub trait_type: Option<TypeExpression>,
    pub for_type: TypeExpression,
    pub type_bounds: Vec<TypeBound>,
    pub definitions: HashMap<Symbol, Vec<DefinitionId>>,
    pub span: Span,
}

#[derive(Debug)]
pub enum ModuleDefinitionKind {
    External,
    Inline { definitions: Vec<Definition> },
}

#[derive(Debug, Clone)]
pub enum Visibility {
    Krate,
    Super,
    Self_,
    Restricted(Path),
    Private,
}

#[derive(Debug, Clone, Copy)]
pub struct Identifier {
    pub symbol: Symbol,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Binding {
    Unresolved,
    Definition(DefinitionId),
    Local(LocalId),
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
    pub attributes: Vec<AttributeId>,
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
    pub attributes: Vec<AttributeId>,
    pub name: Identifier,
    pub kind: EnumVariantKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum EnumVariantKind {
    Unit,
    Scalar(Expression),
    Named(Vec<(Vec<AttributeId>, Identifier, TypeExpression)>),
    Unnamed(Vec<(Vec<AttributeId>, TypeExpression)>),
}

#[derive(Debug)]
pub struct Import {
    pub attributes: Vec<AttributeId>,
    pub visibility: Visibility,
    pub path: Path,
    pub kind: ImportKind,
}

#[derive(Debug)]
pub enum ImportKind {
    Glob,
    Symbol(Option<Identifier>),
    List(Vec<Import>),
}

#[derive(Debug, Clone)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub binding: Binding,
}

#[derive(Debug, Clone)]
pub struct PathSegment {
    pub kind: PathSegmentKind,
    pub binding: Binding,
}

#[derive(Debug, Clone)]
pub enum PathSegmentKind {
    Root,
    Self_,
    Super_,
    Krate,
    Identifier(Identifier),
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
        attributes: Vec<AttributeId>,
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
        receiver: Box<Expression>,
        index: Box<Expression>,
    },
    Struct {
        path: Path,
        type_arguments: Vec<TypeExpression>,
        fields: Vec<(Identifier, Expression)>,
    },
    Await {
        expression: Box<Expression>,
    },
    Range {
        range: RangeExpression,
    },
    Try {
        value: Box<Expression>,
    },
    Yield {
        value: Option<Box<Expression>>,
    },
}

#[derive(Debug)]
pub enum RangeExpression {
    /// `..`
    Full,
    /// `..end`
    To { end: Box<Expression> },
    /// `start..end`
    FromTo {
        start: Box<Expression>,
        end: Box<Expression>,
    },
    /// `start..`
    From { start: Box<Expression> },
    /// `start..=end`
    FromToInclusive {
        start: Box<Expression>,
        end: Box<Expression>,
    },
    /// `..=end`
    ToInclusive { end: Box<Expression> },
}

#[derive(Debug)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum StatementKind {
    Definition {
        definition: DefinitionId,
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
    pub mutable: bool,
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
        path: Path,
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
    pub mutable: bool,
    pub kind: TypeExpressionKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum TypeExpressionKind {
    Never,
    Path { path: Path },
    Slice { element: Box<TypeExpression> },
    Tuple { elements: Vec<TypeExpression> },
}

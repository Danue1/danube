use danubec_symbol::Symbol;

use super::{Literal, Path, PathSegment, Pattern, Statement};

pub struct Expression {
    pub kind: ExpressionKind,
}

pub enum ExpressionKind {
    Break,
    Continue,
    For {
        binding: Pattern,
        iterator: Box<Expression>,
        block: Vec<Statement>,
    },
    If {
        condition: Box<Expression>,
        then_block: Vec<Statement>,
        else_block: Option<Box<Expression>>,
    },
    Let {
        binding: Pattern,
        expression: Option<Box<Expression>>,
    },
    Loop(Vec<Statement>),
    Match {
        condition: Box<Expression>,
        arms: Vec<(Pattern, Expression)>,
    },
    Return(Option<Box<Expression>>),
    While {
        condition: Box<Expression>,
        block: Vec<Statement>,
    },

    Array(Vec<Expression>),
    Block(Vec<Statement>),
    Literal(Literal),
    Path(Path),
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },

    Assignment {
        lhs: Box<Expression>,
        operator: AssignmentOperator,
        rhs: Box<Expression>,
    },
    Binary {
        lhs: Box<Expression>,
        operator: BinaryOperator,
        rhs: Box<Expression>,
    },
    Await(Box<Expression>),
    FunctionCall {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    MethodCall {
        receiver: Box<Expression>,
        path: PathSegment,
        arguments: Vec<Expression>,
    },
    Field {
        receiver: Box<Expression>,
        field: Symbol,
    },
    Index {
        receiver: Box<Expression>,
        index: Box<Expression>,
    },
    Range {
        start: Option<Box<Expression>>,
        end: Box<Expression>,
        inclusive: bool,
    },
    Struct {
        path: Path,
        fields: Vec<(Symbol, Expression)>,
    },
    Try(Box<Expression>),
    Yield(Box<Expression>),
}

#[derive(Debug, Clone, Copy)]
pub enum AssignmentOperator {
    /// `=`
    Assign,

    /// `+=`
    AddAssign,
    /// `+|=`
    AddSaturatingAssign,
    /// `+%=`
    AddWrappingAssign,

    /// `-=`
    SubAssign,
    /// `-|=`
    SubSaturatingAssign,
    /// `-%=`
    SubWrappingAssign,

    /// `*=`
    MulAssign,
    /// `*|=`
    MulSaturatingAssign,
    /// `*%=`
    MulWrappingAssign,

    /// `**=`
    ExpAssign,
    /// `**|=`
    ExpSaturatingAssign,
    /// `**%=`
    ExpWrappingAssign,

    /// `/=`
    DivAssign,
    /// `%=`
    RemAssign,

    /// `^=`
    BitXorAssign,
    /// `&=`
    BitAndAssign,
    /// `|=`
    BitOrAssign,
    /// `<<=`
    ShlAssign,
    /// `<<|=`
    ShlSaturatingAssign,
    /// `>>=`
    ShrAssign,
    /// `>>>=`
    ShrWrappingAssign,

    /// `&&=`
    AndAssign,
    /// `||=`
    OrAssign,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    /// `||`
    Or,

    /// `&&`
    And,

    /// `==`
    Equal,
    /// `!=`
    NotEqual,
    /// `<=`
    LessOrEqual,
    /// `<`
    Less,
    /// `>=`
    GreaterOrEqual,
    /// `>`
    Greater,

    /// `|`
    BitwiseOr,

    /// `^`
    BitwiseXor,

    /// `&`
    BitwiseAnd,

    /// `<<`
    ShiftLeft,
    /// `<<|`
    UnsignedShiftLeft,
    /// `>>`
    ShiftRight,
    /// `>>>`
    UnsignedShiftRight,

    /// `+`
    Add,
    /// `+|`
    AddSaturating,
    /// `+%`
    AddWrapping,
    /// `-`
    Sub,
    /// `-|`
    SubSaturating,
    /// `-%`
    SubWrapping,

    /// `**`
    Mul,
    /// `*|`
    MulSaturating,
    /// `*%`
    MulWrapping,
    /// `**`
    Exp,
    /// `**|`
    ExpSaturating,
    /// `**%`
    ExpWrapping,
    /// `/`
    Div,
    /// `%`
    Rem,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    /// `-`
    Negate,
    /// `!`
    Not,
    /// `~`
    BitwiseNot,
}

use super::{Literal, Pattern, Statement};

pub struct Expression {
    pub kind: ExpressionKind,
}

pub enum ExpressionKind {
    Array(Vec<Expression>),
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
    Block(Vec<Statement>),
    Let {
        pattern: Pattern,
        expression: Option<Box<Expression>>,
    },
    Literal(Literal),
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
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

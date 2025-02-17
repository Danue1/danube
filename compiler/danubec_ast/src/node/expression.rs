use super::{Literal, Statement};
use danubec_syntax::SyntaxNode;

pub struct Expression {
    pub syntax: SyntaxNode,
    pub kind: ExpressionKind,
}

pub enum ExpressionKind {
    Assignment {
        lhs: Box<Expression>,
        operator: AssignmentOperator,
        rhs: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Block(Vec<Statement>),
    Let {
        lhs: Box<Expression>,
        rhs: Option<Box<Expression>>,
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

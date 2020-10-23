#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    LeftParens,         // (
    RightParens,        // )
    LeftBracket,        // [
    RightBracket,       // ]
    LeftBrace,          // {
    RightBrace,         // }
    Hashtag,            // #
    ChainArrow,         // |>
    ReturnArrow,        // ->
    BranchArrow,        // =>
    RangeOpen,          // ..=
    RangeClose,         // ..
    Dot,                // .
    Comma,              // ,
    DoubleColon,        // ::
    Colon,              // :
    Semicolon,          // ;
    Equal,              // ==
    Assign,             // =
    NotEqual,           // !=
    AddAssign,          // +=
    SubAssign,          // -=
    ExpAssign,          // **=
    MulAssign,          // *=
    DivAssign,          // /=
    ModAssign,          // %=
    AndAssign,          // &&=
    OrAssign,           // ||=
    Add,                // +
    Sub,                // -
    Exp,                // **
    Mul,                // *
    Div,                // /
    Mod,                // %
    And,                // &&
    Or,                 // ||
    Not,                // !
    Question,           // ?
    BitAndAssign,       // &=
    BitOrAssign,        // |=
    BitXorAssign,       // ^=
    BitLeftAssign,      // <<=
    BitRightAssign,     // >>=
    BitAnd,             // &
    BitOr,              // |
    BitNot,             // ~
    BitXor,             // ^
    BitLeft,            // <<
    BitRight,           // >>
    GreaterThanOrEqual, // >=
    LessThanOrEqual,    // <=
    GreaterThan,        // >
    LessThan,           // <
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorSymbol {
    Equal,              // ==
    NotEqual,           // !=
    Add,                // +
    Sub,                // -
    Exp,                // **
    Mul,                // *
    Div,                // /
    Mod,                // %
    And,                // &&
    Or,                 // ||
    BitAnd,             // &
    BitOr,              // |
    BitNot,             // ~
    BitXor,             // ^
    BitLeft,            // <<
    BitRight,           // >>
    GreaterThanOrEqual, // >=
    LessThanOrEqual,    // <=
    GreaterThan,        // >
    LessThan,           // <
}

impl From<OperatorSymbol> for Symbol {
    fn from(operator: OperatorSymbol) -> Self {
        macro_rules! operator_to_symbol {
            ($($variant:ident,)+) => {
                match operator {
                    $(OperatorSymbol::$variant => Symbol::$variant,)+
                }
            }
        }

        operator_to_symbol! {
            Equal,
            NotEqual,
            Add,
            Sub,
            Exp,
            Mul,
            Div,
            Mod,
            And,
            Or,
            BitAnd,
            BitOr,
            BitNot,
            BitXor,
            BitLeft,
            BitRight,
            GreaterThanOrEqual,
            LessThanOrEqual,
            GreaterThan,
            LessThan,
        }
    }
}

impl From<Symbol> for Option<OperatorSymbol> {
    fn from(symbol: Symbol) -> Self {
        macro_rules! symbol_to_operator {
            ($($variant:ident,)+) => {
              match symbol {
                  $(Symbol::$variant => Some(OperatorSymbol::$variant),)+
                  _ => None
                }
            };
        }

        symbol_to_operator! {
            Equal,
            NotEqual,
            Add,
            Sub,
            Exp,
            Mul,
            Div,
            Mod,
            And,
            Or,
            BitAnd,
            BitOr,
            BitNot,
            BitXor,
            BitLeft,
            BitRight,
            GreaterThanOrEqual,
            LessThanOrEqual,
            GreaterThan,
            LessThan,
        }
    }
}

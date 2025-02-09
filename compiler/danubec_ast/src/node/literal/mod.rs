pub mod array_literal;
pub mod boolean_literal;
pub mod char_literal;
pub mod numeric_literal;
pub mod string_literal;

pub use array_literal::*;
pub use boolean_literal::*;
pub use char_literal::*;
pub use numeric_literal::*;
pub use string_literal::*;

ast_node! {
    /// ```ebnf
    /// Literal =
    /// | ArrayLiteral
    /// | BooleanLiteral
    /// | CharLiteral
    /// | NumericLiteral
    /// | StringLiteral
    /// ```
    enum Literal {
        Array(ArrayLiteral),
        Boolean(BooleanLiteral),
        Char(CharLiteral),
        Numeric(NumericLiteral),
        String(StringLiteral),
    }
}

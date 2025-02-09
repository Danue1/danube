ast_node! {
    /// ```ebnf
    /// BinaryExpression =
    /// | Expression _ "+" _ Expression
    /// | Expression _ "-" _ Expression
    /// | Expression _ "*" _ Expression
    /// | Expression _ "/" _ Expression
    /// ```
    struct BinaryExpression;

    node lhs -> Expression before BinaryOperator;
    token operator -> BinaryOperator;
    node rhs -> Expression after BinaryOperator;
}

ast_node! {
    /// ```ebnf
    /// BinaryOperator =
    /// | "||"
    ///
    /// | "&&"
    ///
    /// | "|"
    ///
    /// | "^"
    ///
    /// | "&"
    ///
    /// | "<<"
    /// | "<="
    /// | "<<|"
    /// | ">>"
    /// | ">>>"
    /// | ">="
    ///
    /// | "+|"
    /// | "+%"
    /// | "+"
    /// | "-|"
    /// | "-%"
    /// | "-"
    ///
    /// | "*|"
    /// | "*%"
    /// | "**"
    /// | "**|"
    /// | "**%"
    /// | "*"
    /// | "/"
    /// | "%"
    /// ```
    struct BinaryOperator;

    token pipe__pipe -> PIPE__PIPE;

    token ampersand__ampersand -> AMPERSAND__AMPERSAND;

    token pipe -> PIPE;

    token caret -> CARET;

    token ampersand -> AMPERSAND;

    token left_chevron__left_chevron -> LEFT_CHEVRON__LEFT_CHEVRON;
    token left_chevron__equal -> LEFT_CHEVRON__EQUAL;
    token left_chevron -> LEFT_CHEVRON;
    token right_chevron__right_chevron -> RIGHT_CHEVRON__RIGHT_CHEVRON;
    token right_chevron__right_chevron__right_chevron -> RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON;
    token right_chevron__equal -> RIGHT_CHEVRON__EQUAL;

    token plus__pipe -> PLUS__PIPE;
    token plus__percent -> PLUS__PERCENT;
    token plus -> PLUS;
    token hyphen__pipe -> HYPHEN__PIPE;
    token hyphen__percent -> HYPHEN__PERCENT;
    token hyphen -> HYPHEN;

    token asterisk__pipe -> ASTERISK__PIPE;
    token asterisk__percent -> ASTERISK__PERCENT;
    token asterisk__asterisk -> ASTERISK__ASTERISK;
    token asterisk__asterisk__pipe -> ASTERISK__ASTERISK__PIPE;
    token asterisk__asterisk__percent -> ASTERISK__ASTERISK__PERCENT;
    token asterisk -> ASTERISK;
    token slash -> SLASH;
    token percent -> PERCENT;
}

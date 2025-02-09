ast_node! {
    /// ```
    /// AssignmentExpression =
    /// | Expression _ AssignmentOperator _ Expression
    /// ```
    struct AssignmentExpression;

    node lhs -> Expression before AssignmentOperator;
    node operator -> AssignmentOperator;
    node rhs -> Expression after AssignmentOperator;
}

ast_node! {
    /// ```
    /// AssignmentOperator =
    /// | "="
    /// | "+="
    /// | "+|="
    /// | "+%="
    /// | "-="
    /// | "-|="
    /// | "-%="
    /// | "*="
    /// | "*|="
    /// | "*%="
    /// | "**="
    /// | "**|="
    /// | "**%="
    /// | "/="
    /// | "%="
    /// | "^="
    /// | "&="
    /// | "&&="
    /// | "|="
    /// | "||="
    /// | "<<="
    /// | "<<|="
    /// | ">>="
    /// | ">>>"
    /// ```
    struct AssignmentOperator;

    token equal -> EQUAL;
    token plus__equal -> PLUS__EQUAL;
    token plus__pipe__equal -> PLUS__PIPE__EQUAL;
    token plus__percent__equal -> PLUS__PERCENT__EQUAL;
    token hyphen__equal -> HYPHEN__EQUAL;
    token hyphen__pipe__equal -> HYPHEN__PIPE__EQUAL;
    token hyphen__percent__equal -> HYPHEN__PERCENT__EQUAL;
    token asterisk__equal -> ASTERISK__EQUAL;
    token asterisk__pipe__equal -> ASTERISK__PIPE__EQUAL;
    token asterisk__percent__equal -> ASTERISK__PERCENT__EQUAL;
    token asterisk__asterisk__equal -> ASTERISK__ASTERISK__EQUAL;
    token asterisk__asterisk__pipe__equal -> ASTERISK__ASTERISK__PIPE__EQUAL;
    token asterisk__asterisk__percent__equal -> ASTERISK__ASTERISK__PERCENT__EQUAL;
    token slash__equal -> SLASH__EQUAL;
    token percent__equal -> PERCENT__EQUAL;
    token caret__equal -> CARET__EQUAL;
    token ampersand__equal -> AMPERSAND__EQUAL;
    token ampersand__ampersand__equal -> AMPERSAND__AMPERSAND__EQUAL;
    token pipe__equal -> PIPE__EQUAL;
    token pipe__pipe__equal -> PIPE__PIPE__EQUAL;
    token left_chevron__left_chevron__equal -> LEFT_CHEVRON__LEFT_CHEVRON__EQUAL;
    token left_chevron__left_chevron__pipe__equal -> LEFT_CHEVRON__LEFT_CHEVRON__PIPE__EQUAL;
    token right_chevron__right_chevron__equal -> RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL;
    token right_chevron__right_chevron__right_chevron__equal -> RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL;
}

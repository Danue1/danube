ast_node! {
    /// ```ebnf
    /// MatchExpression =
    /// | "match" _ Expression _ "{" _ "}"
    /// | "match" _ Expression _ "{" _ MatchArm+ _ "}"
    /// ```
    struct MatchExpression;

    token match_token -> MATCH;
    node expression -> Expression;
    nodes arms -> MatchArm;
}

ast_node! {
    /// ```ebnf
    /// MatchArm =
    /// | Pattern _ "=>" _ Expression
    /// | Pattern _ "=>" _ Expression _ ","
    /// ```
    struct MatchArm;

    node pattern -> Pattern;
    token EQUAL__RIGHT_CHEVRON -> EQUAL__RIGHT_CHEVRON;
    node expression -> Expression;
    token comma -> COMMA;
}

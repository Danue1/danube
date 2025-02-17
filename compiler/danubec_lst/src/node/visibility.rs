ast_node! {
    /// ```ebnf
    /// Visibility =
    /// | "pub"
    /// | "pub" _ "(" _ VisibilityKind _ ")"
    /// ```
    struct Visibility;

    token public -> PUB;
    token left_paren -> LEFT_PAREN;
    node kind -> VisibilityKind;
    token right_paren -> RIGHT_PAREN;
}

ast_node! {
    /// ```ebnf
    /// VisibilityKind =
    /// | VisibilityCrate
    /// | VisibilitySuper
    /// | VisibilityIn
    /// ```
    enum VisibilityKind {
        Crate(VisibilityCrate),
        Super(VisibilitySuper),
        In(VisibilityIn),
    }
}

ast_node! {
    /// ```ebnf
    /// VisibilityCrate =
    /// | "crate"
    /// ```
    struct VisibilityCrate;

    token crate_token -> CRATE;
}

ast_node! {
    /// ```ebnf
    /// VisibilitySuper =
    /// | "super"
    /// ```
    struct VisibilitySuper;

    token super_token -> SUPER;
}

ast_node! {
    /// ```ebnf
    /// VisibilityIn =
    /// | "in" _ Identifier
    /// ```
    struct VisibilityIn;

    token in_token -> IN;
    node path -> Path;
}

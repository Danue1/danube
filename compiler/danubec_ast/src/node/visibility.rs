ast_node! {
    /// ```
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
    /// ```
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
    /// ```
    /// VisibilityCrate =
    /// | "crate"
    /// ```
    struct VisibilityCrate;

    token crate_token -> CRATE;
}

ast_node! {
    /// ```
    /// VisibilitySuper =
    /// | "super"
    /// ```
    struct VisibilitySuper;

    token super_token -> SUPER;
}

ast_node! {
    /// ```
    /// VisibilityIn =
    /// | "in" _ Identifier
    /// ```
    struct VisibilityIn;

    token in_token -> IN;
    // TODO: replace with Path
    node identifier -> Identifier;
}

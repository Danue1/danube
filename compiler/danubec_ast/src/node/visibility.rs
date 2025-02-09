ast_node! {
    /// - pub
    /// - pub(crate)
    /// - pub(super)
    /// - pub(in some::module)
    struct Visibility;

    token public -> PUB;
    token left_paren -> LEFT_PAREN;
    node kind -> VisibilityKind;
    token right_paren -> RIGHT_PAREN;
}

ast_node! {
    enum VisibilityKind {
        Crate(VisibilityCrate),
        Super(VisibilitySuper),
        In(VisibilityIn),
    }
}

ast_node! {
    struct VisibilityCrate;

    token crate_token -> CRATE;
}

ast_node! {
    struct VisibilitySuper;

    token super_token -> SUPER;
}

ast_node! {
    struct VisibilityIn;

    token in_token -> IN;
    // TODO: replace with Path
    node identifier -> Identifier;
}

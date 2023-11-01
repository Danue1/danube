impl super::Parse for danubec_ast::Ast {
    fn parse(cursor: &mut crate::Cursor) -> Result<Self, crate::ParseError> {
        let ast = danubec_ast::Ast::new();

        Ok(ast)
    }
}

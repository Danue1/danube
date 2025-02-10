use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn trait_definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::TRAIT) {
            self.start_node_at(checkpoint, SyntaxKind::TraitDefinition);

            self.trivia(lex);
            self.identifier(lex);

            self.trivia(lex);
            self.type_parameters(lex);

            self.trivia(lex);
            self.where_clause(lex);

            self.trivia(lex);
            self.impl_body(lex);

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn trait_definition() {
    for source in [
        "trait A {}",
        "trait A<T> {}",
        "trait A where T: B {}",
        "trait A where T: B { fn a() {} }",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.trait_definition(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

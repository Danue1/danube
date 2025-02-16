use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn impl_definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::IMPL) {
            self.start_node_at(checkpoint, SyntaxKind::ImplDefinition);

            self.trivia(lex);
            self.type_parameters(lex);

            self.trivia(lex);
            self.ty(lex);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::FOR);

            self.trivia(lex);
            self.ty(lex);

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

    pub fn impl_body(&mut self, lex: &mut Lex) -> bool {
        if expect!(self, lex, SyntaxKind::LEFT_BRACE) {
            self.trivia(lex);
            while self.impl_item_kind(lex) {
                self.trivia(lex);
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_BRACE);

            true
        } else {
            false
        }
    }

    fn impl_item_kind(&mut self, lex: &mut Lex) -> bool {
        self.function_definition(lex)
            || self.type_definition(lex)
            || self.const_definition(lex)
            || self.static_definition(lex)
    }
}

#[test]
fn impl_definition() {
    for source in [
        "impl A for B { }",
        "impl A for B { fn a() { } }",
        "impl A for B { type a = B; }",
        "impl A for B { const a: B = 3; }",
        "impl A for B { static a: B = 3; }",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.impl_definition(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

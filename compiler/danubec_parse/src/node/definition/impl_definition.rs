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

            let checkpoint = self.checkpoint();
            if expect!(self, lex, SyntaxKind::FOR) {
                self.start_node_at(checkpoint, SyntaxKind::TargetType);

                self.trivia(lex);
                self.ty(lex);

                self.finish_node();
            }

            self.trivia(lex);
            self.where_clause(lex);

            self.trivia(lex);
            self.associated_definitions(lex);

            self.finish_node();

            true
        } else {
            false
        }
    }

    pub fn associated_definitions(&mut self, lex: &mut Lex) -> bool {
        if expect!(self, lex, SyntaxKind::LEFT_BRACE) {
            self.trivia(lex);
            while self.associated_definition(lex) {
                self.trivia(lex);
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_BRACE);

            true
        } else {
            false
        }
    }

    fn associated_definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        let mut matched = false;
        if self.visibility(lex) {
            self.trivia(lex);
            matched = true;
        }

        if self.associated_definition_kind(lex) {
            matched = true;
        }

        if matched {
            self.start_node_at(checkpoint, SyntaxKind::AssociatedItem);
            self.finish_node();
        }

        matched
    }

    fn associated_definition_kind(&mut self, lex: &mut Lex) -> bool {
        self.function_definition(lex) || self.type_definition(lex) || self.const_definition(lex)
    }
}

#[test]
fn impl_definition() {
    for source in [
        "impl A for B { }",
        "impl A for B { fn a() { } }",
        "impl A for B { type a = B; }",
        "impl A for B { const a: B = 3; }",
        "impl A for B { pub fn a() { } }",
        "impl A for B { pub type a = B; }",
        "impl A for B { pub const a: B = 3; }",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.impl_definition(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

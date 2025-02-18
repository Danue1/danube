use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn const_definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::CONST) {
            self.start_node_at(checkpoint, SyntaxKind::ConstDefinition);

            self.trivia(lex);
            self.pattern(lex);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::COLON);

            self.trivia(lex);
            self.ty(lex);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::EQUAL);

            self.trivia(lex);
            self.expression(lex);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::SEMICOLON);

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn const_definition() {
    for source in ["const a: i32 = 42;", "const a: i32 = 42"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.const_definition(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

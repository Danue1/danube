use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn static_definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::STATIC) {
            self.start_node_at(checkpoint, SyntaxKind::StaticDefinition);

            self.trivia(lex);
            self.identifier(lex);

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
fn static_definition() {
    for source in ["static a: b = 3;", "static a: b = 3 + 4;"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.static_definition(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

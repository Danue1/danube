use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn let_statement(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LET) {
            self.start_node_at(checkpoint, SyntaxKind::LetStatement);

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
fn let_statement() {
    for source in [
        "let a;",
        // "let b = false;",
        // "let c: i32 = false;",
        // "let d: i32;",
        // "let e: i32 = false;",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.let_statement(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}

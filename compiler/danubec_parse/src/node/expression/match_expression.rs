use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn match_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::MATCH) {
            self.start_node_at(checkpoint, SyntaxKind::MatchExpression);

            self.trivia(lex);
            self.expression(lex);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::LEFT_BRACE);

            self.trivia(lex);
            while self.match_arm(lex) {
                self.trivia(lex);
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_BRACE);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn match_arm(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::MatchArm) {
            self.start_node_at(checkpoint, SyntaxKind::MatchArm);

            self.trivia(lex);
            self.pattern(lex);

            self.trivia(lex);
            expect!(self, lex, token -> EQUAL__RIGHT_CHEVRON, SyntaxKind::EQUAL, SyntaxKind::RIGHT_CHEVRON,);

            self.trivia(lex);
            self.expression(lex);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::COMMA);

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn match_expression() {
    for source in [
        "match 1 { 1 => 1 }",
        "match 1 { 1 => 1, }",
        "match 1 { 1 => 1, 2 => 2 }",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.match_expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

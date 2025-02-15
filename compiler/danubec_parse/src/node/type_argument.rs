use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn type_argument(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LEFT_CHEVRON) {
            self.start_node_at(checkpoint, SyntaxKind::TypeArgument);

            self.trivia(lex);
            while self.ty(lex) {
                self.trivia(lex);
                if expect!(self, lex, SyntaxKind::COMMA) {
                    self.trivia(lex);
                } else {
                    break;
                }
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_CHEVRON);

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn type_argument() {
    for source in ["<>", "<a>", "<a,>", "<a, b>", "<a::b>"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.type_argument(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

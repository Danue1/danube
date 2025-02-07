use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn identifier(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::UNDERSCORE | SyntaxKind::ALPHABETIC) {
            self.start_node_at(checkpoint, SyntaxKind::Identifier);

            consume_while!(
                self,
                lex,
                SyntaxKind::UNDERSCORE | SyntaxKind::ALPHABETIC | SyntaxKind::NUMERIC
            );

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn identifier() {
    for source in ["abc", "abc123", "abc123_", "abc_123", "_abc123", "_abc123_"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.identifier(&mut lex);
        let node = context.finish();

        assert_eq!(node.kind(), SyntaxKind::Identifier);
        assert_eq!(format!("{}", node), source);
    }
}

use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn semicolon_statement(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::SEMICOLON) {
            self.start_node_at(checkpoint, SyntaxKind::SemicolonStatement);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn semicolon_statement() {
    for source in [";"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.semicolon_statement(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}

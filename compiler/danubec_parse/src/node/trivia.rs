use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn trivia(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(
            self,
            lex,
            SyntaxKind::WHITESPACE | SyntaxKind::NEW_LINE | SyntaxKind::TAB
        ) {
            self.start_node_at(checkpoint, SyntaxKind::Trivia);

            consume_while!(
                self,
                lex,
                SyntaxKind::WHITESPACE | SyntaxKind::NEW_LINE | SyntaxKind::TAB
            );

            self.finish_node();

            true
        } else {
            false
        }
    }
}

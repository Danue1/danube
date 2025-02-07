use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn root(&mut self, lex: &mut Lex) {
        self.start_node(SyntaxKind::Root);
        self.definitions(lex);
        self.finish_node();
    }
}

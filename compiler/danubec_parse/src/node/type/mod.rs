mod path_type;

use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn ty(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        let matched = self.path_type(lex);

        if matched {
            self.start_node_at(checkpoint, SyntaxKind::Type);
            self.finish_node();
        }

        matched
    }
}

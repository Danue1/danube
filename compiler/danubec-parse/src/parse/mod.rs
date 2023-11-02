mod ident_node;
mod item_node;
mod path_node;
mod type_node;

use crate::context::State;
use danubec_syntax_kind::SyntaxKind;

impl crate::context::Context {
    #[inline]
    pub fn ast(&mut self) {
        self.start_node(SyntaxKind::AST_NODE);
        self.skip_whitespace();
        while !self.is_empty() {
            if self.item_node() == State::Continue {
                self.unexpected_token();
            }
            self.skip_whitespace();
        }
        self.finish_node();
    }

    fn unexpected_token(&mut self) {
        self.start_node(SyntaxKind::ERROR);
        self.bump();
        self.finish_node();
    }
}

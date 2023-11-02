use crate::State;
use danubec_syntax_kind::SyntaxKind;

impl crate::Context {
    pub fn ident_node(&mut self) -> State {
        guard!(self, IDENT_KEYWORD, IDENT_NODE);
        self.finish_node();
        State::Stop
    }
}

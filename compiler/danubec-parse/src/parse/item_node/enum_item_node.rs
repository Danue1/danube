use crate::context::State;
use danubec_syntax_kind::SyntaxKind;

impl crate::Context {
    pub fn enum_item_node(&mut self) -> State {
        guard!(self, ENUM_KEYWORD, ENUM_ITEM_NODE);
        self.skip_whitespace();
        self.ident_node();

        State::Stop
    }
}

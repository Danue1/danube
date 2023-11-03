use crate::State;
use danubec_syntax_kind::SyntaxKind;

impl crate::Context {
    pub fn const_item_node(&mut self) -> State {
        guard!(self, CONST_KEYWORD, CONST_ITEM_NODE);
        self.skip_whitespace();
        if self.ident_node() == State::Stop {
            self.skip_whitespace();
        }
        if expect!(self, SyntaxKind::COLON) {
            self.skip_whitespace();
        }
        if self.type_node() == State::Stop {
            self.skip_whitespace();
        }
        if expect!(self, SyntaxKind::EQUAL) {
            self.skip_whitespace();
        }
        if self.expression_node() == State::Stop {
            self.skip_whitespace();
        }
        expect!(self, SyntaxKind::SEMICOLON);

        self.finish_node();
        State::Stop
    }
}

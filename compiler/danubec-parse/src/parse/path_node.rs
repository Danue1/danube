use crate::State;
use danubec_syntax_kind::SyntaxKind;

impl crate::Context {
    pub fn path_node(&mut self) -> State {
        guard!(self, IDENT_KEYWORD, PATH_NODE, ident_node);

        while !self.is_eof() {
            if expect!(self, SyntaxKind::COLON) {
                if expect!(self, SyntaxKind::COLON) {
                    self.skip_whitespace();
                    self.ident_node();
                } else {
                    self.unexpected_token();
                }
            } else {
                break;
            }
        }

        self.finish_node();
        State::Stop
    }
}

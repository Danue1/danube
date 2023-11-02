use crate::State;
use danubec_syntax_kind::SyntaxKind;

impl crate::Context {
    pub fn type_node(&mut self) -> State {
        let checkpoint = self.checkpoint();
        if self.unnamed_type_kind_node() == State::Continue
            && self.path_type_kind_node() == State::Continue
        {
            State::Continue
        } else {
            self.start_node_at(checkpoint, SyntaxKind::TYPE_NODE);
            self.finish_node();
            State::Stop
        }
    }

    // ()
    // (type1)
    // (type1, type2)
    pub fn unnamed_type_kind_node(&mut self) -> State {
        guard!(self, LEFT_PAREN, UNNAMED_TYPE_KIND_NODE);
        self.skip_whitespace();

        while !self.is_eof() {
            if expect!(self, SyntaxKind::RIGHT_PAREN) {
                break;
            }

            let checkpoint = self.checkpoint();
            if self.type_node() == State::Continue {
                self.unexpected_token();
            } else {
                self.start_node_at(checkpoint, SyntaxKind::UNNAMED_TYPE_KIND_NODE);
                self.finish_node();
            }
            self.skip_whitespace();
            if expect!(self, SyntaxKind::COMMA) {
                self.skip_whitespace();
            }
        }

        self.finish_node();
        State::Stop
    }

    // Foo
    // Foo::Bar
    pub fn path_type_kind_node(&mut self) -> State {
        let checkpoint = self.checkpoint();
        if self.path_node() == State::Continue {
            return State::Continue;
        }

        self.start_node_at(checkpoint, SyntaxKind::PATH_TYPE_KIND_NODE);
        self.finish_node();
        State::Stop
    }
}

mod const_item_node;
mod enum_item_node;
mod struct_item_node;

use crate::State;
use danubec_syntax_kind::SyntaxKind;

impl crate::Context {
    pub fn item_node(&mut self) -> crate::State {
        one_of!(
            self.struct_item_node(),
            self.enum_item_node(),
            self.const_item_node()
        )
    }

    pub fn named_field_node(&mut self) -> State {
        guard!(self, IDENT_KEYWORD, NAMED_FIELD_NODE, ident_node);
        self.skip_whitespace();
        expect!(self, SyntaxKind::COLON);
        self.skip_whitespace();
        self.type_node();
        self.skip_whitespace();
        expect!(self, SyntaxKind::COMMA);

        self.finish_node();
        State::Stop
    }

    pub fn unnamed_fields_node(&mut self) -> State {
        guard!(self, LEFT_PAREN, UNNAMED_FIELDS_NODE);
        self.skip_whitespace();

        while !self.is_eof() {
            if expect!(self, SyntaxKind::RIGHT_PAREN) {
                break;
            }

            if self.unnamed_field_node() == State::Continue {
                self.unexpected_token();
            }
            self.skip_whitespace();
            if expect!(self, SyntaxKind::COMMA) {
                self.skip_whitespace();
            }
        }

        self.finish_node();
        State::Stop
    }

    pub fn unnamed_field_node(&mut self) -> State {
        let checkpoint = self.checkpoint();
        if self.type_node() == State::Continue {
            State::Continue
        } else {
            self.start_node_at(checkpoint, SyntaxKind::UNNAMED_FIELD_NODE);
            self.finish_node();
            State::Stop
        }
    }
}

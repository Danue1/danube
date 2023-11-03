use danubec_syntax_kind::SyntaxKind;

use crate::State;

impl crate::Context {
    pub fn expression_literal_node(&mut self) -> State {
        one_of!(
            self.expression_literal_bool_node(),
            self.expression_literal_char_node(),
            self.expression_literal_number_node(),
            self.expression_literal_string_node()
        )
    }

    pub fn expression_literal_bool_node(&mut self) -> State {
        match self.peek_text() {
            "true" | "false" => {
                self.start_node(SyntaxKind::EXPRESSION_LITERAL_BOOL_NODE);
                self.bump();
                self.finish_node();
                State::Stop
            }
            _ => State::Continue,
        }
    }

    pub fn expression_literal_char_node(&mut self) -> State {
        guard!(self, CHAR, EXPRESSION_LITERAL_CHAR_NODE);
        self.finish_node();
        State::Stop
    }

    pub fn expression_literal_number_node(&mut self) -> State {
        if self.peek() != SyntaxKind::NUMERIC {
            return State::Continue;
        }

        if self.look_ahead(1) == SyntaxKind::DOT {
            if self.look_ahead(2) == SyntaxKind::NUMERIC {
                self.start_node(SyntaxKind::EXPRESSION_LITERAL_FLOAT_NODE);
                self.bump();
                self.bump();
                self.bump();
                self.finish_node();
                return State::Stop;
            }
        }

        self.start_node(SyntaxKind::EXPRESSION_LITERAL_INTEGER_NODE);
        self.bump();
        self.finish_node();
        State::Stop
    }

    pub fn expression_literal_string_node(&mut self) -> State {
        guard!(self, STRING, EXPRESSION_LITERAL_STRING_NODE);
        self.finish_node();
        State::Stop
    }
}

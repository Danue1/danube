use crate::pratt::Bp;
use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

impl crate::Context {
    pub fn assignment_expression(
        &mut self,
        lex: &mut Lex,
        checkpoint: Checkpoint,
        binding_power: Bp,
    ) -> bool {
        if self.assignment_operator(lex) {
            self.start_node_at(checkpoint, SyntaxKind::AssignmentExpression);

            self.trivia(lex);
            self.expression_bp(lex, binding_power);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn assignment_operator(&mut self, lex: &mut Lex) -> bool {
        macro_rules! match_operator {
            ($(($($punctuation:ident),+) => $kind:ident,)+) => {
                let checkpoint = self.checkpoint();

                if $(
                    expect!(self, lex, $kind, $(SyntaxKind::$punctuation,)+)
                )||+ {
                    self.start_node_at(checkpoint, SyntaxKind::AssignmentOperator);
                    self.finish_node();

                    true
                } else {
                    false
                }
            };
        }

        match_operator! {
            (EQUAL) => EQUAL,
            (PLUS, EQUAL) => PLUS__EQUAL,
            (PLUS, PIPE, EQUAL) => PLUS__PIPE__EQUAL,
            (PLUS, PERCENT, EQUAL) => PLUS__PERCENT__EQUAL,
            (HYPHEN, EQUAL) => HYPHEN__EQUAL,
            (HYPHEN, PIPE, EQUAL) => HYPHEN__PIPE__EQUAL,
            (HYPHEN, PERCENT, EQUAL) => HYPHEN__PERCENT__EQUAL,
            (ASTERISK, EQUAL) => ASTERISK__EQUAL,
            (ASTERISK, PIPE, EQUAL) => ASTERISK__PIPE__EQUAL,
            (ASTERISK, PERCENT, EQUAL) => ASTERISK__PERCENT__EQUAL,
            (ASTERISK, ASTERISK, EQUAL) => ASTERISK__ASTERISK__EQUAL,
            (ASTERISK, ASTERISK, PIPE, EQUAL) => ASTERISK__ASTERISK__PIPE__EQUAL,
            (ASTERISK, ASTERISK, PERCENT, EQUAL) => ASTERISK__ASTERISK__PERCENT__EQUAL,
            (SLASH, EQUAL) => SLASH__EQUAL,
            (PERCENT, EQUAL) => PERCENT__EQUAL,
            (CARET, EQUAL) => CARET__EQUAL,
            (AMPERSAND, EQUAL) => AMPERSAND__EQUAL,
            (AMPERSAND, AMPERSAND, EQUAL) => AMPERSAND__AMPERSAND__EQUAL,
            (PIPE, EQUAL) => PIPE__EQUAL,
            (PIPE, PIPE, EQUAL) => PIPE__PIPE__EQUAL,
            (LEFT_CHEVRON, LEFT_CHEVRON, EQUAL) => LEFT_CHEVRON__LEFT_CHEVRON__EQUAL,
            (LEFT_CHEVRON, LEFT_CHEVRON, PIPE, EQUAL) => LEFT_CHEVRON__LEFT_CHEVRON__PIPE__EQUAL,
            (RIGHT_CHEVRON, RIGHT_CHEVRON, EQUAL) => RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL,
            (RIGHT_CHEVRON, RIGHT_CHEVRON, RIGHT_CHEVRON, EQUAL) => RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL,
        }
    }
}

#[test]
fn assignment_expression() {
    for source in [
        "=", "+=", "+|=", "+%=", "-=", "-|=", "-%=", "*=", "*|=", "*%=", "**=", "**|=", "**%=",
        "/=", "%=", "^=", "&=", "&&=", "|=", "||=", "<<=", "<<|=", ">>=", ">>>=",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.assignment_operator(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}

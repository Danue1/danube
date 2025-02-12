use crate::pratt::Bp;
use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

impl crate::Context {
    pub fn binary_expression(
        &mut self,
        lex: &mut Lex,
        checkpoint: Checkpoint,
        binding_power: Bp,
    ) -> bool {
        if self.binary_operator(lex) {
            self.start_node_at(checkpoint, SyntaxKind::BinaryExpression);

            self.trivia(lex);
            self.expression_bp(lex, binding_power);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn binary_operator(&mut self, lex: &mut Lex) -> bool {
        macro_rules! match_operator {
            ($(($($punctuation:ident),+) => $kind:ident,)+) => {
                let checkpoint = self.checkpoint();

                if $(
                    expect!(self, lex, token -> $kind, $(SyntaxKind::$punctuation,)+)
                )||+ {
                    self.start_node_at(checkpoint, SyntaxKind::BinaryOperator);
                    self.finish_node();

                    true
                } else {
                    false
                }
            };
        }

        match_operator! {
            (PIPE, PIPE) => PIPE__PIPE,

            (AMPERSAND, AMPERSAND) => AMPERSAND__AMPERSAND,

            (PIPE) => PIPE,

            (CARET) => CARET,

            (AMPERSAND) => AMPERSAND,

            (LEFT_CHEVRON, LEFT_CHEVRON) => LEFT_CHEVRON__LEFT_CHEVRON,
            (LEFT_CHEVRON, EQUAL) => LEFT_CHEVRON__EQUAL,
            (LEFT_CHEVRON) => LEFT_CHEVRON,
            (RIGHT_CHEVRON, RIGHT_CHEVRON, RIGHT_CHEVRON) => RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON,
            (RIGHT_CHEVRON, RIGHT_CHEVRON) => RIGHT_CHEVRON__RIGHT_CHEVRON,
            (RIGHT_CHEVRON, EQUAL) => RIGHT_CHEVRON__EQUAL,

            (PLUS, PIPE) => PLUS__PIPE,
            (PLUS, PERCENT) => PLUS__PERCENT,
            (PLUS) => PLUS,
            (HYPHEN, PIPE) => HYPHEN__PIPE,
            (HYPHEN, PERCENT) => HYPHEN__PERCENT,
            (HYPHEN) => HYPHEN,

            (ASTERISK, PIPE) => ASTERISK__PIPE,
            (ASTERISK, PERCENT) => ASTERISK__PERCENT,
            (ASTERISK, ASTERISK, PIPE) => ASTERISK__ASTERISK__PIPE,
            (ASTERISK, ASTERISK, PERCENT) => ASTERISK__ASTERISK__PERCENT,
            (ASTERISK, ASTERISK) => ASTERISK__ASTERISK,
            (ASTERISK) => ASTERISK,
            (SLASH) => SLASH,
            (PERCENT) => PERCENT,
        }
    }
}

#[test]
fn expression() {
    for source in ["1 + 2 * 3 - 4"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

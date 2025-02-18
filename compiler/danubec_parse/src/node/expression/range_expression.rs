use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

impl crate::Context {
    pub fn range_expression_rhs(&mut self, lex: &mut Lex, checkpoint: Checkpoint) -> bool {
        if self.range_operator(lex) {
            self.start_node_at(checkpoint, SyntaxKind::RangeExpressionRhs);

            self.trivia(lex);
            self.expression(lex);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn range_operator(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, token -> DOT__DOT__EQUAL, SyntaxKind::DOT, SyntaxKind::DOT, SyntaxKind::EQUAL,)
            || expect!(self, lex, token -> DOT__DOT, SyntaxKind::DOT, SyntaxKind::DOT,)
        {
            self.start_node_at(checkpoint, SyntaxKind::RangeOperator);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn range_expression_rhs() {
    for source in ["..", "..foo", "..=foo", "foo..", "foo..bar", "foo..=bar"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

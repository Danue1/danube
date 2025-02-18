use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

impl crate::Context {
    pub fn yield_expression(&mut self, lex: &mut Lex, checkpoint: Checkpoint) -> bool {
        if expect!(self, lex, SyntaxKind::YIELD) {
            self.start_node_at(checkpoint, SyntaxKind::YieldExpression);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn yield_expression() {
    for source in ["foo.yield"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.expression(&mut lex);
        let node = context.finish();

        dbg!(&node);
        assert_eq!(node.to_string(), source);
    }
}

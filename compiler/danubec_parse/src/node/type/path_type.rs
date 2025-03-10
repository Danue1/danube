use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn path_type(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.path(lex) {
            self.start_node_at(checkpoint, SyntaxKind::PathType);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn path_type() {
    for source in ["foo", "foo::bar", "foo::bar::baz"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.path_type(&mut lex);
        let node = context.finish();

        assert_eq!(node.kind(), SyntaxKind::PathType.into());
        assert_eq!(format!("{}", node), source);
    }
}

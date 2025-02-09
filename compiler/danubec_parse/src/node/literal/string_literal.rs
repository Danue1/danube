use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn string_literal(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::DOUBLE_QUOTE) {
            self.start_node_at(checkpoint, SyntaxKind::StringLiteral);
            while !expect!(self, lex, SyntaxKind::DOUBLE_QUOTE) {
                if !self.string_literal_fragment(lex) {
                    self.error(lex);
                }
            }
            self.finish_node();

            true
        } else {
            false
        }
    }

    fn string_literal_fragment(&mut self, lex: &mut Lex) -> bool {
        // TODO: Implement `interpolation` and `escape`
        // self.interpolation(lex) ||
        // self.escape(lex) ||
        self.raw(lex)
    }

    fn raw(&mut self, lex: &mut Lex) -> bool {
        consume_while!(
            self,
            lex,
            Raw,
            SyntaxKind::LEFT_BRACE | SyntaxKind::BACKSLASH
        )
    }
}

#[test]
fn string_literal() {
    for source in [
        r#""Hello, World!""#,
        // r#""Hello, {name}!""#,
        // r#""Hello, \nWorld!""#,
        // r#""Hello, \n{name}!""#,
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.string_literal(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}

use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn module_definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::MOD) {
            self.start_node_at(checkpoint, SyntaxKind::ModuleDefinition);

            self.trivia(lex);
            self.identifier(lex);

            self.trivia(lex);
            if expect!(self, lex, SyntaxKind::SEMICOLON) {
                self.trivia(lex);
                self.ty(lex);
            } else {
                if expect!(self, lex, SyntaxKind::LEFT_BRACE) {
                    self.trivia(lex);
                    while self.definition(lex) {
                        self.trivia(lex);
                    }
                    self.trivia(lex);
                    expect!(self, lex, SyntaxKind::RIGHT_BRACE);
                }
            }

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn module_definition() {
    for source in ["mod a;", "mod a { }", "mod a { type b = c; }"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.module_definition(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn function_definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::FN) {
            self.start_node_at(checkpoint, SyntaxKind::FunctionDefinition);

            self.identifier(lex);

            self.trivia(lex);
            self.function_parameters(lex);

            self.trivia(lex);
            self.function_return_type(lex);

            self.trivia(lex);
            self.function_body(lex);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn function_parameters(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LEFT_PAREN) {
            self.start_node_at(checkpoint, SyntaxKind::FunctionParameter);

            self.trivia(lex);

            while self.function_parameter(lex) {
                self.trivia(lex);
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_PAREN);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn function_parameter(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.identifier(lex) {
            self.start_node_at(checkpoint, SyntaxKind::FunctionParameter);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::COLON);

            self.trivia(lex);
            self.ty(lex);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::COMMA);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn function_return_type(&mut self, lex: &mut Lex) -> bool {
        if expect!(
            self,
            lex,
            HYPHEN__RIGHT_CHEVRON,
            SyntaxKind::HYPHEN,
            SyntaxKind::RIGHT_CHEVRON,
        ) {
            self.trivia(lex);
            self.ty(lex);

            true
        } else {
            false
        }
    }

    fn function_body(&mut self, lex: &mut Lex) -> bool {
        self.block_expression(lex) || expect!(self, lex, SyntaxKind::SEMICOLON)
    }
}

#[test]
fn function_parameters() {
    for source in ["(a: i32)", "(a: i32, b: i32)", "(a: i32, b: i32,)"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.function_parameters(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}

#[test]
fn function_return_type() {
    for source in ["-> i32"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.start_node(SyntaxKind::Root);
        context.function_return_type(&mut lex);
        context.finish_node();
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}

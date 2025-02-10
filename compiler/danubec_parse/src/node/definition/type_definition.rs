use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn type_definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::TYPE) {
            self.start_node_at(checkpoint, SyntaxKind::TypeDefinition);

            self.trivia(lex);
            self.identifier(lex);

            self.trivia(lex);
            self.type_parameters(lex);

            self.trivia(lex);
            self.type_body(lex);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::SEMICOLON);

            self.finish_node();

            true
        } else {
            false
        }
    }

    pub fn type_parameters(&mut self, lex: &mut Lex) -> bool {
        if expect!(self, lex, SyntaxKind::LEFT_CHEVRON) {
            self.trivia(lex);

            while self.type_parameter(lex) {
                self.trivia(lex);
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_CHEVRON);

            true
        } else {
            false
        }
    }

    fn type_parameter(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.identifier(lex) {
            self.start_node_at(checkpoint, SyntaxKind::TypeParameter);

            self.trivia(lex);
            if expect!(self, lex, SyntaxKind::COLON) {
                self.trivia(lex);
                while self.ty(lex) {
                    self.trivia(lex);
                    if expect!(self, lex, SyntaxKind::PLUS) {
                        self.trivia(lex);
                    } else {
                        break;
                    }
                }
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::COMMA);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn type_body(&mut self, lex: &mut Lex) -> bool {
        if expect!(self, lex, SyntaxKind::EQUAL) {
            self.trivia(lex);
            self.ty(lex);

            true
        } else {
            false
        }
    }

    pub fn where_clause(&mut self, lex: &mut Lex) -> bool {
        if expect!(self, lex, SyntaxKind::WHERE) {
            self.trivia(lex);
            while self.type_constraint(lex) {
                self.trivia(lex);
                if expect!(self, lex, SyntaxKind::COMMA) {
                    self.trivia(lex);
                } else {
                    break;
                }
            }

            true
        } else {
            false
        }
    }

    fn type_constraint(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.ty(lex) {
            self.start_node_at(checkpoint, SyntaxKind::TypeConstraint);

            self.trivia(lex);
            if expect!(self, lex, SyntaxKind::COLON) {
                self.trivia(lex);
                self.type_constraint_parameter(lex);
            }

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn type_constraint_parameter(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.ty(lex) {
            self.start_node_at(checkpoint, SyntaxKind::TypeConstraintParameter);

            self.trivia(lex);
            if expect!(self, lex, SyntaxKind::PLUS) {
                self.trivia(lex);
                while self.ty(lex) {
                    self.trivia(lex);
                    if expect!(self, lex, SyntaxKind::PLUS) {
                        self.trivia(lex);
                    } else {
                        break;
                    }
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
fn type_parameters() {
    for source in [
        "<>",
        "<T>",
        "<T,>",
        "<T: U>",
        "<T: U +>",
        "<T, U>",
        "<T: U + V, W>",
    ] {
        let mut lex = Lex::new(source);
        let mut context = crate::Context::new();
        context.start_node(SyntaxKind::Root);
        context.type_parameters(&mut lex);
        context.finish_node();
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

#[test]
fn where_clause() {
    for source in [
        "where",
        "where T",
        "where T, U",
        "where T: U, V",
        "where T: U +",
        "where T: U + V",
    ] {
        let mut lex = Lex::new(source);
        let mut context = crate::Context::new();
        context.start_node(SyntaxKind::Root);
        context.where_clause(&mut lex);
        context.finish_node();
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

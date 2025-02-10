use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn struct_definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::STRUCT) {
            self.start_node_at(checkpoint, SyntaxKind::StructDefinition);

            self.trivia(lex);
            self.identifier(lex);

            self.trivia(lex);
            self.type_parameters(lex);

            self.trivia(lex);
            self.where_clause(lex);

            self.trivia(lex);
            if !expect!(self, lex, SyntaxKind::SEMICOLON) {
                self.struct_body_kind(lex);
            }

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn struct_body_kind(&mut self, lex: &mut Lex) -> bool {
        self.struct_body_named(lex) || self.struct_body_unnamed(lex)
    }

    fn struct_body_named(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LEFT_BRACE) {
            self.start_node_at(checkpoint, SyntaxKind::StructBodyNamed);

            self.trivia(lex);
            while self.struct_body_named_field(lex) {
                self.trivia(lex);
                if expect!(self, lex, SyntaxKind::COMMA) {
                    self.trivia(lex);
                } else {
                    break;
                }
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_BRACE);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn struct_body_named_field(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        let mut like_field = false;
        if self.visibility(lex) {
            self.trivia(lex);
            like_field = true;
        }

        if self.identifier(lex) {
            self.trivia(lex);
            expect!(self, lex, SyntaxKind::COLON);
            self.trivia(lex);
            self.ty(lex);

            like_field = true;
        }

        if like_field {
            self.start_node_at(checkpoint, SyntaxKind::StructBodyNamedField);
            self.finish_node();
        }

        like_field
    }

    fn struct_body_unnamed(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LEFT_PAREN) {
            self.start_node_at(checkpoint, SyntaxKind::StructBodyUnnamed);

            self.trivia(lex);
            while self.struct_body_unnamed_field(lex) {
                self.trivia(lex);
                if expect!(self, lex, SyntaxKind::COMMA) {
                    self.trivia(lex);
                } else {
                    break;
                }
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_PAREN);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn struct_body_unnamed_field(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        let mut like_field = false;
        if self.visibility(lex) {
            self.trivia(lex);
            like_field = true;
        }

        if self.identifier(lex) {
            self.trivia(lex);
            expect!(self, lex, SyntaxKind::COLON);

            self.trivia(lex);
            self.ty(lex);

            like_field = true;
        }

        if like_field {
            self.start_node_at(checkpoint, SyntaxKind::StructBodyUnnamedField);
            self.finish_node();
        }

        like_field
    }
}

#[test]
fn struct_definition() {
    for source in [
        "struct A;",
        "struct A { a: b, c: d }",
        "struct A { a: b, c: d, }",
        "struct A<T>{ a: b, c: d }",
        "struct A<T>(a: b, c: d)",
        "struct A<T> where T: U (a: b, c: d)",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.struct_definition(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

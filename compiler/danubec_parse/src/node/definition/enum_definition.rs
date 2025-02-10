use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn enum_definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::ENUM) {
            self.start_node_at(checkpoint, SyntaxKind::EnumDefinition);

            self.trivia(lex);
            self.identifier(lex);

            self.trivia(lex);
            self.enum_body(lex);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn enum_body(&mut self, lex: &mut Lex) -> bool {
        if expect!(self, lex, SyntaxKind::LEFT_BRACE) {
            self.trivia(lex);
            while self.enum_variant(lex) {
                self.trivia(lex);
                if expect!(self, lex, SyntaxKind::COMMA) {
                    self.trivia(lex);
                } else {
                    break;
                }
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_BRACE);

            true
        } else {
            false
        }
    }

    fn enum_variant(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.identifier(lex) {
            self.start_node_at(checkpoint, SyntaxKind::EnumVariant);

            self.trivia(lex);
            self.enum_variant_kind(lex);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn enum_variant_kind(&mut self, lex: &mut Lex) -> bool {
        self.enum_variant_named(lex)
            || self.enum_variant_unnamed(lex)
            || self.enum_variant_sequence(lex)
    }

    fn enum_variant_named(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LEFT_BRACE) {
            self.start_node_at(checkpoint, SyntaxKind::EnumVariantNamed);

            self.trivia(lex);
            while self.enum_variant_named_field(lex) {
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

    fn enum_variant_named_field(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.identifier(lex) {
            self.start_node_at(checkpoint, SyntaxKind::EnumVariantNamedField);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::COLON);

            self.trivia(lex);
            self.ty(lex);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn enum_variant_unnamed(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LEFT_PAREN) {
            self.start_node_at(checkpoint, SyntaxKind::EnumVariantUnnamed);

            self.trivia(lex);
            while self.enum_variant_unnamed_field(lex) {
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

    fn enum_variant_unnamed_field(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.ty(lex) {
            self.start_node_at(checkpoint, SyntaxKind::EnumVariantUnnamedField);
            self.finish_node();

            true
        } else {
            false
        }
    }

    fn enum_variant_sequence(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::EQUAL) {
            self.start_node_at(checkpoint, SyntaxKind::EnumVariantSequence);

            self.trivia(lex);
            self.expression(lex);

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn enum_definition() {
    for source in [
        // Unit
        "enum A {}",
        "enum A { B }",
        "enum A { B, }",
        "enum A { B, C }",
        // Sequence
        "enum A { B = 42 }",
        "enum A { B = 42, }",
        "enum A { B = 42, C = 42 }",
        // Named
        "enum A { B { } }",
        "enum A { B { C: i32 } }",
        "enum A { B { C: i32, } }",
        "enum A { B { C: i32, D: i32 } }",
        // Unnamed
        "enum A { B() }",
        "enum A { B(C) }",
        "enum A { B(C,) }",
        "enum A { B(C, D) }",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.enum_definition(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

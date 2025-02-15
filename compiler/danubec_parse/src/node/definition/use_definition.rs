use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn use_definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::USE) {
            self.start_node_at(checkpoint, SyntaxKind::UseDefinition);

            self.trivia(lex);
            self.use_tree(lex);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::SEMICOLON);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn use_tree(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        let mut matched = false;

        if self.path(lex) {
            self.trivia(lex);
            matched = true;
        }

        loop {
            if expect!(self, lex, token -> COLON__COLON, SyntaxKind::COLON, SyntaxKind::COLON,) {
                self.trivia(lex);
                matched = true;
            }

            if self.use_tree_kind(lex) {
                self.trivia(lex);
                matched = true;
            } else {
                break;
            }
        }

        if matched {
            self.start_node_at(checkpoint, SyntaxKind::UseTree);
            self.finish_node();
        }

        matched
    }

    fn use_tree_kind(&mut self, lex: &mut Lex) -> bool {
        self.use_tree_barrel(lex) || self.use_tree_ident(lex) || self.use_tree_nested(lex)
    }

    fn use_tree_barrel(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::ASTERISK) {
            self.start_node_at(checkpoint, SyntaxKind::UseTreeBarrel);
            self.finish_node();

            true
        } else {
            false
        }
    }

    fn use_tree_ident(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        let mut matched = false;

        if self.identifier(lex) {
            self.trivia(lex);
            matched = true;
        }

        if self.use_tree_ident_prefix(lex) {
            matched = true;
        }

        if matched {
            self.start_node_at(checkpoint, SyntaxKind::UseTreeIdent);
            self.finish_node();
        }

        matched
    }

    fn use_tree_nested(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LEFT_BRACE) {
            self.start_node_at(checkpoint, SyntaxKind::UseTreeNested);

            self.trivia(lex);
            while self.use_tree(lex) {
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

    fn use_tree_ident_prefix(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::AS) {
            self.start_node_at(checkpoint, SyntaxKind::UseTreeIdentPrefix);

            self.trivia(lex);
            self.identifier(lex);

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn use_definition() {
    for source in [
        "use *",
        "use a;",
        "use a::b;",
        "use a::{b};",
        "use a::{b,};",
        "use a::{b, c};",
        "use a::*;",
        "use a as b;",
        "use a::b as c;",
        "use {a};",
        "use {a,};",
        "use {a, b};",
        "use {{a}};",
        "use ::a;",
        "use ::a::b;",
        "use ::{a};",
        "use ::{a,};",
        "use ::{a, b};",
        "use ::{*};",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.use_definition(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

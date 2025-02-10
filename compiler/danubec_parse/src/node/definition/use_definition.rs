use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn use_definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::USE) {
            self.start_node_at(checkpoint, SyntaxKind::UseDefinition);

            self.trivia(lex);
            self.use_tree_kind(lex);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::SEMICOLON);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn use_tree_kind(&mut self, lex: &mut Lex) -> bool {
        self.use_tree_nested(lex) || self.use_tree_barrel(lex) || self.use_tree_ident(lex)
    }

    fn use_tree_nested(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LEFT_BRACE) {
            self.start_node_at(checkpoint, SyntaxKind::UseTreeNested);

            self.trivia(lex);
            while self.use_tree_kind(lex) {
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
        if self.identifier(lex) {
            self.start_node_at(checkpoint, SyntaxKind::UseTreeNested);

            self.trivia(lex);
            if expect!(
                self,
                lex,
                COLON__COLON,
                SyntaxKind::COLON,
                SyntaxKind::COLON,
            ) {
                self.trivia(lex);
                self.use_tree_kind(lex);
            } else if expect!(self, lex, SyntaxKind::AS) {
                self.trivia(lex);
                self.identifier(lex);
            }

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
        "use a;",
        "use a::b;",
        "use a::{b, c};",
        "use *;",
        "use a::*;",
        "use a as b;",
        "use a::{b as c, d as e};",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.use_definition(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}

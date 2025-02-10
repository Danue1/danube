mod const_definition;
mod enum_definition;
mod function_definition;
mod impl_definition;
mod module_definition;
mod static_definition;
mod struct_definition;
mod trait_definition;
mod type_definition;
mod use_definition;

use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn definitions(&mut self, lex: &mut Lex) {
        while !lex.is_empty() {
            self.trivia(lex);
            if self.definition(lex) {
                self.trivia(lex);
            } else {
                self.error(lex);
            }
        }
    }

    pub fn definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        let mut like_definition = false;
        if self.visibility(lex) {
            self.trivia(lex);
            like_definition = true;
        }

        if self.definition_kind(lex) {
            like_definition = true;
        }

        if like_definition {
            self.start_node_at(checkpoint, SyntaxKind::Definition);
            self.finish_node();
        }

        like_definition
    }

    pub fn definition_kind(&mut self, lex: &mut Lex) -> bool {
        self.function_definition(lex)
            || self.type_definition(lex)
            || self.struct_definition(lex)
            || self.enum_definition(lex)
            || self.trait_definition(lex)
            || self.impl_definition(lex)
            || self.const_definition(lex)
            || self.static_definition(lex)
            || self.use_definition(lex)
            || self.module_definition(lex)
    }
}

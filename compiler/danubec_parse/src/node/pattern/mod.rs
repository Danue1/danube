mod array_pattern;
mod literal_pattern;
mod named_pattern;
mod never_pattern;
mod or_pattern;
mod path_pattern;
mod placeholder_pattern;
mod rest_pattern;
mod tuple_pattern;
mod unnamed_pattern;

use super::expression::{LITERAL_FIRST, PATH_FIRST};
use crate::{pratt::Bp, tokens::Tokens};
use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

const PATTERN_FIRST: Tokens = tokens![
    // !
    EXCLAMATION,
    // _
    UNDERSCORE,
    // ..
    DOT,
    // (a, b)
    LEFT_PAREN,
    // [a, b]
    LEFT_BRACKET,
];

const LHS_PATTERN_FIRST: Tokens = LITERAL_FIRST.concat(PATH_FIRST).concat(PATTERN_FIRST);

impl crate::Context {
    pub fn pattern(&mut self, lex: &mut Lex) -> bool {
        self.pattern_kind(lex)
    }

    fn pattern_kind(&mut self, lex: &mut Lex) -> bool {
        self.pattern_kind_bp(lex, Bp(1))
    }

    fn pattern_kind_bp(&mut self, lex: &mut Lex, binding_power: Bp) -> bool {
        let checkpoint = self.checkpoint();
        if self.pattern_kind_lhs(lex) {
            self.start_node_at(checkpoint, SyntaxKind::Pattern);
            self.finish_node();

            loop {
                self.trivia(lex);
                let (left_binding_power, right_binding_power) =
                    self.infix_pattern_binding_power(lex);
                if left_binding_power < binding_power {
                    break;
                }

                if self.infix_pattern(lex, checkpoint, right_binding_power) {
                    self.start_node_at(checkpoint, SyntaxKind::Pattern);
                    self.finish_node();
                } else {
                    break;
                }
            }

            true
        } else {
            false
        }
    }

    fn pattern_kind_lhs(&mut self, lex: &mut Lex) -> bool {
        if lex.matches(|kind, _| LHS_PATTERN_FIRST.contains(kind)) {
            self.never_pattern(lex)
                || self.placeholder_pattern(lex)
                || self.rest_pattern(lex)
                || self.path_pattern(lex)
                || self.tuple_pattern(lex)
                || self.array_pattern(lex)
                || self.literal_pattern(lex)
        } else {
            false
        }
    }

    fn infix_pattern_binding_power(&mut self, lex: &mut Lex) -> (Bp, Bp) {
        macro_rules! match_operator {
            ($(
                ($kind1:pat, $kind2:pat) => $order:ident,
            )+) => {
                let mut lex = lex.clone();
                let kind1 = lex.next().map(|(kind, _)| kind);
                let kind2 = lex.next().map(|(kind, _)| kind);

                match (kind1, kind2) {
                    $(
                        ($kind1, $kind2) => Bp::$order,
                    )+
                    _ => Bp::P0,
                }
            };
        }

        match_operator! {
            (Some(SyntaxKind::PIPE), _) => P1,
        }
    }

    fn infix_pattern(&mut self, lex: &mut Lex, checkpoint: Checkpoint, binding_power: Bp) -> bool {
        self.or_pattern(lex, checkpoint, binding_power)
    }
}

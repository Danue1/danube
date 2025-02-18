mod array_expression;
mod assignment_expression;
mod binary_expression;
mod block_expression;
mod break_expression;
mod continue_expression;
mod for_expression;
mod if_expression;
mod let_expression;
mod literal_expression;
mod loop_expression;
mod match_expression;
mod path_expression;
mod return_expression;
mod unary_expression;
mod while_expression;

use crate::{tokens::Tokens, Bp};
use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

const LITERAL_FIRST: Tokens = tokens![
    // true
    TRUE,
    // false
    FALSE,
    // "abc"
    DOUBLE_QUOTE,
    // 'a'
    SINGLE_QUOTE,
    // 123
    NUMERIC,
];

const PATH_FIRST: Tokens = tokens![
    // abc::def::ghi
    ALPHABETIC,
    // super::abc
    SUPER,
    // crate::abc
    CRATE,
    // ::abc
    COLON,
    // <abc>::def
    LEFT_CHEVRON,
];

const UNARY_FIRST: Tokens = tokens![
    // -1
    HYPHEN,
    // !true
    EXCLAMATION,
    // ~1
    TILDE,
];

const EXPR_FIRST: Tokens = tokens![
    // let a = 1;
    LET,
    // if true { 1 } else { 2 }
    IF,
    // match 1 { 1 => 2 }
    MATCH,
    // loop { 1 }
    LOOP,
    // while true { 1 }
    WHILE,
    // for a in b { 1 }
    FOR,
    // return 1
    RETURN,
    // break
    BREAK,
    // continue
    CONTINUE,
    // [1, 2, 3]
    LEFT_BRACKET,
    // (1, 2, 3)
    LEFT_PAREN,
    // { 1, 2, 3 }
    LEFT_BRACE,
];

const LHS_FIRST: Tokens = LITERAL_FIRST
    .concat(PATH_FIRST)
    .concat(UNARY_FIRST)
    .concat(EXPR_FIRST);

impl crate::Context {
    pub fn expression_top(&mut self, lex: &mut Lex) -> bool {
        self.let_expression(lex)
            || self.if_expression(lex)
            || self.match_expression(lex)
            || self.loop_expression(lex)
            || self.while_expression(lex)
            || self.for_expression(lex)
            || self.return_expression(lex)
            || self.break_expression(lex)
            || self.continue_expression(lex)
            || self.expression(lex)
    }

    pub fn expression(&mut self, lex: &mut Lex) -> bool {
        self.expression_bp(lex, Bp(1))
    }

    fn expression_bp(&mut self, lex: &mut Lex, binding_power: Bp) -> bool {
        let checkpoint = self.checkpoint();
        if self.expression_lhs(lex) {
            self.start_node_at(checkpoint, SyntaxKind::Expression);
            self.finish_node();

            loop {
                self.trivia(lex);
                let (left_binding_power, right_binding_power) = self.infix_binding_power(lex);
                if left_binding_power < binding_power {
                    break;
                }

                if self.infix_expression(lex, checkpoint, right_binding_power) {
                    self.start_node_at(checkpoint, SyntaxKind::Expression);
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

    fn expression_lhs(&mut self, lex: &mut Lex) -> bool {
        if lex.matches(|kind, _| LHS_FIRST.contains(kind)) {
            self.literal_expression(lex)
                || self.unary_expression(lex)
                || self.block_expression(lex)
                || self.array_expression(lex)
                || self.path_expression(lex)
        } else {
            false
        }
    }

    fn infix_binding_power(&self, lex: &mut Lex) -> (Bp, Bp) {
        macro_rules! match_operator {
            ($(
                ($kind1:pat, $kind2:pat, $kind3:pat, $kind4:pat) => $order:ident,
            )+) => {
                let mut lex = lex.clone();
                let kind1 = lex.next().map(|(kind, _)| kind);
                let kind2 = lex.next().map(|(kind, _)| kind);
                let kind3 = lex.next().map(|(kind, _)| kind);
                let kind4 = lex.next().map(|(kind, _)| kind);

                match (kind1, kind2, kind3, kind4) {
                    $(
                        ($kind1, $kind2, $kind3, $kind4) => Bp::$order,
                    )+
                    _ => Bp::P0,
                }
            };
        }

        match_operator! {
            // Assignment operators
            (Some(SyntaxKind::PLUS), Some(SyntaxKind::EQUAL), _, _) => P1,
            (Some(SyntaxKind::PLUS), Some(SyntaxKind::PIPE), Some(SyntaxKind::EQUAL), _) => P1,
            (Some(SyntaxKind::PLUS), Some(SyntaxKind::PERCENT), Some(SyntaxKind::EQUAL), _) => P1,
            (Some(SyntaxKind::HYPHEN), Some(SyntaxKind::EQUAL), _, _) => P1,
            (Some(SyntaxKind::HYPHEN), Some(SyntaxKind::PIPE), Some(SyntaxKind::EQUAL), _) => P1,
            (Some(SyntaxKind::HYPHEN), Some(SyntaxKind::PERCENT), Some(SyntaxKind::EQUAL), _) => P1,
            (Some(SyntaxKind::ASTERISK), Some(SyntaxKind::EQUAL), _, _) => P1,
            (Some(SyntaxKind::ASTERISK), Some(SyntaxKind::PIPE), Some(SyntaxKind::EQUAL), _) => P1,
            (Some(SyntaxKind::ASTERISK), Some(SyntaxKind::PERCENT), Some(SyntaxKind::EQUAL), _) => P1,
            (Some(SyntaxKind::ASTERISK), Some(SyntaxKind::ASTERISK), Some(SyntaxKind::EQUAL), _) => P1,
            (Some(SyntaxKind::ASTERISK), Some(SyntaxKind::ASTERISK), Some(SyntaxKind::PIPE), Some(SyntaxKind::EQUAL)) => P1,
            (Some(SyntaxKind::ASTERISK), Some(SyntaxKind::ASTERISK), Some(SyntaxKind::PERCENT), Some(SyntaxKind::EQUAL)) => P1,
            (Some(SyntaxKind::SLASH), Some(SyntaxKind::EQUAL), _, _) => P1,
            (Some(SyntaxKind::PERCENT), Some(SyntaxKind::EQUAL), _, _) => P1,
            (Some(SyntaxKind::CARET), Some(SyntaxKind::EQUAL), _, _) => P1,
            (Some(SyntaxKind::AMPERSAND), Some(SyntaxKind::EQUAL), _, _) => P1,
            (Some(SyntaxKind::AMPERSAND), Some(SyntaxKind::AMPERSAND), Some(SyntaxKind::EQUAL), _) => P1,
            (Some(SyntaxKind::PIPE), Some(SyntaxKind::EQUAL), _, _) => P1,
            (Some(SyntaxKind::PIPE), Some(SyntaxKind::PIPE), Some(SyntaxKind::EQUAL), _) => P1,
            (Some(SyntaxKind::LEFT_CHEVRON), Some(SyntaxKind::LEFT_CHEVRON), Some(SyntaxKind::EQUAL), _) => P1,
            (Some(SyntaxKind::LEFT_CHEVRON), Some(SyntaxKind::LEFT_CHEVRON), Some(SyntaxKind::PIPE), Some(SyntaxKind::EQUAL)) => P1,
            (Some(SyntaxKind::RIGHT_CHEVRON), Some(SyntaxKind::RIGHT_CHEVRON), Some(SyntaxKind::EQUAL), _) => P1,
            (Some(SyntaxKind::RIGHT_CHEVRON), Some(SyntaxKind::RIGHT_CHEVRON), Some(SyntaxKind::RIGHT_CHEVRON), Some(SyntaxKind::EQUAL)) => P1,

            // Binary operators
            (Some(SyntaxKind::PIPE), Some(SyntaxKind::PIPE), _, _) => P2,

            (Some(SyntaxKind::AMPERSAND), Some(SyntaxKind::AMPERSAND), _, _) => P3,

            (Some(SyntaxKind::EQUAL), Some(SyntaxKind::EQUAL), _, _) => P4,
            (Some(SyntaxKind::EXCLAMATION), Some(SyntaxKind::EQUAL), _, _) => P4,
            (Some(SyntaxKind::LEFT_CHEVRON), Some(SyntaxKind::EQUAL), _, _) => P4,
            (Some(SyntaxKind::RIGHT_CHEVRON), Some(SyntaxKind::EQUAL), _, _) => P4,

            (Some(SyntaxKind::PIPE), _, _, _) => P5,

            (Some(SyntaxKind::CARET), _, _, _) => P6,

            (Some(SyntaxKind::AMPERSAND), _, _, _) => P7,

            (Some(SyntaxKind::LEFT_CHEVRON), Some(SyntaxKind::LEFT_CHEVRON), Some(SyntaxKind::PIPE), _) => P8,
            (Some(SyntaxKind::LEFT_CHEVRON), Some(SyntaxKind::LEFT_CHEVRON), _, _) => P8,
            (Some(SyntaxKind::RIGHT_CHEVRON), Some(SyntaxKind::RIGHT_CHEVRON), Some(SyntaxKind::RIGHT_CHEVRON), _) => P8,
            (Some(SyntaxKind::RIGHT_CHEVRON), Some(SyntaxKind::RIGHT_CHEVRON), _, _) => P8,

            (Some(SyntaxKind::PLUS), Some(SyntaxKind::PIPE), _, _) => P9,
            (Some(SyntaxKind::PLUS), Some(SyntaxKind::PERCENT), _, _) => P9,
            (Some(SyntaxKind::PLUS), _, _, _) => P9,
            (Some(SyntaxKind::HYPHEN), Some(SyntaxKind::PIPE), _, _) => P9,
            (Some(SyntaxKind::HYPHEN), Some(SyntaxKind::PERCENT), _, _) => P9,
            (Some(SyntaxKind::HYPHEN), _, _, _) => P9,

            (Some(SyntaxKind::ASTERISK), Some(SyntaxKind::PIPE), _, _) => P10,
            (Some(SyntaxKind::ASTERISK), Some(SyntaxKind::PERCENT), _, _) => P10,
            (Some(SyntaxKind::ASTERISK), Some(SyntaxKind::ASTERISK), Some(SyntaxKind::PIPE), _) => P10,
            (Some(SyntaxKind::ASTERISK), Some(SyntaxKind::ASTERISK), Some(SyntaxKind::PERCENT), _) => P10,
            (Some(SyntaxKind::ASTERISK), Some(SyntaxKind::ASTERISK), _, _) => P10,
            (Some(SyntaxKind::ASTERISK), _, _, _) => P10,
            (Some(SyntaxKind::SLASH), _, _, _) => P10,
            (Some(SyntaxKind::PERCENT), _, _, _) => P10,

            // Assignment operators
            (Some(SyntaxKind::EQUAL), _, _, _) => P1,
            (Some(SyntaxKind::LEFT_CHEVRON), _, _, _) => P4,
            (Some(SyntaxKind::RIGHT_CHEVRON), _, _, _) => P4,
        }
    }

    fn infix_expression(
        &mut self,
        lex: &mut Lex,
        checkpoint: Checkpoint,
        binding_power: Bp,
    ) -> bool {
        self.assignment_expression(lex, checkpoint, binding_power)
            || self.binary_expression(lex, checkpoint, binding_power)
    }
}

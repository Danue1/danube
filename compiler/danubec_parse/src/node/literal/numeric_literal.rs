use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn numeric_literal(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.binary_numeric_literal(lex)
            || self.octal_numeric_literal(lex)
            || self.hex_numeric_literal(lex)
            || self.decimal_numeric_literal(lex)
        {
            self.start_node_at(checkpoint, SyntaxKind::NumericLiteral);
            self.finish_node();

            true
        } else {
            false
        }
    }

    fn binary_numeric_literal(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, NUMERIC_LITERAL_PREFIX -> "0", "b",) {
            self.start_node_at(checkpoint, SyntaxKind::BinaryNumericLiteral);
            self.numeric_fragment(lex);
            self.finish_node();

            true
        } else {
            false
        }
    }

    fn octal_numeric_literal(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, NUMERIC_LITERAL_PREFIX -> "0", "o",) {
            self.start_node_at(checkpoint, SyntaxKind::OctalNumericLiteral);
            self.numeric_fragment(lex);
            self.finish_node();

            true
        } else {
            false
        }
    }

    fn hex_numeric_literal(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!( self, lex, NUMERIC_LITERAL_PREFIX -> "0", "x",) {
            self.start_node_at(checkpoint, SyntaxKind::HexNumericLiteral);
            self.numeric_fragment(lex);
            self.finish_node();

            true
        } else {
            false
        }
    }

    fn decimal_numeric_literal(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.integer_part(lex) {
            self.start_node_at(checkpoint, SyntaxKind::DecimalNumericLiteral);
            self.fractional_part(lex);
            self.exponent_part(lex);
            self.finish_node();

            true
        } else {
            false
        }
    }

    fn integer_part(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.numeric_fragment(lex) {
            self.start_node_at(checkpoint, SyntaxKind::IntegerPart);
            self.finish_node();

            true
        } else {
            false
        }
    }

    fn fractional_part(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::DOT) {
            self.start_node_at(checkpoint, SyntaxKind::FractionPart);
            self.numeric_fragment(lex);
            self.finish_node();

            true
        } else {
            false
        }
    }

    fn exponent_part(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, E -> "e" | "E",) {
            self.start_node_at(checkpoint, SyntaxKind::ExponentPart);
            self.exponent_part_sign(lex);
            self.numeric_fragment(lex);
            self.finish_node();

            true
        } else {
            false
        }
    }

    fn exponent_part_sign(&mut self, lex: &mut Lex) -> bool {
        expect!(
            self,
            lex,
            ExponentPartSign,
            SyntaxKind::PLUS | SyntaxKind::HYPHEN,
        )
    }

    fn numeric_fragment(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::NUMERIC) {
            self.start_node_at(checkpoint, SyntaxKind::NumericFragment);

            while expect!(self, lex, SyntaxKind::UNDERSCORE) {
                expect!(self, lex, SyntaxKind::NUMERIC);
            }

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn numeric_literal() {
    for source in [
        "0b1010",
        "0b1010_1010",
        "0o123",
        "0o123_456",
        "0x123",
        "0x123_456",
        "123",
        "123_456",
        "123.456",
        "123.456_789",
        "123.456e789",
        "123.456E789",
        "123.456e+789",
        "123.456E+789",
        "123.456e-789",
        "123.456E-789",
        "123e+456",
        "123E+456",
        "123e+456_789",
        "123E+456_789",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.numeric_literal(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}

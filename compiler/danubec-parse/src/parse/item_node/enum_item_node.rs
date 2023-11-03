use crate::context::State;
use danubec_syntax_kind::SyntaxKind;

impl crate::Context {
    pub fn enum_item_node(&mut self) -> State {
        guard!(self, ENUM_KEYWORD, ENUM_ITEM_NODE);
        self.skip_whitespace();
        self.ident_node();
        self.skip_whitespace();
        expect!(self, SyntaxKind::LEFT_BRACE);
        self.skip_whitespace();

        while !self.is_eof() {
            if expect!(self, SyntaxKind::RIGHT_BRACE) {
                break;
            }

            self.enum_variant_kind_node();
            self.skip_whitespace();
            if expect!(self, SyntaxKind::COMMA) {
                self.skip_whitespace();
            }
        }

        self.finish_node();
        State::Stop
    }

    pub fn enum_variant_kind_node(&mut self) -> State {
        guard!(self, IDENT_KEYWORD, ENUM_VARIANT_KIND_NODE, ident_node);
        self.skip_whitespace();
        one_of!(self.named_fields_node(), self.unnamed_fields_node());
        self.finish_node();
        State::Stop
    }
}

#[cfg(test)]
mod tests {
    use danubec_syntax_kind::SyntaxKind;

    #[test]
    fn test_zero_item_node() {
        assert_node!(
            vec![
                (SyntaxKind::ENUM_KEYWORD, "enum".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Foo".to_string()),
                (SyntaxKind::LEFT_BRACE, "{".to_string()),
                (SyntaxKind::RIGHT_BRACE, "}".to_string()),
            ],
            r#"AST_NODE@0..10
  ENUM_ITEM_NODE@0..10
    ENUM_KEYWORD@0..4 "enum"
    WHITESPACE@4..5 " "
    IDENT_NODE@5..8
      IDENT_KEYWORD@5..8 "Foo"
    LEFT_BRACE@8..9 "{"
    RIGHT_BRACE@9..10 "}"
"#,
        );
    }

    #[test]
    fn test_one_variant_with_no_fields() {
        assert_node!(
            vec![
                (SyntaxKind::ENUM_KEYWORD, "enum".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Foo".to_string()),
                (SyntaxKind::LEFT_BRACE, "{".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Bar".to_string()),
                (SyntaxKind::RIGHT_BRACE, "}".to_string()),
            ],
            r#"AST_NODE@0..13
  ENUM_ITEM_NODE@0..13
    ENUM_KEYWORD@0..4 "enum"
    WHITESPACE@4..5 " "
    IDENT_NODE@5..8
      IDENT_KEYWORD@5..8 "Foo"
    LEFT_BRACE@8..9 "{"
    ENUM_VARIANT_KIND_NODE@9..12
      IDENT_NODE@9..12
        IDENT_KEYWORD@9..12 "Bar"
    RIGHT_BRACE@12..13 "}"
"#,
        );
    }

    #[test]
    fn test_one_variant_with_one_unnamed_field() {
        assert_node!(
            vec![
                (SyntaxKind::ENUM_KEYWORD, "enum".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Foo".to_string()),
                (SyntaxKind::LEFT_BRACE, "{".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Bar".to_string()),
                (SyntaxKind::LEFT_PAREN, "(".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Baz".to_string()),
                (SyntaxKind::RIGHT_PAREN, ")".to_string()),
                (SyntaxKind::RIGHT_BRACE, "}".to_string()),
            ],
            r#"AST_NODE@0..18
  ENUM_ITEM_NODE@0..18
    ENUM_KEYWORD@0..4 "enum"
    WHITESPACE@4..5 " "
    IDENT_NODE@5..8
      IDENT_KEYWORD@5..8 "Foo"
    LEFT_BRACE@8..9 "{"
    ENUM_VARIANT_KIND_NODE@9..17
      IDENT_NODE@9..12
        IDENT_KEYWORD@9..12 "Bar"
      UNNAMED_FIELDS_NODE@12..17
        LEFT_PAREN@12..13 "("
        UNNAMED_FIELD_NODE@13..16
          TYPE_NODE@13..16
            PATH_TYPE_KIND_NODE@13..16
              PATH_NODE@13..16
                IDENT_NODE@13..16
                  IDENT_KEYWORD@13..16 "Baz"
        RIGHT_PAREN@16..17 ")"
    RIGHT_BRACE@17..18 "}"
"#,
        );
    }

    #[test]
    fn test_one_variant_with_one_named_field() {
        assert_node!(
            vec![
                (SyntaxKind::ENUM_KEYWORD, "enum".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Foo".to_string()),
                (SyntaxKind::LEFT_BRACE, "{".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Bar".to_string()),
                (SyntaxKind::LEFT_PAREN, "(".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "baz".to_string()),
                (SyntaxKind::COLON, ":".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Baz".to_string()),
                (SyntaxKind::RIGHT_PAREN, ")".to_string()),
                (SyntaxKind::RIGHT_BRACE, "}".to_string()),
            ],
            r#"AST_NODE@0..22
  ENUM_ITEM_NODE@0..22
    ENUM_KEYWORD@0..4 "enum"
    WHITESPACE@4..5 " "
    IDENT_NODE@5..8
      IDENT_KEYWORD@5..8 "Foo"
    LEFT_BRACE@8..9 "{"
    ENUM_VARIANT_KIND_NODE@9..21
      IDENT_NODE@9..12
        IDENT_KEYWORD@9..12 "Bar"
      UNNAMED_FIELDS_NODE@12..21
        LEFT_PAREN@12..13 "("
        UNNAMED_FIELD_NODE@13..20
          TYPE_NODE@13..20
            PATH_TYPE_KIND_NODE@13..20
              PATH_NODE@13..20
                IDENT_NODE@13..16
                  IDENT_KEYWORD@13..16 "baz"
                COLON@16..17 ":"
                ERROR@17..20
                  IDENT_KEYWORD@17..20 "Baz"
        RIGHT_PAREN@20..21 ")"
    RIGHT_BRACE@21..22 "}"
"#,
        );
    }
}

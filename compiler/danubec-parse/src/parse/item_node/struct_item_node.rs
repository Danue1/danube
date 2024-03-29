use crate::State;
use danubec_syntax_kind::SyntaxKind;

impl crate::Context {
    pub fn struct_item_node(&mut self) -> State {
        guard!(self, STRUCT_KEYWORD, STRUCT_ITEM_NODE);
        self.skip_whitespace();
        if self.ident_node() == State::Stop {
            self.skip_whitespace();
        }
        if self.named_fields_node() == State::Continue {
            if self.unnamed_fields_node() == State::Continue {
                self.skip_whitespace();
            }
            expect!(self, SyntaxKind::SEMICOLON);
        }

        self.finish_node();
        State::Stop
    }

    pub fn named_fields_node(&mut self) -> State {
        guard!(self, LEFT_BRACE, NAMED_FIELDS_NODE);
        self.skip_whitespace();

        while !self.is_eof() {
            if expect!(self, SyntaxKind::RIGHT_BRACE) {
                break;
            }

            if self.named_field_node() == State::Continue {
                self.unexpected_token();
            }
            self.skip_whitespace();
        }

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
                (SyntaxKind::STRUCT_KEYWORD, "struct".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Foo".to_string()),
                (SyntaxKind::SEMICOLON, ";".to_string()),
            ],
            r#"AST_NODE@0..11
  STRUCT_ITEM_NODE@0..11
    STRUCT_KEYWORD@0..6 "struct"
    WHITESPACE@6..7 " "
    IDENT_NODE@7..10
      IDENT_KEYWORD@7..10 "Foo"
    SEMICOLON@10..11 ";"
"#,
        );
    }

    #[test]
    fn test_named_struct_with_no_field() {
        assert_node!(
            vec![
                (SyntaxKind::STRUCT_KEYWORD, "struct".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Foo".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::LEFT_BRACE, "{".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::RIGHT_BRACE, "}".to_string()),
            ],
            r#"AST_NODE@0..14
  STRUCT_ITEM_NODE@0..14
    STRUCT_KEYWORD@0..6 "struct"
    WHITESPACE@6..7 " "
    IDENT_NODE@7..10
      IDENT_KEYWORD@7..10 "Foo"
    WHITESPACE@10..11 " "
    NAMED_FIELDS_NODE@11..14
      LEFT_BRACE@11..12 "{"
      WHITESPACE@12..13 " "
      RIGHT_BRACE@13..14 "}"
"#,
        );
    }

    #[test]
    fn test_named_struct_with_one_field_with_no_trailing_comma() {
        assert_node!(
            vec![
                (SyntaxKind::STRUCT_KEYWORD, "struct".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Foo".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::LEFT_BRACE, "{".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "bar".to_string()),
                (SyntaxKind::COLON, ":".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "i32".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::RIGHT_BRACE, "}".to_string()),
            ],
            r#"AST_NODE@0..23
  STRUCT_ITEM_NODE@0..23
    STRUCT_KEYWORD@0..6 "struct"
    WHITESPACE@6..7 " "
    IDENT_NODE@7..10
      IDENT_KEYWORD@7..10 "Foo"
    WHITESPACE@10..11 " "
    NAMED_FIELDS_NODE@11..23
      LEFT_BRACE@11..12 "{"
      WHITESPACE@12..13 " "
      NAMED_FIELD_NODE@13..22
        IDENT_NODE@13..16
          IDENT_KEYWORD@13..16 "bar"
        COLON@16..17 ":"
        WHITESPACE@17..18 " "
        TYPE_NODE@18..21
          PATH_TYPE_KIND_NODE@18..21
            PATH_NODE@18..21
              IDENT_NODE@18..21
                IDENT_KEYWORD@18..21 "i32"
        WHITESPACE@21..22 " "
      RIGHT_BRACE@22..23 "}"
"#,
        );
    }

    #[test]
    fn test_named_struct_with_one_field_with_trailing_comma() {
        assert_node!(
            vec![
                (SyntaxKind::STRUCT_KEYWORD, "struct".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Foo".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::LEFT_BRACE, "{".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "bar".to_string()),
                (SyntaxKind::COLON, ":".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "i32".to_string()),
                (SyntaxKind::COMMA, ",".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::RIGHT_BRACE, "}".to_string()),
            ],
            r#"AST_NODE@0..24
  STRUCT_ITEM_NODE@0..24
    STRUCT_KEYWORD@0..6 "struct"
    WHITESPACE@6..7 " "
    IDENT_NODE@7..10
      IDENT_KEYWORD@7..10 "Foo"
    WHITESPACE@10..11 " "
    NAMED_FIELDS_NODE@11..24
      LEFT_BRACE@11..12 "{"
      WHITESPACE@12..13 " "
      NAMED_FIELD_NODE@13..22
        IDENT_NODE@13..16
          IDENT_KEYWORD@13..16 "bar"
        COLON@16..17 ":"
        WHITESPACE@17..18 " "
        TYPE_NODE@18..21
          PATH_TYPE_KIND_NODE@18..21
            PATH_NODE@18..21
              IDENT_NODE@18..21
                IDENT_KEYWORD@18..21 "i32"
        COMMA@21..22 ","
      WHITESPACE@22..23 " "
      RIGHT_BRACE@23..24 "}"
"#,
        );
    }

    #[test]
    fn test_unnamed_struct_with_no_field() {
        assert_node!(
            vec![
                (SyntaxKind::STRUCT_KEYWORD, "struct".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Foo".to_string()),
                (SyntaxKind::LEFT_PAREN, "(".to_string()),
                (SyntaxKind::RIGHT_PAREN, ")".to_string()),
                (SyntaxKind::SEMICOLON, ";".to_string()),
            ],
            r#"AST_NODE@0..13
  STRUCT_ITEM_NODE@0..13
    STRUCT_KEYWORD@0..6 "struct"
    WHITESPACE@6..7 " "
    IDENT_NODE@7..10
      IDENT_KEYWORD@7..10 "Foo"
    UNNAMED_FIELDS_NODE@10..12
      LEFT_PAREN@10..11 "("
      RIGHT_PAREN@11..12 ")"
    SEMICOLON@12..13 ";"
"#,
        );
    }

    #[test]
    fn test_unnamed_struct_with_one_field_with_no_trailing_comma() {
        assert_node!(
            vec![
                (SyntaxKind::STRUCT_KEYWORD, "struct".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Foo".to_string()),
                (SyntaxKind::LEFT_PAREN, "(".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "i32".to_string()),
                (SyntaxKind::RIGHT_PAREN, ")".to_string()),
                (SyntaxKind::SEMICOLON, ";".to_string()),
            ],
            r#"AST_NODE@0..16
  STRUCT_ITEM_NODE@0..16
    STRUCT_KEYWORD@0..6 "struct"
    WHITESPACE@6..7 " "
    IDENT_NODE@7..10
      IDENT_KEYWORD@7..10 "Foo"
    UNNAMED_FIELDS_NODE@10..15
      LEFT_PAREN@10..11 "("
      UNNAMED_FIELD_NODE@11..14
        TYPE_NODE@11..14
          PATH_TYPE_KIND_NODE@11..14
            PATH_NODE@11..14
              IDENT_NODE@11..14
                IDENT_KEYWORD@11..14 "i32"
      RIGHT_PAREN@14..15 ")"
    SEMICOLON@15..16 ";"
"#,
        );
    }

    #[test]
    fn test_unnamed_struct_with_one_field_with_trailing_comma() {
        assert_node!(
            vec![
                (SyntaxKind::STRUCT_KEYWORD, "struct".to_string()),
                (SyntaxKind::WHITESPACE, " ".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "Foo".to_string()),
                (SyntaxKind::LEFT_PAREN, "(".to_string()),
                (SyntaxKind::IDENT_KEYWORD, "i32".to_string()),
                (SyntaxKind::COMMA, ",".to_string()),
                (SyntaxKind::RIGHT_PAREN, ")".to_string()),
                (SyntaxKind::SEMICOLON, ";".to_string()),
            ],
            r#"AST_NODE@0..17
  STRUCT_ITEM_NODE@0..17
    STRUCT_KEYWORD@0..6 "struct"
    WHITESPACE@6..7 " "
    IDENT_NODE@7..10
      IDENT_KEYWORD@7..10 "Foo"
    UNNAMED_FIELDS_NODE@10..16
      LEFT_PAREN@10..11 "("
      UNNAMED_FIELD_NODE@11..14
        TYPE_NODE@11..14
          PATH_TYPE_KIND_NODE@11..14
            PATH_NODE@11..14
              IDENT_NODE@11..14
                IDENT_KEYWORD@11..14 "i32"
      COMMA@14..15 ","
      RIGHT_PAREN@15..16 ")"
    SEMICOLON@16..17 ";"
"#,
        );
    }
}

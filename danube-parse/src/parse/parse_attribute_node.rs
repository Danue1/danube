use super::*;
use std::collections::HashMap;
use std::iter::FromIterator;

pub(super) fn parse_attribute_node(t: Tokens) -> ParseResult<AttributeNode> {
    map(
        tuple((
            parse_symbol(Symbol::Hashtag),
            parse_symbol(Symbol::LeftBracket),
            parse_path_node,
            opt(parse_attribute_argument_list),
            parse_symbol(Symbol::RightBracket),
        )),
        |(_, _, path, args, _)| AttributeNode {
            path,
            args: args.unwrap_or_else(Default::default),
        },
    )(t)
}

fn parse_attribute_argument_list(t: Tokens) -> ParseResult<HashMap<String, Option<LiteralKind>>> {
    map(
        tuple((
            parse_symbol(Symbol::LeftParens),
            separated_nonempty_list(parse_symbol(Symbol::Comma), parse_attribute_argument_node),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::RightParens),
        )),
        |(_, args, _, _)| HashMap::from_iter(args),
    )(t)
}

fn parse_attribute_argument_node(t: Tokens) -> ParseResult<(String, Option<LiteralKind>)> {
    tuple((
        parse_identifier,
        opt(preceded(parse_symbol(Symbol::Assign), parse_literal_kind)),
    ))(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> AttributeNode {
        let token_list = lex(s).unwrap();
        match parse_attribute_node(Tokens::new(&token_list)) {
            Ok((_, node)) => node,
            Err(error) => {
                dbg!(error);
                panic!();
            }
        }
    }

    #[test]
    fn base() {
        let source = r#"#[foo]"#;
        assert_eq!(
            parse(source),
            AttributeNode {
                path: PathNode {
                    ident_list: vec![IdentNode {
                        raw: "foo".to_owned(),
                    }],
                },
                args: Default::default(),
            }
        );
    }

    #[test]
    fn a_argument() {
        let source = r#"#[foo(bar)]"#;
        assert_eq!(
            parse(source),
            AttributeNode {
                path: PathNode {
                    ident_list: vec![IdentNode {
                        raw: "foo".to_owned(),
                    }],
                },
                args: vec![("bar".to_owned(), None)].into_iter().collect(),
            }
        );
    }

    #[test]
    fn a_argument_value() {
        let source = r#"#[foo(bar="baz")]"#;
        assert_eq!(
            parse(source),
            AttributeNode {
                path: PathNode {
                    ident_list: vec![IdentNode {
                        raw: "foo".to_owned(),
                    }],
                },
                args: vec![(
                    "bar".to_owned(),
                    Some(LiteralKind::String("baz".to_owned()))
                )]
                .into_iter()
                .collect(),
            }
        );
    }

    #[test]
    fn two_arguments() {
        let source = r#"#[foo(bar, baz)]"#;
        assert_eq!(
            parse(source),
            AttributeNode {
                path: PathNode {
                    ident_list: vec![IdentNode {
                        raw: "foo".to_owned(),
                    }],
                },
                args: vec![("bar".to_owned(), None), ("baz".to_owned(), None)]
                    .into_iter()
                    .collect(),
            }
        );
    }
}

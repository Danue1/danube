use super::*;
use std::collections::HashMap;
use std::iter::FromIterator;

pub fn parse_module(t: Tokens) -> ParseResult<Module> {
    map(
        all_consuming(tuple((many0(parse_feature), parse_module_node))),
        |(attribute_list, node)| Attributed {
            attribute_list,
            node,
        },
    )(t)
}

fn parse_feature(t: Tokens) -> ParseResult<AttributeNode> {
    map(
        tuple((
            parse_symbol(Symbol::Hashtag),
            parse_symbol(Symbol::Not),
            parse_symbol(Symbol::LeftBracket),
            parse_path_node,
            opt(parse_feature_argument_list),
            parse_symbol(Symbol::RightBracket),
        )),
        |(_, _, _, path, args, _)| AttributeNode {
            path,
            args: args.unwrap_or_else(Default::default),
        },
    )(t)
}

fn parse_feature_argument_list(t: Tokens) -> ParseResult<HashMap<String, Option<LiteralKind>>> {
    map(
        tuple((
            parse_symbol(Symbol::LeftParens),
            separated_list(parse_symbol(Symbol::Comma), parse_feature_argument_node),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::RightParens),
        )),
        |(_, args, _, _)| HashMap::from_iter(args),
    )(t)
}

fn parse_feature_argument_node(t: Tokens) -> ParseResult<(String, Option<LiteralKind>)> {
    tuple((
        parse_identifier,
        opt(preceded(parse_symbol(Symbol::Assign), parse_literal_kind)),
    ))(t)
}

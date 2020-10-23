use super::*;

pub(super) fn parse_path_node(t: Tokens) -> ParseResult<PathNode> {
    map(
        separated_nonempty_list(
            parse_symbol(Symbol::DoubleColon),
            alt((
                map(parse_keyword(Keyword::TypeSelf), |_| IdentNode {
                    raw: "Self".to_owned(),
                }),
                parse_ident_node,
            )),
        ),
        |ident_list| PathNode { ident_list },
    )(t)
}

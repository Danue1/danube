use super::*;

pub(super) fn parse_return_node(t: Tokens) -> ParseResult<ReturnNode> {
    map(
        tuple((
            parse_keyword(Keyword::Return),
            opt(parse_expression_kind),
            opt(parse_symbol(Symbol::Semicolon)),
        )),
        |(_, expression, _)| ReturnNode {
            value: expression.map(Box::new),
        },
    )(t)
}

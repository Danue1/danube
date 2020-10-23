use super::*;

pub(super) fn parse_let_node(t: Tokens) -> ParseResult<LetNode> {
    map(
        tuple((
            parse_keyword(Keyword::Let),
            parse_immutablity_kind,
            parse_pattern_kind,
            opt(map(
                tuple((parse_symbol(Symbol::Colon), parse_type_kind)),
                |(_, ty)| ty,
            )),
            opt(map(
                tuple((parse_symbol(Symbol::Assign), parse_expression_kind)),
                |(_, expression)| expression,
            )),
            parse_symbol(Symbol::Semicolon),
        )),
        |(_, immutablity, pattern, ty, value, _)| LetNode {
            immutablity,
            pattern,
            ty,
            value,
        },
    )(t)
}

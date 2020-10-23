use super::*;

pub(super) fn parse_type_array_node(t: Tokens) -> ParseResult<TypeArrayNode> {
    map(
        tuple((
            parse_symbol(Symbol::LeftBracket),
            parse_type_kind,
            parse_symbol(Symbol::Semicolon),
            parse_int,
            parse_symbol(Symbol::RightBracket),
        )),
        |(_, ty, _, size, _)| TypeArrayNode {
            ty: Box::new(ty),
            size: size as usize,
        },
    )(t)
}

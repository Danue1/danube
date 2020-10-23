use super::*;

pub(super) fn parse_closure_argument_node(t: Tokens) -> ParseResult<ClosureArgumentNode> {
    map(
        tuple((
            parse_ident_node,
            opt(preceded(parse_symbol(Symbol::Colon), parse_type_kind)),
        )),
        |(ident, ty)| ClosureArgumentNode { ident, ty },
    )(t)
}

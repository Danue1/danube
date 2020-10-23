use super::*;

pub(super) fn parse_function_argument_node(t: Tokens) -> ParseResult<FunctionArgumentNode> {
    map(
        tuple((
            parse_immutablity_kind,
            parse_ident_node,
            opt(preceded(parse_symbol(Symbol::Colon), parse_type_kind)),
        )),
        |(immutablity, ident, ty)| FunctionArgumentNode {
            immutablity,
            ident,
            ty,
        },
    )(t)
}

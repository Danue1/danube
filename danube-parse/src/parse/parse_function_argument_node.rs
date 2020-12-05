use super::*;

pub(super) fn parse_function_argument_node(t: Tokens) -> ParseResult<FunctionArgumentNode> {
    map(
        tuple((
            parse_immutablity_kind,
            parse_pattern_kind,
            opt(preceded(parse_symbol(Symbol::Colon), parse_type_kind)),
        )),
        |(immutablity, pattern, ty)| FunctionArgumentNode {
            immutablity,
            pattern,
            ty,
        },
    )(t)
}

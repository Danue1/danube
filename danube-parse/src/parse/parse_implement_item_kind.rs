use super::*;

pub(super) fn parse_implement_item_kind(t: Tokens) -> ParseResult<ImplementItemKind> {
    alt((
        map(
            parse_attributed(parse_implement_output_type_node),
            ImplementItemKind::OutputType,
        ),
        map(
            parse_attributed(parse_constant_node),
            ImplementItemKind::Constant,
        ),
        map(
            parse_attributed(parse_function_node),
            ImplementItemKind::Function,
        ),
    ))(t)
}

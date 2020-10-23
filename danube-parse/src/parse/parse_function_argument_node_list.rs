use super::*;

pub(super) fn parse_function_argument_node_list(
    t: Tokens,
) -> ParseResult<(Option<ImmutablityKind>, Vec<FunctionArgumentNode>)> {
    map(
        tuple((
            parse_symbol(Symbol::LeftParens),
            alt((
                parse_function_argument_node_list_0,
                parse_function_argument_node_list_1,
                parse_function_argument_node_list_2,
            )),
            parse_symbol(Symbol::RightParens),
        )),
        |(_, returns, _)| returns,
    )(t)
}

fn parse_function_argument_node_list_0(
    t: Tokens,
) -> ParseResult<(Option<ImmutablityKind>, Vec<FunctionArgumentNode>)> {
    map(
        tuple((
            terminated(parse_immutablity_kind, parse_keyword(Keyword::VariableSelf)),
            parse_symbol(Symbol::Comma),
            opt(terminated(
                separated_nonempty_list(parse_symbol(Symbol::Comma), parse_function_argument_node),
                parse_symbol(Symbol::Comma),
            )),
        )),
        |(self_type, _, argument_list)| {
            (
                Some(self_type),
                argument_list.unwrap_or_else(Default::default),
            )
        },
    )(t)
}

fn parse_function_argument_node_list_1(
    t: Tokens,
) -> ParseResult<(Option<ImmutablityKind>, Vec<FunctionArgumentNode>)> {
    map(
        preceded(
            parse_symbol(Symbol::LeftParens),
            terminated(parse_immutablity_kind, parse_keyword(Keyword::VariableSelf)),
        ),
        |self_type| (Some(self_type), Default::default()),
    )(t)
}

fn parse_function_argument_node_list_2(
    t: Tokens,
) -> ParseResult<(Option<ImmutablityKind>, Vec<FunctionArgumentNode>)> {
    map(
        tuple((
            separated_nonempty_list(parse_symbol(Symbol::Comma), parse_function_argument_node),
            opt(parse_symbol(Symbol::Comma)),
        )),
        |(argument_list, _)| (None, argument_list),
    )(t)
}

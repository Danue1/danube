use super::*;

pub(super) fn parse_closure_node(t: Tokens) -> ParseResult<ClosureNode> {
    let (t, argument_list) = alt((
        map(parse_symbol(Symbol::Or), |_| vec![]),
        map(
            tuple((
                parse_symbol(Symbol::BitOr),
                separated_list(parse_symbol(Symbol::Comma), parse_closure_argument_node),
                parse_symbol(Symbol::BitOr),
            )),
            |(_, argument_list, _)| argument_list,
        ),
    ))(t)?;
    let (t, return_type) = opt(preceded(parse_symbol(Symbol::ReturnArrow), parse_type_kind))(t)?;
    let (t, block) = if return_type.is_some() {
        parse_block_node(t)?
    } else {
        alt((
            map(parse_expression_kind, |expression| BlockNode {
                statement_list: vec![StatementKind::Expression(expression)],
            }),
            parse_block_node,
        ))(t)?
    };
    let node = ClosureNode {
        argument_list,
        return_type,
        block,
    };

    Ok((t, node))
}

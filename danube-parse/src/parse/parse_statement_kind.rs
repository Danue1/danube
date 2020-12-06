use super::*;

pub fn parse_statement_kind(t: Tokens) -> ParseResult<StatementKind> {
    alt((
        map(parse_attributed(parse_item_kind), |node| {
            StatementKind::Item(Box::new(node))
        }),
        map(parse_let_node, |node| StatementKind::Let(Box::new(node))),
        map(parse_expression_kind, StatementKind::Expression),
        map(parse_symbol(Symbol::Semicolon), |_| {
            StatementKind::Semicolon
        }),
    ))(t)
}

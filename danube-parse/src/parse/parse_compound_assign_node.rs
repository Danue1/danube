use super::*;

pub(super) fn parse_compound_assign_node(t: Tokens) -> ParseResult<CompoundAssignNode> {
    map(
        tuple((
            parse_expression_kind,
            parse_compound_assign_kind,
            parse_expression_kind,
            parse_symbol(Symbol::Semicolon),
        )),
        |(lhs, kind, rhs, _)| CompoundAssignNode { kind, lhs, rhs },
    )(t)
}

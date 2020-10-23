use super::*;

pub(super) fn parse_program_node(t: Tokens) -> ParseResult<ProgramNode> {
    map(many0(parse_attributed(parse_item_kind)), |item_list| {
        ProgramNode { item_list }
    })(t)
}

use super::*;

pub(super) fn parse_while_node(t: Tokens) -> ParseResult<WhileNode> {
    map(
        tuple((
            parse_keyword(Keyword::While),
            parse_condition_node,
            parse_block_node,
        )),
        |(_, condition, block)| WhileNode { condition, block },
    )(t)
}

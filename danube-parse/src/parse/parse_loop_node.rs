use super::*;

pub(super) fn parse_loop_node(t: Tokens) -> ParseResult<LoopNode> {
    map(
        tuple((parse_keyword(Keyword::Loop), parse_block_node)),
        |(_, block)| LoopNode { block },
    )(t)
}

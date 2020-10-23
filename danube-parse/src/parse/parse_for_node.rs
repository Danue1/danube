use super::*;

pub(super) fn parse_for_node(t: Tokens) -> ParseResult<ForNode> {
    map(
        tuple((
            parse_keyword(Keyword::For),
            parse_immutablity_kind,
            parse_pattern_kind,
            parse_keyword(Keyword::In),
            parse_expression_kind,
            parse_block_node,
        )),
        |(_, immutablity, pattern, _, iteration, block)| ForNode {
            immutablity,
            pattern,
            iteration: Box::new(iteration),
            block,
        },
    )(t)
}

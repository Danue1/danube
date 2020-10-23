use super::*;

pub(super) fn parse_conditional_node(t: Tokens) -> ParseResult<ConditionalNode> {
    map(
        tuple((
            parse_keyword(Keyword::If),
            tuple((parse_condition_node, parse_block_node)),
            many0(map(
                tuple((
                    parse_keyword(Keyword::Else),
                    parse_keyword(Keyword::If),
                    tuple((parse_condition_node, parse_block_node)),
                )),
                |(_, _, block)| block,
            )),
            opt(map(
                tuple((parse_keyword(Keyword::Else), parse_block_node)),
                |(_, block)| block,
            )),
        )),
        |(_, if_conditional_branch, else_if_conditional_branch, else_conditional_branch)| {
            ConditionalNode {
                main_branch: if_conditional_branch,
                branch_list: else_if_conditional_branch,
                other: else_conditional_branch,
            }
        },
    )(t)
}

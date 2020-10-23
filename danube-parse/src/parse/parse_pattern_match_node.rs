use super::*;

pub(super) fn parse_pattern_match_node(t: Tokens) -> ParseResult<PatternMatchNode> {
    map(
        tuple((
            parse_keyword(Keyword::Match),
            parse_expression_kind,
            parse_symbol(Symbol::LeftBrace),
            separated_nonempty_list(
                parse_symbol(Symbol::Comma),
                map(
                    tuple((
                        separated_nonempty_list(parse_symbol(Symbol::BitOr), parse_pattern_kind),
                        parse_symbol(Symbol::BranchArrow),
                        parse_body,
                    )),
                    |(pattern, _, body)| (pattern, body),
                ),
            ),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::RightBrace),
        )),
        |(_, condition, _, branch_list, _, _)| PatternMatchNode {
            condition: Box::new(condition),
            branch_list,
        },
    )(t)
}

fn parse_body(t: Tokens) -> ParseResult<BlockNode> {
    alt((parse_block_node, parse_body_shortcut))(t)
}

fn parse_body_shortcut(t: Tokens) -> ParseResult<BlockNode> {
    map(parse_statement_kind, |statement| BlockNode {
        statement_list: vec![statement],
    })(t)
}

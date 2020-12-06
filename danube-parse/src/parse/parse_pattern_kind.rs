use super::*;

pub fn parse_pattern_kind(t: Tokens) -> ParseResult<PatternKind> {
    alt((
        map(parse_keyword(Keyword::Placeholder), |_| {
            PatternKind::Placeholder
        }),
        map(parse_path_node, PatternKind::Path),
    ))(t)
}

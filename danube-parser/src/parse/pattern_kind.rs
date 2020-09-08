use super::*;

pub(super) fn parse_pattern_kind(s: Tokens) -> ParseResult<PatternKind> {
  alt((
    map(parse_keyword(Keyword::Placeholder), |_| {
      PatternKind::Placeholder
    }),
    map(parse_unnamed_struct_node, PatternKind::UnnamedStruct),
    map(parse_named_struct_node, PatternKind::NamedStruct),
    map(parse_literal_kind, PatternKind::Literal),
    map(parse_path_node, PatternKind::Path),
  ))(s)
}

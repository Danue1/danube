use super::*;

pub(super) fn parse_pattern_node(s: Tokens) -> ParseResult<PatternNode> {
  alt((
    map(parse_keyword(Keyword::Placeholder), |_| {
      PatternNode::Placeholder
    }),
    map(parse_unnamed_struct_node, PatternNode::UnnamedStruct),
    map(parse_named_struct_node, PatternNode::NamedStruct),
    map(parse_literal_value_node, PatternNode::Literal),
    map(parse_path_node, PatternNode::Path),
  ))(s)
}

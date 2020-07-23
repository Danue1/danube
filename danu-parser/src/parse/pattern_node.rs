use super::*;
use crate::*;
use nom::{bytes::complete::tag, combinator::map, multi::separated_list};

pub(super) fn pattern_node(s: Span) -> Result<PatternNode> {
  alt((
    map(pattern_node_literal, PatternNode::Literal),
    pattern_node_path,
  ))(s)
}

fn pattern_node_literal(s: Span) -> Result<LiteralValueNode> {
  literal_value_node(s)
}

fn pattern_node_path(s: Span) -> Result<PatternNode> {
  let (s, ident_list) = separated_list(
    tuple((ignore_token0, tag("::"), ignore_token0)),
    positioned(ident_node),
  )(s)?;
  let path = PathNode { ident_list };
  let (s, _) = ignore_token0(s)?;

  match tuple((
    left_parens,
    ignore_token0,
    separated_list(
      tuple((ignore_token0, comma, ignore_token0)),
      positioned(pattern_node),
    ),
    ignore_token0,
    right_parens,
  ))(s)
  {
    Ok((s, (_, _, field_list, _, _))) => Ok((
      s,
      PatternNode::UnnamedStruct(UnnamedStructNode { path, field_list }),
    )),
    _ => Ok((s, PatternNode::Path(path))),
  }
}

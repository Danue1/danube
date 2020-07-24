use super::*;
use crate::*;
use nom::{
  bytes::complete::tag,
  combinator::{map, opt},
  multi::separated_list,
};

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
  enum PatternKind {
    Unnamed(Vec<Positioned<PatternNode>>),
    Named(Vec<FieldNode>),
  }

  let (s, ident_list) = separated_list(
    tuple((ignore_token0, tag("::"), ignore_token0)),
    positioned(ident_node),
  )(s)?;
  let path = PathNode { ident_list };
  let (s, _) = ignore_token0(s)?;

  match alt((
    map(
      tuple((
        left_parens,
        ignore_token0,
        separated_list(
          tuple((ignore_token0, comma, ignore_token0)),
          positioned(pattern_node),
        ),
        opt(tuple((ignore_token0, comma))),
        ignore_token0,
        right_parens,
      )),
      |(_, _, field_list, _, _, _)| PatternKind::Unnamed(field_list),
    ),
    map(
      tuple((
        left_brace,
        ignore_token0,
        separated_list(tuple((ignore_token0, comma, ignore_token0)), field_node),
        ignore_token0,
        right_brace,
      )),
      |(_, _, field_list, _, _)| PatternKind::Named(field_list),
    ),
  ))(s)
  {
    Ok((s, PatternKind::Unnamed(field_list))) => Ok((
      s,
      PatternNode::UnnamedStruct(UnnamedStructNode { path, field_list }),
    )),
    Ok((s, PatternKind::Named(field_list))) => Ok((
      s,
      PatternNode::NamedStruct(NamedStructNode { path, field_list }),
    )),
    _ => Ok((s, PatternNode::Path(path))),
  }
}

fn field_node(s: Span) -> Result<FieldNode> {
  map(
    tuple((
      positioned(ident_node),
      opt(map(
        tuple((
          ignore_token0,
          colon,
          ignore_token0,
          positioned(pattern_node),
        )),
        |(_, _, _, pattern)| pattern,
      )),
    )),
    |(ident, pattern)| {
      let pattern = pattern.unwrap_or_else(|| Positioned {
        start: ident.start.clone(),
        end: ident.end.clone(),
        node: PatternNode::Path(PathNode {
          ident_list: vec![ident.clone()],
        }),
      });
      FieldNode { ident, pattern }
    },
  )(s)
}

use super::*;
use crate::*;
use nom::{
  bytes::complete::tag,
  combinator::{map, opt},
  sequence::tuple,
};

pub(super) fn statement_node(s: Span) -> Result<StatementNode> {
  alt((
    map(let_mut_node, StatementNode::LetMut),
    map(let_node, StatementNode::Let),
    map(if_node, StatementNode::If),
  ))(s)
}

fn let_mut_node(s: Span) -> Result<LetMutNode> {
  map(
    tuple((
      tag("let"),
      ignore_token1,
      tag("mut"),
      ignore_token1,
      positioned(ident_node),
      ignore_token0,
      opt(map(
        tuple((colon, ignore_token0, positioned(type_node), ignore_token0)),
        |(_, _, type_node, _)| type_node,
      )),
      equal,
      ignore_token0,
      positioned(expression_node),
      ignore_token0,
      semicolon,
    )),
    |(_, _, _, _, ident, _, ty, _, _, value, _, _)| LetMutNode { ident, ty, value },
  )(s)
}

fn let_node(s: Span) -> Result<LetNode> {
  map(
    tuple((
      tag("let"),
      ignore_token1,
      positioned(ident_node),
      ignore_token0,
      opt(map(
        tuple((colon, ignore_token0, positioned(type_node), ignore_token0)),
        |(_, _, type_node, _)| type_node,
      )),
      equal,
      ignore_token0,
      positioned(expression_node),
      ignore_token0,
      semicolon,
    )),
    |(_, _, ident, _, ty, _, _, value, _, _)| LetNode { ident, ty, value },
  )(s)
}

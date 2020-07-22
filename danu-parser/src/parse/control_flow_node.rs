use super::*;
use crate::*;
use nom::{
  bytes::complete::tag,
  combinator::{map, opt},
  multi::{many0, many1},
  sequence::tuple,
};

pub(super) fn if_node(s: Span) -> Result<IfNode> {
  map(
    tuple((
      tag("if"),
      ignore_token1,
      positioned(expression_node),
      ignore_token0,
      left_brace,
      ignore_token0,
      many0(map(
        tuple((positioned(statement_node), ignore_token0)),
        |(statement, _)| statement,
      )),
      right_brace,
      opt(many1(map(
        tuple((
          ignore_token0,
          tag("else"),
          ignore_token1,
          tag("if"),
          ignore_token1,
          positioned(expression_node),
          ignore_token0,
          left_brace,
          ignore_token0,
          many0(map(
            tuple((positioned(statement_node), ignore_token0)),
            |(statement, _)| statement,
          )),
          ignore_token0,
          right_brace,
        )),
        |(_, _, _, _, _, condition, _, _, _, statement_list, _, _)| (condition, statement_list),
      ))),
      opt(map(
        tuple((
          ignore_token0,
          tag("else"),
          ignore_token0,
          left_brace,
          ignore_token0,
          many0(map(
            tuple((positioned(statement_node), ignore_token0)),
            |(statement, _)| statement,
          )),
          ignore_token0,
          right_brace,
        )),
        |(_, _, _, _, _, statement_list, _, _)| statement_list,
      )),
    )),
    |(_, _, condition, _, _, _, if_true, _, if_else_if, if_false)| IfNode {
      condition: Box::new(condition),
      if_true,
      if_else_if,
      if_false,
    },
  )(s)
}

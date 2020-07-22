use super::*;
use crate::*;
use nom::{
  bytes::complete::tag,
  combinator::{map, opt},
  multi::{many0, many1},
  sequence::tuple,
};

pub(super) fn statement_conditional_node(s: Span) -> Result<StatementConditionalNode> {
  map(
    tuple((
      condition,
      ignore_token0,
      body,
      opt(many1(map(
        tuple((
          ignore_token0,
          tag("else"),
          ignore_token1,
          condition,
          ignore_token0,
          body,
        )),
        |(_, _, _, condition, _, body)| (condition, body),
      ))),
      opt(map(
        tuple((ignore_token0, tag("else"), ignore_token0, body)),
        |(_, _, _, statement_list)| statement_list,
      )),
    )),
    |(condition, _, if_true, if_else_if, if_false)| StatementConditionalNode {
      condition: Box::new(condition),
      if_true,
      if_else_if,
      if_false,
    },
  )(s)
}

pub(super) fn expression_conditional_node(s: Span) -> Result<ExpressionConditionalNode> {
  map(
    tuple((
      condition,
      ignore_token0,
      body,
      opt(many1(map(
        tuple((
          ignore_token0,
          tag("else"),
          ignore_token1,
          condition,
          ignore_token0,
          body,
        )),
        |(_, _, _, condition, _, body)| (condition, body),
      ))),
      map(
        tuple((ignore_token0, tag("else"), ignore_token0, body)),
        |(_, _, _, statement_list)| statement_list,
      ),
    )),
    |(condition, _, if_true, if_else_if, if_false)| ExpressionConditionalNode {
      condition: Box::new(condition),
      if_true,
      if_else_if,
      if_false,
    },
  )(s)
}

fn condition(s: Span) -> Result<Positioned<ExpressionNode>> {
  map(
    tuple((tag("if"), ignore_token1, positioned(expression_node))),
    |(_, _, condition)| condition,
  )(s)
}

fn body(s: Span) -> Result<Vec<Positioned<StatementNode>>> {
  map(
    tuple((
      left_brace,
      ignore_token0,
      many0(map(
        tuple((positioned(statement_node), ignore_token0)),
        |(statement, _)| statement,
      )),
      right_brace,
    )),
    |(_, _, body, _)| body,
  )(s)
}

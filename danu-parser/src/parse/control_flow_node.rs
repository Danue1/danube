use super::*;
use crate::*;
use nom::{
  bytes::complete::tag,
  combinator::{map, opt},
  multi::{many0, many1, separated_list},
  sequence::tuple,
};

pub(super) fn statement_conditional_node(s: Span) -> Result<StatementConditionalNode> {
  map(
    tuple((
      if_condition,
      ignore_token0,
      body,
      opt(many1(map(
        tuple((
          ignore_token0,
          tag("else"),
          ignore_token1,
          if_condition,
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
      if_condition,
      ignore_token0,
      body,
      opt(many1(map(
        tuple((
          ignore_token0,
          tag("else"),
          ignore_token1,
          if_condition,
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

pub(super) fn loop_node(s: Span) -> Result<LoopNode> {
  map(tuple((tag("loop"), ignore_token0, body)), |(_, _, body)| {
    LoopNode { body }
  })(s)
}

pub(super) fn while_node(s: Span) -> Result<WhileNode> {
  map(
    tuple((
      tag("while"),
      ignore_token1,
      positioned(expression_node),
      ignore_token0,
      body,
    )),
    |(_, _, condition, _, body)| WhileNode {
      condition: Box::new(condition),
      body,
    },
  )(s)
}

pub(super) fn pattern_match_node(s: Span) -> Result<PatternMatchNode> {
  map(
    tuple((
      tag("match"),
      ignore_token1,
      positioned(expression_node),
      ignore_token0,
      left_brace,
      many1(map(
        tuple((
          ignore_token0,
          separated_list(
            tuple((ignore_token0, pipeline, ignore_token0)),
            positioned(pattern_node),
          ),
          ignore_token0,
          tag("=>"),
          ignore_token0,
          pattern_match_branch_statement,
        )),
        |(_, condition_list, _, _, _, statement_list)| (condition_list, statement_list),
      )),
      ignore_token0,
      right_brace,
    )),
    |(_, _, condition, _, _, branch_list, _, _)| PatternMatchNode {
      condition: Box::new(condition),
      branch_list,
    },
  )(s)
}

fn if_condition(s: Span) -> Result<Positioned<ExpressionNode>> {
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

fn pattern_match_branch_statement(s: Span) -> Result<Vec<Positioned<StatementNode>>> {
  alt((
    map(pattern_match_statement_shorthand, |statement| {
      vec![statement]
    }),
    pattern_match_statement_block,
  ))(s)
}

fn pattern_match_statement_shorthand(s: Span) -> Result<Positioned<StatementNode>> {
  let (s, start) = position(s)?;
  let (s, expression) = expression_node(s)?;
  let (s, end) = position(s)?;
  let (s, _) = tuple((ignore_token0, comma))(s)?;

  let node = StatementNode::Expression(expression);
  let positioned = Positioned { start, end, node };

  Ok((s, positioned))
}

fn pattern_match_statement_block(s: Span) -> Result<Vec<Positioned<StatementNode>>> {
  map(
    tuple((
      left_brace,
      ignore_token0,
      many0(map(
        tuple((positioned(statement_node), ignore_token0)),
        |(statement, _)| statement,
      )),
      right_brace,
      opt(tuple((comma, ignore_token0))),
    )),
    |(_, _, expression, _, _)| expression,
  )(s)
}

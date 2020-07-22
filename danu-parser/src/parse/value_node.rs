use crate::*;
use nom::{
  branch::alt,
  bytes::complete::{is_not, tag, take_while1},
  character::complete::anychar,
  combinator::{map, opt},
  sequence::tuple,
};

pub(super) fn value_node(s: Span) -> Result<ValueNode> {
  alt((
    map(value_bool, ValueNode::Bool),
    map(value_char, ValueNode::Char),
    map(value_string, ValueNode::String),
  ))(s)
}

pub(super) fn value_usize(s: Span) -> Result<usize> {
  map(take_while1(is_digit), |value: Span| {
    value.fragment().parse().unwrap()
  })(s)
}

fn value_bool(s: Span) -> Result<bool> {
  alt((map(tag("true"), |_| true), map(tag("false"), |_| false)))(s)
}

fn value_char(s: Span) -> Result<char> {
  map(
    tuple((
      single_quote,
      alt((map(tuple((back_slash, anychar)), |(_, c)| c), anychar)),
      single_quote,
    )),
    |(_, c, _)| c,
  )(s)
}

fn value_string(s: Span) -> Result<String> {
  map(
    tuple((double_quote, opt(is_not("\"")), double_quote)),
    |(_, string, _)| {
      string
        .map(|s| s.fragment().to_string())
        .unwrap_or_else(String::new)
    },
  )(s)
}

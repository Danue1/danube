use crate::*;
use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1},
  combinator::map,
};

pub(super) fn value_node(s: Span) -> Result<ValueNode> {
  map(value_bool, ValueNode::Bool)(s)
}

pub(super) fn value_usize(s: Span) -> Result<usize> {
  map(take_while1(is_digit), |value: Span| {
    value.fragment().parse().unwrap()
  })(s)
}

fn value_bool(s: Span) -> Result<bool> {
  alt((map(tag("true"), |_| true), map(tag("false"), |_| false)))(s)
}

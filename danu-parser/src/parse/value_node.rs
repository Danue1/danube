use crate::*;
use nom::{
  branch::alt,
  bytes::complete::{tag, take, take_while, take_while1},
  character::complete::{anychar, char},
  combinator::{map, opt},
  multi::separated_list,
  sequence::tuple,
};

pub(super) fn literal_value_node(s: Span) -> Result<LiteralValueNode> {
  alt((
    map(value_bool, LiteralValueNode::Bool),
    map(value_char, LiteralValueNode::Char),
    value_numeric,
    map(value_string, LiteralValueNode::String),
    map(value_array, LiteralValueNode::Array),
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

fn value_numeric(s: Span) -> Result<LiteralValueNode> {
  enum Numeric {
    Int(usize),
    Float(usize),
  }

  let sign_to_int = |sign: Option<_>| if sign.is_some() { 1 } else { 0 };
  let len = |s: Span| s.fragment().len();
  fn zero<O, F>(f: F) -> impl Fn(Span) -> Result<O>
  where
    F: Fn(usize) -> O,
  {
    move |s: Span| map(take(0usize), |_| f(0))(s)
  }

  let (ss, sign) = map(opt(hyphen), sign_to_int)(s)?;

  let (ss, majority) = alt((
    map(tag("0"), |_| 1),
    map(
      tuple((take_while1(is_nonzero_digit), take_while(is_digit))),
      |(nonzero_digit, digit)| len(nonzero_digit) + len(digit),
    ),
  ))(ss)?;

  match alt((
    map(tuple((dot, take_while(is_digit))), move |(_, minority)| {
      if minority.fragment().is_empty() {
        Numeric::Int(sign + majority)
      } else {
        Numeric::Float(1 + len(minority))
      }
    }),
    zero(Numeric::Float),
  ))(ss)?
  {
    (_, Numeric::Int(size)) => {
      let (s, numeric) = take(size)(s)?;
      Ok((
        s,
        LiteralValueNode::Int(numeric.fragment().parse().unwrap()),
      ))
    }
    (ss, Numeric::Float(minority)) => {
      let (_, exponential) = alt((
        map(
          tuple((
            alt((char('e'), char('E'))),
            opt(alt((plus, hyphen))),
            take_while1(is_digit),
          )),
          |(_, sign, digit)| 1 + sign_to_int(sign) + len(digit),
        ),
        zero(|size| size),
      ))(ss)?;

      let (s, numeric) = take(sign + majority + minority + exponential)(s)?;

      Ok((
        s,
        LiteralValueNode::Float(numeric.fragment().parse().unwrap()),
      ))
    }
  }
}

fn value_string(s: Span) -> Result<String> {
  string(s)
}

fn value_array(s: Span) -> Result<Vec<LiteralValueNode>> {
  map(
    tuple((
      left_bracket,
      ignore_token0,
      opt(separated_list(
        tuple((ignore_token0, comma, ignore_token0)),
        literal_value_node,
      )),
      ignore_token0,
      opt(tuple((comma, ignore_token0))),
      right_bracket,
    )),
    |(_, _, value_list, _, _, _)| value_list.unwrap_or_else(Vec::new),
  )(s)
}

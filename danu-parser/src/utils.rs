use crate::*;
use nom::{
  branch::alt,
  bytes::complete::{is_a, is_not, tag},
  character::complete::char,
  combinator::{map, opt},
  multi::{many0, many1},
  sequence::tuple,
};

pub(crate) fn name(s: Span) -> Result<String> {
  map(
    tuple((
      alt((alphabet, tag("_"))),
      many0(alt((alphabet, digit, tag("_")))),
    )),
    |(first, tail)| {
      let tail: String = tail.iter().map(|s| s.fragment().to_owned()).collect();
      format!("{}{}", first, tail)
    },
  )(s)
}

pub(crate) fn string(s: Span) -> Result<String> {
  map(
    tuple((double_quote, opt(is_not("\"\r\n")), double_quote)),
    |(_, string, _)| {
      string
        .map(|s| s.fragment().to_string())
        .unwrap_or_else(|| "".to_owned())
    },
  )(s)
}

pub(crate) fn is_nonzero_digit(c: char) -> bool {
  matches!(c, '1'..='9')
}

pub(crate) fn is_digit(c: char) -> bool {
  matches!(c, '0'..='9')
}

pub(crate) fn digit(s: Span) -> Result<Span> {
  is_a("0123456789")(s)
}

pub(crate) fn is_semicolon(c: char) -> bool {
  c == ';'
}

pub(crate) fn is_right_brace(c: char) -> bool {
  c == '}'
}

pub(crate) fn ignore_token0(s: Span) -> Result<()> {
  map(many0(is_a(" \t\r\n")), |_| ())(s)
}

pub(crate) fn ignore_token1(s: Span) -> Result<()> {
  map(many1(is_a(" \t\r\n")), |_| ())(s)
}

pub(crate) fn char_empty(c: char) -> impl Fn(Span) -> Result<()> {
  move |s: Span| map(char(c), |_| ())(s)
}

macro_rules! char_empty {
    ($($expr:expr => $ident:ident,)+) => {
        $(
            pub(crate) fn $ident(s: Span) -> Result<()> {
                char_empty($expr)(s)
            }
        )+
    };
}

char_empty!(
  '(' => left_parens,
  ')' => right_parens,
  '[' => left_bracket,
  ']' => right_bracket,
  '{' => left_brace,
  '}' => right_brace,
  '.' => dot,
  ',' => comma,
  ':' => colon,
  ';' => semicolon,
  '+' => plus,
  '-' => hyphen,
  '=' => equal,
  '\\' => back_slash,
  '|' => pipeline,
  '\'' => single_quote,
  '"' => double_quote,
);

pub(crate) fn alphabet(s: Span) -> Result<Span> {
  is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")(s)
}

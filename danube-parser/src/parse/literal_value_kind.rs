use super::*;

pub(super) fn parse_literal_value_kind(s: Tokens) -> ParseResult<LiteralValueKind> {
  alt((
    map(parse_boolean, LiteralValueKind::Bool),
    map(parse_int, LiteralValueKind::Int),
    map(parse_float, LiteralValueKind::Float),
    map(parse_string, LiteralValueKind::String),
  ))(s)
}

fn parse_boolean(s: Tokens) -> ParseResult<bool> {
  let (s, t) = take(1usize)(s)?;

  match t.list[0] {
    Token::BooleanLiteral(b) => Ok((s, b)),
    _ => Err(nom::Err::Error(nom::error_position!(
      s,
      nom::error::ErrorKind::Count
    ))),
  }
}

pub(super) fn parse_int(s: Tokens) -> ParseResult<i64> {
  let (s, t) = take(1usize)(s)?;

  match t.list[0] {
    Token::IntLiteral(i) => Ok((s, i)),
    _ => Err(nom::Err::Error(nom::error_position!(
      s,
      nom::error::ErrorKind::Count
    ))),
  }
}

fn parse_float(s: Tokens) -> ParseResult<f64> {
  let (s, t) = take(1usize)(s)?;

  match t.list[0] {
    Token::FloatLiteral(f) => Ok((s, f)),
    _ => Err(nom::Err::Error(nom::error_position!(
      s,
      nom::error::ErrorKind::Count
    ))),
  }
}

fn parse_string(s: Tokens) -> ParseResult<String> {
  let (s, t) = take(1usize)(s)?;

  match t.list[0] {
    Token::StringLiteral(ref string) => Ok((s, string.clone())),
    _ => Err(nom::Err::Error(nom::error_position!(
      s,
      nom::error::ErrorKind::Count
    ))),
  }
}

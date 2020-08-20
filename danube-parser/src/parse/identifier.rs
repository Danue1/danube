use super::*;

pub(super) fn parse_identifier(s: Tokens) -> ParseResult<String> {
  let (s, t) = take(1usize)(s)?;

  if let Token::Identifier(ref i) = t.list[0] {
    Ok((s, i.clone()))
  } else {
    Err(nom::Err::Error(nom::error_position!(
      s,
      nom::error::ErrorKind::Count
    )))
  }
}

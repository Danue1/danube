use super::*;

pub(super) fn parse_symbol(symbol: Symbol) -> impl Fn(Tokens) -> ParseResult<()> {
  move |s: Tokens| {
    let (s, t) = take(1usize)(s)?;

    if let Token::Symbol(ref k) = t.list[0] {
      if k == &symbol {
        return Ok((s, ()));
      }
    }
    Err(nom::Err::Error(nom::error_position!(
      s,
      nom::error::ErrorKind::Count
    )))
  }
}

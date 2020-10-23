use super::*;

pub(super) fn parse_symbol(symbol: Symbol) -> impl Fn(Tokens) -> ParseResult<()> {
    move |t: Tokens| {
        let (t, token) = take(1usize)(t)?;

        if let Token::Symbol(ref k) = token.list[0] {
            if k == &symbol {
                return Ok((t, ()));
            }
        }
        Err(nom::Err::Error(nom::error_position!(
            t,
            nom::error::ErrorKind::Count
        )))
    }
}

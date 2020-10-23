use super::*;

pub(super) fn parse_float(t: Tokens) -> ParseResult<f64> {
    let (t, token) = take(1usize)(t)?;

    match token.list[0] {
        Token::FloatLiteral(f) => Ok((t, f)),
        _ => Err(nom::Err::Error(nom::error_position!(
            t,
            nom::error::ErrorKind::Count
        ))),
    }
}

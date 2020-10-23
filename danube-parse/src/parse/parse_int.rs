use super::*;

pub(super) fn parse_int(t: Tokens) -> ParseResult<i64> {
    let (t, token) = take(1usize)(t)?;

    match token.list[0] {
        Token::IntLiteral(i) => Ok((t, i)),
        _ => Err(nom::Err::Error(nom::error_position!(
            t,
            nom::error::ErrorKind::Count
        ))),
    }
}

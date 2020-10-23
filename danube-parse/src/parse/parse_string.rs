use super::*;

pub(super) fn parse_string(t: Tokens) -> ParseResult<String> {
    let (t, token) = take(1usize)(t)?;

    match token.list[0] {
        Token::StringLiteral(ref string) => Ok((t, string.clone())),
        _ => Err(nom::Err::Error(nom::error_position!(
            t,
            nom::error::ErrorKind::Count
        ))),
    }
}

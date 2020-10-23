use super::*;

pub(super) fn parse_identifier(t: Tokens) -> ParseResult<String> {
    let (t, token) = take(1usize)(t)?;

    if let Token::Identifier(ref i) = token.list[0] {
        Ok((t, i.clone()))
    } else {
        Err(nom::Err::Error(nom::error_position!(
            t,
            nom::error::ErrorKind::Count
        )))
    }
}

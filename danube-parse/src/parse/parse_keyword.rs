use super::*;

pub(super) fn parse_keyword(keyword: Keyword) -> impl Fn(Tokens) -> ParseResult<()> {
    move |t: Tokens| {
        let (t, token) = take(1usize)(t)?;

        if let Token::Keyword(ref k) = token.list[0] {
            if k == &keyword {
                return Ok((t, ()));
            }
        }
        Err(nom::Err::Error(nom::error_position!(
            t,
            nom::error::ErrorKind::Count
        )))
    }
}

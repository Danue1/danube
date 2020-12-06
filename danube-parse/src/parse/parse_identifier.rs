use super::*;

pub fn parse_identifier(t: Tokens) -> ParseResult<String> {
    map_opt(take(1usize), |t: Tokens| {
        if let Token::Identifier(ref i) = t.list[0] {
            Some(i.clone())
        } else {
            None
        }
    })(t)
}

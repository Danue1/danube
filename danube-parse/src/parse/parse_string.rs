use super::*;

pub fn parse_string(t: Tokens) -> ParseResult<String> {
    map_opt(take(1usize), |t: Tokens| {
        if let Token::StringLiteral(literal) = t.list[0].to_owned() {
            Some(literal)
        } else {
            None
        }
    })(t)
}

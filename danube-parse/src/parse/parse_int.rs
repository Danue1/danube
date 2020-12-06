use super::*;

pub fn parse_int(t: Tokens) -> ParseResult<i64> {
    map_opt(take(1usize), |t: Tokens| {
        if let Token::IntLiteral(literal) = t.list[0] {
            Some(literal)
        } else {
            None
        }
    })(t)
}

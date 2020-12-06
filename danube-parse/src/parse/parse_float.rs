use super::*;

pub fn parse_float(t: Tokens) -> ParseResult<f64> {
    map_opt(take(1usize), |t: Tokens| {
        if let Token::FloatLiteral(literal) = t.list[0] {
            Some(literal)
        } else {
            None
        }
    })(t)
}

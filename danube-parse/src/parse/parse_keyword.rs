use super::*;

pub fn parse_keyword(keyword: Keyword) -> impl Fn(Tokens) -> ParseResult<()> {
    move |t: Tokens| {
        map_opt(take(1usize), |t: Tokens| {
            if let Token::Keyword(ref k) = t.list[0] {
                if k == &keyword {
                    Some(())
                } else {
                    None
                }
            } else {
                None
            }
        })(t)
    }
}

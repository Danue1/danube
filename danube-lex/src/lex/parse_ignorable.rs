use super::*;

pub(super)
fn parse_ignorable(s: LexSpan) -> LexResult<()> {
    alt((parse_whitespace, parse_comment_line, parse_comment_block))(s)
}

fn parse_whitespace(s: LexSpan) -> LexResult<()> {
    map(many1(is_a(" \t\r\n")), |_| ())(s)
}

fn parse_comment_line(s: LexSpan) -> LexResult<()> {
    let (s, _) = tag("//")(s)?;

    if let Some(line) = s.fragment().lines().next() {
        let (s, _) = take(line.len())(s)?;

        Ok((s, ()))
    } else {
        Ok((s, ()))
    }
}

fn parse_comment_block(s: LexSpan) -> LexResult<()> {
    let (s, _) = tag("/*")(s)?;

    if let Some((size, _)) = s.fragment().match_indices("*/").next() {
        let (s, _) = take(size + 2)(s)?;

        Ok((s, ()))
    } else {
        Ok((LexSpan::new(""), ()))
    }
}

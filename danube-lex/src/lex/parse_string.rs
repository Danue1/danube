use super::*;

pub(super) fn parse_string(s: LexSpan) -> LexResult<String> {
    fn pis(s: LexSpan) -> LexResult<Vec<u8>> {
        let (ss, c) = take(1usize)(s)?;
        match c.fragment().to_owned().as_bytes() {
            b"\"" => Ok((s, vec![])),
            b"\\" => {
                let (s, c) = take(1usize)(ss)?;
                let c = c.fragment().to_owned().as_bytes();
                pis(s).map(|(s, done)| (s, concat_slice_vec(c, done)))
            }
            c => pis(ss).map(|(s, done)| (s, concat_slice_vec(c, done))),
        }
    }

    fn concat_slice_vec(c: &[u8], done: Vec<u8>) -> Vec<u8> {
        let mut new_vec = c.to_vec();
        new_vec.extend(&done);
        new_vec
    }

    fn convert_vec_utf8(v: Vec<u8>) -> std::result::Result<String, std::str::Utf8Error> {
        std::str::from_utf8(v.as_slice()).map(|s| s.to_owned())
    }

    map(
        tuple((tag("\""), map_res(pis, convert_vec_utf8), tag("\""))),
        |(_, string, _)| string,
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let source = "\"foo\"";
        assert_eq!(
            lex(source),
            Ok(vec![Token::StringLiteral("foo".to_owned())])
        );

        let source = "\"foo
bar\"";
        assert_eq!(
            lex(source),
            Ok(vec![Token::StringLiteral("foo\nbar".to_owned())])
        );
    }
}

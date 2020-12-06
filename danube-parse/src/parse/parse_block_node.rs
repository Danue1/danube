use super::*;

pub fn parse_block_node(t: Tokens) -> ParseResult<BlockNode> {
    map(
        tuple((
            parse_symbol(Symbol::LeftBrace),
            many0(parse_statement_kind),
            parse_symbol(Symbol::RightBrace),
        )),
        |(_, statement_list, _)| BlockNode { statement_list },
    )(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let source = r#"{}"#;
        assert_eq!(parse_by(source, parse_block_node), block![],);
    }
}

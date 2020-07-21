use super::*;
use crate::*;
use nom::{bytes::complete::tag, combinator::map, sequence::tuple};

pub(super) fn type_alias_node(s: Span) -> Result<TypeAliasNode> {
  map(
    tuple((
      tag("type"),
      ignore_token1,
      positioned(ident_node),
      ignore_token0,
      equal,
      ignore_token0,
      positioned(type_node),
      ignore_token0,
      semicolon,
    )),
    |(_, _, ident, _, _, _, ty, _, _)| TypeAliasNode { ident, ty },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let source = r#"type Foo = Bar;"#;
    dbg!(parse(source));
  }
}

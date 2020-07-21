use super::*;
use crate::*;
use nom::{
  bytes::complete::tag,
  combinator::{map, opt},
  multi::separated_list,
  sequence::tuple,
};

pub(super) fn enum_node(s: Span) -> Result<EnumNode> {
  map(
    tuple((
      tag("enum"),
      ignore_token1,
      positioned(ident_node),
      ignore_token0,
      equal,
      ignore_token0,
      opt(tuple((pipeline, ignore_token0))),
      separated_list(
        tuple((ignore_token0, pipeline, ignore_token0)),
        positioned(variant_node),
      ),
      ignore_token0,
      semicolon,
    )),
    |(_, _, ident, _, _, _, _, variant_list, _, _)| EnumNode {
      ident,
      variant_list,
    },
  )(s)
}

fn variant_node(s: Span) -> Result<EnumVariantNode> {
  map(
    tuple((
      positioned(ident_node),
      opt(map(
        tuple((
          ignore_token0,
          left_parens,
          ignore_token0,
          positioned(type_node),
          ignore_token0,
          right_parens,
        )),
        |(_, _, _, ty, _, _)| ty,
      )),
    )),
    |(ident, ty)| EnumVariantNode { ident, ty },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let source = r#"enum Foo = Bar;"#;
    dbg!(parse(source));

    let source = r#"enum Foo = Bar | Baz;"#;
    dbg!(parse(source));

    let source = r#"enum Foo = Bar(Bar);"#;
    dbg!(parse(source));

    let source = r#"enum Foo = | Bar;"#;
    dbg!(parse(source));

    let source = r#"enum Foo = | Bar | Baz;"#;
    dbg!(parse(source));
  }
}

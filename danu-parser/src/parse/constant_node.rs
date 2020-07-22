use super::*;
use crate::*;
use nom::{
  bytes::complete::{tag, take_till1},
  combinator::{map, opt},
  sequence::tuple,
};

pub(super) fn constant_node(s: Span) -> Result<ConstantNode> {
  map(
    tuple((
      constant_ident,
      ignore_token0,
      constant_type,
      constant_body_block,
      ignore_token0,
      semicolon,
    )),
    |(ident, _, ty, value, _, _)| ConstantNode { ident, ty, value },
  )(s)
}

pub(super) fn trait_item_constant_node(s: Span) -> Result<TraitItemConstantNode> {
  map(
    tuple((
      constant_ident,
      ignore_token0,
      constant_type,
      opt(constant_body_block),
      ignore_token0,
      semicolon,
    )),
    |(ident, _, ty, default_value, _, _)| TraitItemConstantNode {
      ident,
      ty,
      default_value,
    },
  )(s)
}

fn constant_ident(s: Span) -> Result<Positioned<IdentNode>> {
  map(
    tuple((tag("const"), ignore_token1, positioned(ident_node))),
    |(_, _, ident)| ident,
  )(s)
}

fn constant_type(s: Span) -> Result<Positioned<TypeNode>> {
  map(
    tuple((colon, ignore_token0, positioned(type_node))),
    |(_, _, ty)| ty,
  )(s)
}

fn constant_body_block(s: Span) -> Result<Positioned<String>> {
  map(
    tuple((
      ignore_token0,
      equal,
      ignore_token0,
      positioned(constant_body),
    )),
    |(_, _, _, body)| body,
  )(s)
}

fn constant_body(s: Span) -> Result<String> {
  map(take_till1(is_semicolon), |value: Span| {
    value.fragment().to_string()
  })(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let source = r#"const FOO: Foo = ...;"#;
    dbg!(parse(source));
  }
}

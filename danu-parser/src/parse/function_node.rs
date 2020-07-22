use super::*;
use crate::*;
use nom::{
  branch::alt,
  bytes::complete::{tag, take_till1},
  combinator::{cond, map, opt},
  multi::separated_list,
  sequence::tuple,
};

pub(super) fn function_node(s: Span) -> Result<FunctionNode> {
  map(
    tuple((
      tag("fn"),
      ignore_token1,
      positioned(ident_node),
      ignore_token0,
      function_argument_list,
      ignore_token0,
      opt(map(
        tuple((
          tag("->"),
          ignore_token0,
          positioned(type_node),
          ignore_token0,
        )),
        |(_, _, ty, _)| ty,
      )),
      function_body,
    )),
    |(_, _, ident, _, argument_list, _, return_type, body)| FunctionNode {
      ident,
      return_type,
      argument_list,
      body,
    },
  )(s)
}

pub(super) fn trait_item_function_node(s: Span) -> Result<TraitItemFunctionNode> {
  let (s, (_, _, ident, _, argument_list, _, return_type, body)) = tuple((
    tag("fn"),
    ignore_token1,
    positioned(ident_node),
    ignore_token0,
    function_argument_list,
    ignore_token0,
    opt(map(
      tuple((
        tag("->"),
        ignore_token0,
        positioned(type_node),
        ignore_token0,
      )),
      |(_, _, ty, _)| ty,
    )),
    opt(function_body),
  ))(s)?;
  let (s, _) = cond(body.is_none(), tuple((ignore_token0, semicolon)))(s)?;

  let node = TraitItemFunctionNode {
    ident,
    return_type,
    argument_list,
    body,
  };

  Ok((s, node))
}

fn function_argument_list(s: Span) -> Result<Vec<Positioned<FunctionArgumentNode>>> {
  map(
    tuple((
      left_parens,
      ignore_token0,
      opt(separated_list(
        tuple((ignore_token0, comma, ignore_token0)),
        positioned(function_argument_node),
      )),
      ignore_token0,
      opt(tuple((comma, ignore_token0))),
      right_parens,
    )),
    |(_, _, argument_list, _, _, _)| argument_list.unwrap_or_else(Vec::new),
  )(s)
}

fn function_argument_node(s: Span) -> Result<FunctionArgumentNode> {
  map(
    tuple((
      positioned(ident_node),
      opt(map(
        tuple((ignore_token0, colon, ignore_token0, positioned(type_node))),
        |(_, _, _, ty)| ty,
      )),
    )),
    |(ident, ty)| FunctionArgumentNode { ident, ty },
  )(s)
}

fn function_body(s: Span) -> Result<Positioned<String>> {
  alt((function_body_shorthand, function_body_block))(s)
}

fn function_body_shorthand(s: Span) -> Result<Positioned<String>> {
  map(
    tuple((
      equal,
      ignore_token0,
      positioned(function_body_shorthand_expression),
      semicolon,
    )),
    |(_, _, body, _)| body,
  )(s)
}

fn function_body_shorthand_expression(s: Span) -> Result<String> {
  map(take_till1(is_semicolon), |expression: Span| {
    expression.fragment().to_string()
  })(s)
}

fn function_body_block(s: Span) -> Result<Positioned<String>> {
  map(
    tuple((
      left_brace,
      ignore_token0,
      positioned(function_body_block_expression),
      ignore_token0,
      right_brace,
    )),
    |(_, _, body, _, _)| body,
  )(s)
}

fn function_body_block_expression(s: Span) -> Result<String> {
  map(take_till1(is_right_brace), |expression: Span| {
    expression.fragment().to_string()
  })(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let source = r#"fn foo() { ... }"#;
    dbg!(parse(source));

    let source = r#"fn foo(a) { ... }"#;
    dbg!(parse(source));

    let source = r#"fn foo(a: A) { ... }"#;
    dbg!(parse(source));

    let source = r#"fn foo() -> Bar { ... }"#;
    dbg!(parse(source));

    let source = r#"fn foo(a) -> Bar { ... }"#;
    dbg!(parse(source));

    let source = r#"fn foo(a: A) -> Bar { ... }"#;
    dbg!(parse(source));

    let source = r#"fn foo() -> Bar = ...;"#;
    dbg!(parse(source));

    let source = r#"fn foo(a) -> Bar = ...;"#;
    dbg!(parse(source));

    let source = r#"fn foo(a: A) -> Bar = ...;"#;
    dbg!(parse(source));
  }
}

use super::*;

pub(super) fn parse_type_alias_node(s: Tokens) -> ParseResult<TypeAliasNode> {
  map(
    tuple((
      parse_keyword(Keyword::Type),
      parse_ident_node,
      parse_symbol(Symbol::Assign),
      parse_type_node,
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, ident, _, ty, _)| TypeAliasNode { ident, ty },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> TypeAliasNode {
    let (_, token_list) = lex(s).unwrap();
    let (_, node) = parse_type_alias_node(Tokens::new(&token_list)).unwrap();

    node
  }

  #[test]
  fn test() {
    let source = "type Foo = bool;";
    assert_eq!(
      compile(source),
      TypeAliasNode {
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        ty: TypeNode::Ident(IdentNode {
          raw: "bool".to_owned()
        })
      }
    );
  }
}

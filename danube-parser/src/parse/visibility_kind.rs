use super::*;

pub(super) fn parse_visibility_kind(s: Tokens) -> ParseResult<VisibilityKind> {
  let (s, _) = parse_keyword(Keyword::Public)(s)?;

  match tuple((
    parse_symbol(Symbol::LeftParens),
    parse_visibility,
    parse_symbol(Symbol::RightParens),
  ))(s.clone())
  {
    Ok((s, (_, Visibility::Super, _))) => Ok((s, VisibilityKind::Super)),
    Ok((s, (_, Visibility::Module, _))) => Ok((s, VisibilityKind::Module)),
    Ok((s, (_, Visibility::Restricted(path), _))) => Ok((s, VisibilityKind::Restricted(path))),
    _ => Ok((s, VisibilityKind::Public)),
  }
}

enum Visibility {
  Super,
  Module,
  Restricted(PathNode),
}

fn parse_visibility(s: Tokens) -> ParseResult<Visibility> {
  alt((
    map(parse_keyword(Keyword::Super), |_| Visibility::Super),
    map(parse_keyword(Keyword::Module), |_| Visibility::Module),
    map(
      tuple((parse_keyword(Keyword::In), parse_path_node)),
      |(_, path)| Visibility::Restricted(path),
    ),
  ))(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> VisibilityKind {
    let (_, token_list) = lex(s).unwrap();
    let (_, node) = parse_visibility_kind(Tokens::new(&token_list)).unwrap();

    node
  }

  #[test]
  fn visibility_public() {
    let source = "pub";
    assert_eq!(compile(source), VisibilityKind::Public);
  }

  #[test]
  fn visibility_super() {
    let source = "pub(super)";
    assert_eq!(compile(source), VisibilityKind::Super);
  }

  #[test]
  fn visibility_module() {
    let source = "pub(mod)";
    assert_eq!(compile(source), VisibilityKind::Module);
  }

  #[test]
  fn visibility_restricted() {
    let source = "pub(in foo)";
    assert_eq!(
      compile(source),
      VisibilityKind::Restricted(PathNode {
        ident_list: vec![IdentNode {
          raw: "foo".to_owned()
        }]
      })
    );
  }
}

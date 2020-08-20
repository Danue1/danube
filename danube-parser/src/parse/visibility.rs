use super::*;

pub(super) fn parse_visibility(s: Tokens) -> ParseResult<Visibility> {
  let (s, _) = parse_keyword(Keyword::Public)(s)?;

  match tuple((
    parse_symbol(Symbol::LeftParens),
    parse_visibility_kind,
    parse_symbol(Symbol::RightParens),
  ))(s.clone())
  {
    Ok((s, (_, VisibilityKind::Super, _))) => Ok((s, Visibility::Super)),
    Ok((s, (_, VisibilityKind::Module, _))) => Ok((s, Visibility::Module)),
    Ok((s, (_, VisibilityKind::Restricted(path), _))) => Ok((s, Visibility::Restricted(path))),
    _ => Ok((s, Visibility::Public)),
  }
}

enum VisibilityKind {
  Super,
  Module,
  Restricted(PathNode),
}

fn parse_visibility_kind(s: Tokens) -> ParseResult<VisibilityKind> {
  alt((
    map(parse_keyword(Keyword::Super), |_| VisibilityKind::Super),
    map(parse_keyword(Keyword::Module), |_| VisibilityKind::Module),
    map(
      tuple((parse_keyword(Keyword::In), parse_path_node)),
      |(_, path)| VisibilityKind::Restricted(path),
    ),
  ))(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> Visibility {
    let (_, token_list) = lex(s).unwrap();
    let (_, node) = parse_visibility(Tokens::new(&token_list)).unwrap();

    node
  }

  #[test]
  fn visibility_public() {
    let source = "pub";
    assert_eq!(compile(source), Visibility::Public);
  }

  #[test]
  fn visibility_super() {
    let source = "pub(super)";
    assert_eq!(compile(source), Visibility::Super);
  }

  #[test]
  fn visibility_module() {
    let source = "pub(mod)";
    assert_eq!(compile(source), Visibility::Module);
  }

  #[test]
  fn visibility_restricted() {
    let source = "pub(in foo)";
    assert_eq!(
      compile(source),
      Visibility::Restricted(PathNode {
        ident_list: vec![IdentNode {
          raw: "foo".to_owned()
        }]
      })
    );
  }
}

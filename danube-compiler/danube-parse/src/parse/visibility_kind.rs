use crate::{Error, Parse};
use danube_ast::VisibilityKind;

impl<'parse> Parse<'parse> {
    pub fn parse_visibility_kind(&mut self) -> Result<VisibilityKind, Error> {
        if !identifier!(self.cursor => Pub) {
            return Ok(VisibilityKind::Current);
        }
        if !symbol!(self.cursor => LeftParens) {
            return Ok(VisibilityKind::Public);
        }

        let visibility_kind = VisibilityKind::Restricted(self.parse_path_node()?);

        if symbol!(self.cursor => RightParens) {
            Ok(visibility_kind)
        } else {
            Err(Error::Invalid)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;
    use danube_ast::{IdentNode, PathKind, PathNode, VisibilityKind};
    use danube_lex::Lex;
    use danube_token::{SymbolInterner, Token};

    #[test]
    fn current() {
        let source = "";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_visibility_kind(),
            Ok(VisibilityKind::Current)
        );
    }

    #[test]
    fn public() {
        let source = "pub";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_visibility_kind(),
            Ok(VisibilityKind::Public)
        );
    }

    #[test]
    fn public_super() {
        let source = "pub(super)";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_visibility_kind(),
            Ok(VisibilityKind::Restricted(PathNode {
                kinds: vec![PathKind::Super]
            }))
        );
    }

    #[test]
    fn public_package() {
        let source = "pub(package)";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_visibility_kind(),
            Ok(VisibilityKind::Restricted(PathNode {
                kinds: vec![PathKind::Package]
            }))
        );
    }

    #[test]
    fn public_restricted() {
        let source = "pub(foo)";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();
        let mut interner = SymbolInterner::default();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_visibility_kind(),
            Ok(VisibilityKind::Restricted(PathNode {
                kinds: vec![PathKind::Ident(IdentNode {
                    symbol: interner.intern("foo")
                })]
            }))
        );
    }
}

use crate::{Error, Parse};
use danube_ast::EnumNode;
use danube_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn parse_enum_node(&mut self) -> Result<EnumNode, Error> {
        let ident = self.parse_ident_node()?;
        let generics = self.parse_generic_nodes()?;

        match symbol!(self.cursor) {
            Some(TokenKind::Semicolon) => {
                self.cursor.next();

                Ok(EnumNode {
                    ident,
                    generics,
                    variants: vec![],
                })
            }
            Some(TokenKind::LeftBrace) => {
                self.cursor.next();

                let mut variants = vec![];

                while !symbol!(self.cursor => RightBrace) {
                    variants.push(self.parse_enum_variant_node()?);

                    if !symbol!(self.cursor => Comma) {
                        if symbol!(self.cursor => RightBrace) {
                            break;
                        }

                        return Err(Error::Invalid);
                    }
                }

                Ok(EnumNode {
                    ident,
                    generics,
                    variants,
                })
            }
            _ => Err(Error::Invalid),
        }
    }
}

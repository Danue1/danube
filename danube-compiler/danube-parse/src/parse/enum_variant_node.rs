use crate::{Error, Parse};
use danube_ast::{EnumVariantKind, EnumVariantNode};
use danube_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn parse_enum_variant_node(&mut self) -> Result<EnumVariantNode, Error> {
        let ident = self.parse_ident_node()?;

        match symbol!(self.cursor) {
            Some(TokenKind::LeftParens) => {
                self.cursor.next();

                let mut variants = vec![];

                while !symbol!(self.cursor => RightParens) {
                    variants.push(self.parse_type_node()?);

                    if !symbol!(self.cursor => Comma) {
                        if symbol!(self.cursor => RightParens) {
                            break;
                        }

                        return Err(Error::Invalid);
                    }
                }

                Ok(EnumVariantNode {
                    ident,
                    kind: Some(EnumVariantKind::Unnamed(variants)),
                })
            }
            Some(TokenKind::LeftBrace) => {
                self.cursor.next();

                let mut variants = vec![];

                while !symbol!(self.cursor => RightBrace) {
                    let ident = self.parse_ident_node()?;
                    let ty = if symbol!(self.cursor => Colon) {
                        self.parse_type_node()?
                    } else {
                        return Err(Error::Invalid);
                    };
                    variants.push((ident, ty));

                    if !symbol!(self.cursor => Comma) {
                        if symbol!(self.cursor => RightBrace) {
                            break;
                        }

                        return Err(Error::Invalid);
                    }
                }

                Ok(EnumVariantNode {
                    ident,
                    kind: Some(EnumVariantKind::Named(variants)),
                })
            }
            _ => Ok(EnumVariantNode { ident, kind: None }),
        }
    }
}

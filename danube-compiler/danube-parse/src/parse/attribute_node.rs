use crate::{Error, Parse};
use danube_ast::{AttributeNode, DUMMY_ATTRIBUTE_ID};

impl<'parse> Parse<'parse> {
    pub fn parse_package_attributes(&mut self) -> Result<Vec<AttributeNode>, Error> {
        let mut attributes = vec![];

        while let Some(attribute) = self.parse_package_attribute()? {
            attributes.push(attribute);
        }

        Ok(attributes)
    }

    pub fn parse_item_attributes(&mut self) -> Result<Vec<AttributeNode>, Error> {
        let mut attributes = vec![];

        while let Some(attribute) = self.parse_item_attribute()? {
            attributes.push(attribute);
        }

        Ok(attributes)
    }

    #[inline]
    fn parse_package_attribute(&mut self) -> Result<Option<AttributeNode>, Error> {
        if symbol!(self.cursor => Hash) {
            if symbol!(self.cursor => Exclamation) {
                let attribute = self.parse_attribute_inner()?;

                Ok(Some(attribute))
            } else {
                Err(Error::Invalid)
            }
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn parse_item_attribute(&mut self) -> Result<Option<AttributeNode>, Error> {
        if symbol!(self.cursor => Hash) {
            let attribute = self.parse_attribute_inner()?;

            Ok(Some(attribute))
        } else {
            Ok(None)
        }
    }

    fn parse_attribute_inner(&mut self) -> Result<AttributeNode, Error> {
        if !symbol!(self.cursor => LeftBracket) {
            return Err(Error::Invalid);
        }
        let path = if let Some(path) = self.parse_path_node()? {
            path
        } else {
            return Err(Error::Invalid);
        };
        let args = if !symbol!(self.cursor => LeftParens) {
            vec![]
        } else {
            let mut args = vec![];

            while !symbol!(self.cursor => RightParens) {
                let ident = self.parse_ident_node()?;
                let expression = if symbol!(self.cursor => Eq) {
                    Some(self.parse_expression_node()?)
                } else {
                    None
                };

                args.push((ident, expression));

                if !symbol!(self.cursor => Comma) {
                    if symbol!(self.cursor => RightParens) {
                        break;
                    }

                    return Err(Error::Invalid);
                }
            }

            args
        };

        if symbol!(self.cursor => RightBracket) {
            Ok(AttributeNode {
                id: DUMMY_ATTRIBUTE_ID,
                path,
                args,
            })
        } else {
            Err(Error::Invalid)
        }
    }
}

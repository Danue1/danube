use crate::{Error, Parse};
use danube_ast::{ImplementItemKind, ImplementItemNode, DUMMY_NODE_ID};
use danube_token::keywords;

impl<'parse> Parse<'parse> {
    pub fn parse_implement_item_nodes(&mut self) -> Result<Vec<ImplementItemNode>, Error> {
        if !symbol!(self.cursor => LeftBrace) {
            return Err(Error::Invalid);
        }

        let mut items = vec![];

        while !symbol!(self.cursor => RightBrace) {
            items.push(self.parse_implement_item_node()?);
        }

        Ok(items)
    }

    pub fn parse_implement_item_node(&mut self) -> Result<ImplementItemNode, Error> {
        let attributes = self.parse_item_attributes()?;

        match identifier!(self.cursor) {
            Some(keywords::Type) => {
                self.cursor.next();

                Ok(ImplementItemNode {
                    id: DUMMY_NODE_ID,
                    attributes,
                    kind: ImplementItemKind::Type(self.parse_type_alias_node()?),
                })
            }
            Some(keywords::Const) => {
                self.cursor.next();

                Ok(ImplementItemNode {
                    id: DUMMY_NODE_ID,
                    attributes,
                    kind: ImplementItemKind::Constant(self.parse_constant_node()?),
                })
            }
            Some(keywords::Fn) => {
                self.cursor.next();

                Ok(ImplementItemNode {
                    id: DUMMY_NODE_ID,
                    attributes,
                    kind: ImplementItemKind::Function(self.parse_function_node()?),
                })
            }
            _ => Err(Error::Invalid),
        }
    }
}

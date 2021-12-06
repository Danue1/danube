use crate::{Error, Parse};
use danube_ast::{ItemKind, ItemNode};
use danube_token::Keyword;

impl<'parse> Parse<'parse> {
    pub fn parse_item_nodes(&mut self) -> Result<Vec<ItemNode>, Error> {
        let mut items = vec![];
        while let Some(item) = self.parse_item_node()? {
            items.push(item);
        }

        Ok(items)
    }

    pub fn parse_item_node(&mut self) -> Result<Option<ItemNode>, Error> {
        let attributes = self.parse_item_attributes()?;
        let visibility = self.parse_visibility_kind()?;

        let kind = match keyword!(self.cursor) {
            Some(Keyword::Use) => {
                self.cursor.next();

                ItemKind::Use(self.parse_use_node()?)
            }
            Some(Keyword::Enum) => {
                self.cursor.next();

                ItemKind::Enum(self.parse_enum_node()?)
            }
            Some(Keyword::Fn) => {
                self.cursor.next();

                ItemKind::Function(self.parse_function_node()?)
            }
            Some(Keyword::Type) => {
                self.cursor.next();

                ItemKind::TypeAlias(self.parse_type_alias_node()?)
            }
            Some(Keyword::Trait) => {
                self.cursor.next();

                ItemKind::Trait(self.parse_trait_node()?)
            }
            Some(Keyword::Const) => {
                self.cursor.next();

                ItemKind::Constant(self.parse_constant_node()?)
            }
            Some(Keyword::Impl) => {
                self.cursor.next();

                ItemKind::Implement(self.parse_implement_node()?)
            }
            _ => {
                return if attributes.is_empty() {
                    Ok(None)
                } else {
                    Err(Error::Invalid)
                }
            }
        };

        Ok(Some(ItemNode {
            id: self.resolver.next_id().into(),
            attributes,
            visibility,
            kind,
        }))
    }
}

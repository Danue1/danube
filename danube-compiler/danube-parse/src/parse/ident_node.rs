use crate::{Context, Error, Parse};
use danube_ast::{IdentNode, DUMMY_NODE_ID};

impl Parse for IdentNode {
    type Output = IdentNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        match identifier!(context.cursor) {
            Some(symbol) => {
                context.cursor.next();

                Ok(IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol,
                })
            }
            None => Err(Error::Invalid),
        }
    }
}

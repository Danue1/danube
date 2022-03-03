use crate::{Context, Error, Parse};
use danube_ast::{ArgumentNode, ExpressionNode, IdentNode, DUMMY_NODE_ID};

pub(crate) struct ArgumentNodeList;

impl Parse for ArgumentNodeList {
    type Output = Vec<ArgumentNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        if symbol!(context.cursor => LeftParens) {
            return Ok(vec![]);
        }

        let mut arguments = vec![];

        while !symbol!(context.cursor => RightParens) {
            arguments.push(ArgumentNode::parse(context)?);

            if !symbol!(context.cursor => Comma) {
                if symbol!(context.cursor => RightParens) {
                    break;
                }
            }
        }

        Ok(arguments)
    }
}

impl Parse for ArgumentNode {
    type Output = ArgumentNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let mut cursor = context.cursor.clone();
        let ident = if let Some(symbol) = identifier!(cursor) {
            let symbol = symbol.clone();

            if symbol!(cursor => Colon) {
                context.cursor.next();
                context.cursor.next();

                Some(IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol,
                })
            } else {
                None
            }
        } else {
            None
        };

        Ok(ArgumentNode {
            id: DUMMY_NODE_ID,
            ident,
            expression: ExpressionNode::parse(context)?,
        })
    }
}

use crate::{Context, Error, Parse};
use danube_ast::{BlockNode, StatementNode, DUMMY_NODE_ID};

impl Parse for BlockNode {
    type Output = BlockNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        if !symbol!(context.cursor => LeftBrace) {
            return Err(Error::Invalid);
        }

        let mut statements = vec![];

        while !symbol!(context.cursor => RightBrace) {
            statements.push(StatementNode::parse(context)?);
        }

        Ok(BlockNode {
            id: DUMMY_NODE_ID,
            statements,
        })
    }
}

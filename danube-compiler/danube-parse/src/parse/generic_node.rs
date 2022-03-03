use crate::{Context, Error, Parse, ParseList};
use danube_ast::{GenericNode, IdentNode, PathNode, DUMMY_NODE_ID};

impl ParseList for GenericNode {
    type Output = GenericNode;

    fn parse_list(context: &mut Context) -> Result<Vec<Self::Output>, Error> {
        if !symbol!(context.cursor => LeftChevron) {
            return Ok(vec![]);
        }

        let mut generics = vec![];

        while !symbol!(context.cursor => RightChevron) {
            generics.push(GenericNode::parse(context)?);

            if !symbol!(context.cursor => Comma) {
                if symbol!(context.cursor => RightChevron) {
                    break;
                }

                return Err(Error::Invalid);
            }
        }

        Ok(generics)
    }
}

impl Parse for GenericNode {
    type Output = GenericNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let ident = IdentNode::parse(context)?;
        let traits = if symbol!(context.cursor => Colon) {
            let mut paths = if let Some(path) = PathNode::parse(context)? {
                vec![path]
            } else {
                return Err(Error::Invalid);
            };

            while symbol!(context.cursor => Plus) {
                if let Some(path) = PathNode::parse(context)? {
                    paths.push(path);
                } else {
                    return Err(Error::Invalid);
                }
            }

            paths
        } else {
            vec![]
        };
        let default = if symbol!(context.cursor => Eq) {
            if let Some(path) = PathNode::parse(context)? {
                Some(path)
            } else {
                return Err(Error::Invalid);
            }
        } else {
            None
        };

        Ok(GenericNode {
            id: DUMMY_NODE_ID,
            ident,
            traits,
            default,
        })
    }
}

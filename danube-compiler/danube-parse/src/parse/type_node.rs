use crate::{Context, Parse};
use danube_ast::{ImmutabilityKind, TypeKind, TypeNode, DUMMY_NODE_ID};

impl Parse for TypeNode {
    type Output = TypeNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        Ok(TypeNode {
            id: DUMMY_NODE_ID,
            immutability: ImmutabilityKind::parse(context)?,
            kind: TypeKind::parse(context)?,
        })
    }
}

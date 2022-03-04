use crate::{Context, Parse};
use danube_ast::{TypeNode, UnnamedStructField, VisibilityKind};

impl Parse for UnnamedStructField {
    type Output = UnnamedStructField;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let visibility = VisibilityKind::parse(context)?;
        let ty = TypeNode::parse(context)?;

        Ok(UnnamedStructField { visibility, ty })
    }
}

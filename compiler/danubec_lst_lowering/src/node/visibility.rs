use super::lower_path;
use danubec_diagnostic::Diagnostic;
use danubec_middle::{ast, lst};

pub fn lower_visibility(
    visibility: Option<lst::Visibility>,
) -> Result<ast::Visibility, Diagnostic> {
    let kind = match visibility {
        Some(visibility) => match visibility.kind() {
            Some(lst::VisibilityKind::Crate(_)) => ast::VisibilityKind::Crate,
            Some(lst::VisibilityKind::Super(_)) => ast::VisibilityKind::Super,
            Some(lst::VisibilityKind::In(in_)) => {
                let path = opt!(in_.path(), "ICE: Identifier not found");
                let path = lower_path(path)?;

                ast::VisibilityKind::In(path)
            }
            None => ast::VisibilityKind::Public,
        },
        None => ast::VisibilityKind::Private,
    };

    Ok(ast::Visibility { kind })
}

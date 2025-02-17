use super::lower_path;
use danubec_diagnostic::Diagnostic;
use danubec_middle::{ast, lst};

pub fn lower_type(ty: lst::Type) -> Result<ast::Type, Diagnostic> {
    let kind = opt!(ty.kind(), "ICE: TypeKind not found");
    let kind = lower_type_kind(kind)?;

    Ok(ast::Type { kind })
}

#[allow(unused)]
pub fn lower_type_kind(kind: lst::TypeKind) -> Result<ast::TypeKind, Diagnostic> {
    match kind {
        lst::TypeKind::Never(never) => {
            opt!(never.exclamation(), "ICE: Exclamation not found");

            Ok(ast::TypeKind::Never)
        }
        lst::TypeKind::Path(path) => {
            let path = opt!(path.path(), "ICE: Path not found");
            let path = lower_path(path)?;

            Ok(ast::TypeKind::Path(path))
        }
        lst::TypeKind::Slice(slice) => {
            let ty = opt!(slice.ty(), "ICE: Type not found");
            let ty = lower_type(ty)?;

            Ok(ast::TypeKind::Slice(Box::new(ty)))
        }
        lst::TypeKind::Tuple(tuple) => {
            let mut types = vec![];
            for element in tuple.elements() {
                let ty = opt!(element.ty(), "ICE: Type not found");

                types.push(lower_type(ty)?);
            }

            Ok(ast::TypeKind::Tuple(types))
        }
    }
}

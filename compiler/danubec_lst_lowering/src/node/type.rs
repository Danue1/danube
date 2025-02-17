use danubec_diagnostic::Diagnostic;
use danubec_middle::{ast, lst};

pub fn lower_type(ty: lst::Type) -> Result<ast::Type, Diagnostic> {
    let kind = opt!(ty.kind(), "ICE: TypeKind not found");
    let kind = lower_type_kind(kind)?;

    Ok(ast::Type { kind })
}

#[allow(unused)]
pub fn lower_type_kind(kind: lst::TypeKind) -> Result<ast::TypeKind, Diagnostic> {
    std::todo!();
}

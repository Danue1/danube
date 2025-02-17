use super::{lower_identifier, lower_type};
use danubec_diagnostic::Diagnostic;
use danubec_middle::{ast, lst};

pub fn lower_path(path: lst::Path) -> Result<ast::Path, Diagnostic> {
    let mut segments = vec![];
    for segment in path.segments() {
        segments.push(lower_path_segment(segment)?);
    }

    Ok(ast::Path { segments })
}

pub fn lower_path_segment(segment: lst::PathSegment) -> Result<ast::PathSegment, Diagnostic> {
    let ident = opt!(segment.identifier(), "ICE: Identifier not found");
    let ident = lower_identifier(ident)?;

    let mut types = vec![];
    if let Some(arguments) = segment.type_argument() {
        for ty in arguments.types() {
            types.push(lower_type(ty)?);
        }
    }

    Ok(ast::PathSegment { ident, types })
}

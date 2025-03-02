use crate::lower_definition;
use danubec_diagnostic::Diagnostic;
use danubec_middle::{ast, lst};

pub fn lower_root(krate: lst::Krate) -> Result<ast::Krate, Diagnostic> {
    let mut definitions = vec![];
    for definition in krate.root().definitions() {
        definitions.push(lower_definition(definition, krate.modules())?);
    }

    Ok(ast::Krate { definitions })
}

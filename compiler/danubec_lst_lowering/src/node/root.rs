use crate::lower_definition;
use danubec_diagnostic::Diagnostic;
use danubec_middle::{ast, lst};

pub fn lower_root(root: lst::Root) -> Result<ast::Root, Diagnostic> {
    let mut definitions = vec![];
    for node in root.definitions() {
        definitions.push(lower_definition(node)?);
    }

    Ok(ast::Root { definitions })
}

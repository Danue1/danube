use danubec_diagnostic::Diagnostic;
use danubec_middle::lst;
use danubec_symbol::Symbol;

pub fn lower_identifier(identifier: lst::Identifier) -> Result<Symbol, Diagnostic> {
    let ident = identifier.to_string();

    Ok(Symbol::new(&ident))
}

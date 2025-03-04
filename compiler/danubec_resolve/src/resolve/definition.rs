use super::Resolver;
use crate::collect::ScopeIndex;
use danubec_middle::ast;
use danubec_symbol::Symbol;

impl Resolver {
    pub fn resolve_krate(&mut self, name: Symbol, krate: &ast::Krate) {
        if let Some(module) = self.collector.find_krate(name) {
            let scope = self.collector[module].scope();
            for definition in &krate.definitions {
                self.resolve_definition(scope, &definition);
            }
        }
    }

    fn resolve_definition(&mut self, scope: ScopeIndex, definition: &ast::Definition) {
        std::todo!();
    }
}

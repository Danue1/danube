use super::{Symbol, SymbolTable};

pub struct Scope {
    pub definitions: SymbolTable,
    pub variables: SymbolTable,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            definitions: SymbolTable::new(),
            variables: SymbolTable::new(),
        }
    }

    #[inline]
    pub fn add_definition(&mut self, name: &str, symbol: Symbol) {
        self.definitions.define(name, symbol);
    }

    #[inline]
    pub fn add_variable(&mut self, name: &str, symbol: Symbol) {
        self.variables.define(name, symbol);
    }
}

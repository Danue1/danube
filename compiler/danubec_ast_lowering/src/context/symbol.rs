use std::collections::HashMap;

pub struct SymbolTable {
    symbols: HashMap<String, Vec<Symbol>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(usize);

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, symbol: Symbol) {
        self.symbols
            .entry(name.to_owned())
            .or_default()
            .push(symbol);
    }
}

impl Symbol {
    pub const fn from_usize(id: usize) -> Self {
        Symbol(id)
    }

    pub const fn as_usize(&self) -> usize {
        self.0
    }
}

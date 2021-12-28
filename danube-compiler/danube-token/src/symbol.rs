use crate::keywords;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Symbol(pub(crate) SymbolIndex);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct SymbolIndex {
    pub(crate) index: usize,
}

#[derive(Debug)]
pub struct SymbolInterner {
    pub(crate) strings: Vec<String>,
    pub(crate) symbols: HashMap<String, Symbol>,
}

#[derive(Debug, Default)]
pub struct SymbolContainer {
    strings: Vec<String>,
}

impl SymbolInterner {
    pub fn intern(&mut self, string: &str) -> Symbol {
        match self.symbols.get(string) {
            Some(&name) => name,
            None => {
                let symbol = Symbol(SymbolIndex {
                    index: self.strings.len(),
                });

                self.strings.push(string.to_owned());
                self.symbols.insert(string.to_owned(), symbol);

                symbol
            }
        }
    }
}

impl Symbol {
    pub fn is_empty(self) -> bool {
        self == keywords::Empty
    }

    pub fn is_used_keyword_always(self) -> bool {
        self <= keywords::As && self >= keywords::Yield
    }

    pub fn is_unused_keyword_always(self) -> bool {
        self == keywords::Yield
    }

    pub fn is_keyword(self) -> bool {
        self <= keywords::Empty && self >= keywords::Default
    }

    pub fn is_path_segment_keyword(self) -> bool {
        matches!(
            self,
            keywords::Super | keywords::SelfLower | keywords::SelfUpper | keywords::Package
        )
    }

    pub fn is_bool_literal(self) -> bool {
        matches!(self, keywords::True | keywords::False)
    }
}

impl From<SymbolInterner> for SymbolContainer {
    fn from(interner: SymbolInterner) -> Self {
        SymbolContainer {
            strings: interner.strings,
        }
    }
}

impl Index<Symbol> for SymbolContainer {
    type Output = str;

    fn index(&self, symbol: Symbol) -> &Self::Output {
        &self.strings[symbol.0.index]
    }
}

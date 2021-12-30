use crate::keywords;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Symbol(pub(crate) SymbolIndex);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct SymbolIndex {
    pub(crate) index: usize,
}

#[derive(Debug)]
pub(crate) struct SymbolInterner {
    pub(crate) strings: Vec<&'static str>,
    pub(crate) symbols: HashMap<&'static str, Symbol>,
}

lazy_static! {
    static ref INTERNER: Mutex<SymbolInterner> = Mutex::new(SymbolInterner::default());
}

impl Symbol {
    pub fn intern(string: &str) -> Symbol {
        INTERNER.lock().unwrap().intern(string)
    }

    pub fn get(&self) -> &'static str {
        INTERNER.lock().unwrap().strings[self.0.index]
    }
}

impl SymbolInterner {
    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(&symbol) = self.symbols.get(string) {
            symbol
        } else {
            let symbol = Symbol(SymbolIndex {
                index: self.strings.len(),
            });

            let string = Box::leak(string.into());

            self.strings.push(string);
            self.symbols.insert(string, symbol);

            symbol
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

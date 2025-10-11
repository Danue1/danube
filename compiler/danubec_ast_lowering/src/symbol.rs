#[derive(Debug)]
pub struct SymbolInterner {
    inner: indexmap::IndexSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(usize);

impl SymbolInterner {
    pub fn new() -> Self {
        Self {
            inner: indexmap::IndexSet::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> Symbol {
        let (index, _) = self.inner.insert_full(s.to_owned());

        Symbol(index)
    }
}

impl std::ops::Index<Symbol> for SymbolInterner {
    type Output = str;

    #[inline]
    fn index(&self, Symbol(index): Symbol) -> &Self::Output {
        &self.inner[index]
    }
}

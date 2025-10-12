use fxhash::FxHashMap;
use indexmap::IndexSet;

slotmap::new_key_type! {
    pub struct FileId;

    pub struct ModuleId;

    pub struct ScopeId;

    pub struct ImportId;

    pub struct DefinitionId;

    pub struct LocalId;
}

#[derive(Debug)]
pub struct SymbolInterner {
    inner: IndexSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(usize);

impl SymbolInterner {
    pub fn new() -> Self {
        Self {
            inner: IndexSet::new(),
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

impl std::ops::Index<Symbol> for FxHashMap<Symbol, Vec<DefinitionId>> {
    type Output = Vec<DefinitionId>;

    #[inline]
    fn index(&self, index: Symbol) -> &Self::Output {
        &self[&index]
    }
}

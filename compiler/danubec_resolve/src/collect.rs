mod definition;

use danubec_middle::hir;
use danubec_symbol::Symbol;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Collector {
    modules: Vec<Module>,
    scopes: Vec<Scope>,
    krates: HashMap<Symbol, ModuleIndex>,
}

#[derive(Debug, Clone, Copy)]
pub struct ModuleIndex(usize);

#[derive(Debug, Clone, Copy)]
pub struct ScopeIndex(usize);

#[derive(Debug)]
pub struct Module {
    parent: Option<ModuleIndex>,
    scope: ScopeIndex,

    submodules: HashMap<Symbol, ModuleIndex>,
}

#[derive(Debug)]
pub struct Scope {
    parent: Option<ScopeIndex>,
    module: ModuleIndex,
    current: ScopeIndex,

    /// `use crate::foo;`
    /// `use crate::foo as bar;`
    /// `use super::foo;`
    /// `use super::foo as bar;`
    /// `use foo::bar;`
    /// `use foo::bar as baz;`
    named_imports: Vec<(QualifiedPath, Option<Symbol>)>,

    /// `use crate::*;`
    /// `use crate::foo::*;`
    /// `use super::*;`
    /// `use super::foo::*;`
    /// `use foo::*;`
    glob_imports: Vec<QualifiedPath>,

    submodules: HashMap<Symbol, ModuleIndex>,
    types: NameBinding,
    variables: NameBinding,
}

#[derive(Debug)]
pub struct NameBinding {
    kind: RibKind,
    symbols: HashMap<Symbol, Binding>,
}

#[derive(Debug, Clone)]
pub struct Binding {
    hir_id: hir::HirId,
    scope: Option<ScopeIndex>,
}

#[derive(Debug, Clone, Copy)]
pub enum RibKind {
    CrateRoot,

    // Type namespace
    Struct,
    Enum,
    Variant,
    Trait,
    TypeAlias,
    AssociatedType,
    TypeParameter,

    // Value namespace
    Function,
    Const,
    Static,
    AssociatedFunction,
    AssociatedConst,

    // No namespace
    Field,
    Constructor,
    Use,
    Implement,
    Closure,
    Block,
}

#[derive(Debug)]
pub struct QualifiedPath {
    kind: QualifiedPathKind,
    segments: Vec<Symbol>,
}

#[derive(Debug, Clone, Copy)]
pub enum QualifiedPathKind {
    /// `crate::foo::bar`
    Crate,

    /// `super::foo::bar` // usize = 1
    /// `super::super::foo::bar` // usize = 2
    Super(usize),

    /// `foo::bar`
    Relative,
}

#[derive(Debug, Clone, Copy)]
pub enum Namespace {
    Type,
    Value,
}

impl Collector {
    pub fn new() -> Self {
        Self {
            modules: vec![],
            scopes: vec![],
            krates: HashMap::new(),
        }
    }

    pub fn new_krate(&mut self, name: Symbol) -> ScopeIndex {
        let module = ModuleIndex(self.modules.len());
        let scope = ScopeIndex(self.scopes.len());
        self.scopes.push(Scope::new_root(module, scope));
        self.modules.push(Module::new_root(scope));
        self.krates.insert(name, module);

        scope
    }

    pub fn new_module(&mut self, name: Symbol, parent: ModuleIndex) -> ScopeIndex {
        let module = ModuleIndex(self.modules.len());
        let scope = ScopeIndex(self.scopes.len());
        self.scopes.push(Scope::new_root(module, scope));
        self.modules.push(Module::new_child(parent, scope));
        self[parent].submodules.insert(name, module);

        scope
    }

    pub fn new_scope(&mut self, parent: ScopeIndex, kind: RibKind) -> ScopeIndex {
        let scope = ScopeIndex(self.scopes.len());
        self.scopes.push(self[parent].new_block(scope, kind));

        scope
    }

    #[inline]
    pub fn add_named_import(
        &mut self,
        scope: ScopeIndex,
        segments: Vec<Symbol>,
        alias: Option<Symbol>,
    ) {
        let path = QualifiedPath::from_segments(segments);
        self[scope].named_imports.push((path, alias));
    }

    #[inline]
    pub fn add_glob_import(&mut self, scope: ScopeIndex, segments: Vec<Symbol>) {
        let path = QualifiedPath::from_segments(segments);
        self[scope].glob_imports.push(path);
    }

    #[inline]
    pub fn add_symbol(
        &mut self,
        scope: ScopeIndex,
        namespace: Namespace,
        symbol: Symbol,
        binding: Option<ScopeIndex>,
    ) {
        let hir_id = hir::HirId::new();
        if let Some(_) = self[scope][namespace].insert(symbol, Binding::new((hir_id, binding))) {
            panic!("{:?} already defined: {:?}", namespace, symbol);
        }
    }

    #[inline]
    pub fn find_krate(&self, name: Symbol) -> Option<ModuleIndex> {
        self.krates.get(&name).copied()
    }

    pub fn find_symbol(
        &self,
        scope: ScopeIndex,
        namespace: Namespace,
        symbol: Symbol,
    ) -> Option<Binding> {
        let mut scope = Some(scope);

        while let Some(index) = scope {
            if let Some(binding) = self[index][namespace].get(symbol) {
                return Some(binding.clone());
            }

            scope = self[index].parent;
        }

        None
    }
}

impl Module {
    #[inline]
    fn new(parent: Option<ModuleIndex>, scope: ScopeIndex) -> Self {
        Self {
            parent,
            scope,

            submodules: HashMap::new(),
        }
    }

    #[inline]
    pub fn new_root(scope: ScopeIndex) -> Self {
        Self::new(None, scope)
    }

    #[inline]
    pub fn new_child(parent: ModuleIndex, scope: ScopeIndex) -> Self {
        Self::new(Some(parent), scope)
    }

    #[inline]
    pub const fn parent(&self) -> Option<ModuleIndex> {
        self.parent
    }

    #[inline]
    pub const fn scope(&self) -> ScopeIndex {
        self.scope
    }
}

impl Scope {
    #[inline]
    fn new(
        module: ModuleIndex,
        current: ScopeIndex,
        parent: Option<ScopeIndex>,
        kind: RibKind,
    ) -> Self {
        Self {
            parent,
            module,
            current,

            named_imports: vec![],
            glob_imports: vec![],

            submodules: HashMap::new(),
            types: NameBinding::new(kind),
            variables: NameBinding::new(kind),
        }
    }

    #[inline]
    fn new_root(module: ModuleIndex, current: ScopeIndex) -> Self {
        Self::new(module, current, None, RibKind::CrateRoot)
    }

    #[inline]
    fn new_block(&self, current: ScopeIndex, kind: RibKind) -> Self {
        Self::new(self.module, current, Some(self.current), kind)
    }
}

impl NameBinding {
    #[inline]
    pub fn new(kind: RibKind) -> Self {
        Self {
            kind,
            symbols: HashMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, symbol: Symbol, binding: Binding) -> Option<Binding> {
        self.symbols.insert(symbol, binding)
    }

    #[inline]
    pub fn get(&self, symbol: Symbol) -> Option<&Binding> {
        self.symbols.get(&symbol)
    }
}

impl Binding {
    #[inline]
    const fn new((hir_id, scope): (hir::HirId, Option<ScopeIndex>)) -> Self {
        Self { hir_id, scope }
    }

    #[inline]
    pub const fn hir_id(&self) -> hir::HirId {
        self.hir_id
    }

    #[inline]
    pub const fn scope(&self) -> Option<ScopeIndex> {
        self.scope
    }
}

impl QualifiedPath {
    fn from_segments(segments: Vec<Symbol>) -> Self {
        let mut iter = segments.iter();

        match iter.next() {
            Some(&Symbol::CRATE) => Self {
                kind: QualifiedPathKind::Crate,
                segments: iter.cloned().collect(),
            },
            Some(&Symbol::SUPER) => {
                let mut count = 1;
                while let Some(&Symbol::SUPER) = iter.next() {
                    count += 1;
                }

                Self {
                    kind: QualifiedPathKind::Super(count),
                    segments: segments.iter().skip(count).cloned().collect(),
                }
            }
            _ => Self {
                kind: QualifiedPathKind::Relative,
                segments,
            },
        }
    }
}

impl std::ops::Index<ModuleIndex> for Collector {
    type Output = Module;

    #[inline]
    fn index(&self, index: ModuleIndex) -> &Self::Output {
        &self.modules[index.0]
    }
}

impl std::ops::IndexMut<ModuleIndex> for Collector {
    #[inline]
    fn index_mut(&mut self, index: ModuleIndex) -> &mut Self::Output {
        &mut self.modules[index.0]
    }
}

impl std::ops::Index<ScopeIndex> for Collector {
    type Output = Scope;

    #[inline]
    fn index(&self, index: ScopeIndex) -> &Self::Output {
        &self.scopes[index.0]
    }
}

impl std::ops::IndexMut<ScopeIndex> for Collector {
    #[inline]
    fn index_mut(&mut self, index: ScopeIndex) -> &mut Self::Output {
        &mut self.scopes[index.0]
    }
}

impl std::ops::Index<Namespace> for Scope {
    type Output = NameBinding;

    #[inline]
    fn index(&self, index: Namespace) -> &Self::Output {
        match index {
            Namespace::Type => &self.types,
            Namespace::Value => &self.variables,
        }
    }
}

impl std::ops::IndexMut<Namespace> for Scope {
    #[inline]
    fn index_mut(&mut self, index: Namespace) -> &mut Self::Output {
        match index {
            Namespace::Type => &mut self.types,
            Namespace::Value => &mut self.variables,
        }
    }
}

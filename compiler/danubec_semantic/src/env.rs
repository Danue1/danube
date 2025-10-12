use danubec_symbol::{DefinitionId, FileId, ImportId, ModuleId, ScopeId, Symbol};
use fxhash::FxHashMap;
use slotmap::SlotMap;

#[derive(Debug)]
pub struct Env {
    modules: SlotMap<ModuleId, Module>,
    scopes: SlotMap<ScopeId, Scope>,
    imports: SlotMap<ImportId, Import>,
    definitions: SlotMap<DefinitionId, Definition>,
}

#[derive(Debug)]
pub struct Module {
    pub parent: Option<ModuleId>,
    pub children: FxHashMap<Symbol, ModuleId>,
    pub scope: ScopeId,
    pub file: FileId,
}

#[derive(Debug)]
pub struct Scope {
    pub parent_module: Option<ModuleId>,
    pub parent: Option<ScopeId>,
    pub kind: ScopeKind,
    pub imports: Vec<Import>,
    pub values: FxHashMap<Symbol, Vec<DefinitionId>>,
    pub types: FxHashMap<Symbol, Vec<DefinitionId>>,
    pub implements: Vec<DefinitionId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Module,
    Function,
    Block,
}

#[derive(Debug)]
pub struct Import {
    pub kind: ImportKind,
    pub file: FileId,
}

#[derive(Debug)]
pub enum ImportKind {
    Path(Path, ImportNestedKind),
    List(Vec<ImportKind>),
}

#[derive(Debug)]
pub enum ImportNestedKind {
    Glob,
    Identifier(Option<Symbol>),
    List(Vec<ImportKind>),
}

#[derive(Debug)]
pub struct Path {
    pub segments: Vec<PathSegment>,
}

#[derive(Debug)]
pub struct PathSegment {
    pub name: Symbol,
    pub alias: Option<Symbol>,
}

#[derive(Debug)]
pub struct Definition {
    pub parent_scope: Option<ScopeId>,
    pub file: FileId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Namespace {
    Value,
    Type,
}

impl Env {
    pub fn new() -> Self {
        Self {
            modules: SlotMap::with_key(),
            scopes: SlotMap::with_key(),
            imports: SlotMap::with_key(),
            definitions: SlotMap::with_key(),
        }
    }

    pub fn scope<F>(&mut self, kind: ScopeKind, f: F) -> ScopeId
    where
        F: FnOnce(&mut Scope),
    {
        let id = self.scopes.insert(Scope::new(kind));
        f(&mut self.scopes[id]);
        id
    }

    pub fn definition(&mut self, definition: Definition) -> DefinitionId {
        self.definitions.insert(definition)
    }

    pub fn import(&mut self, import: Import) -> ImportId {
        self.imports.insert(import)
    }

    pub fn module(&mut self, file: FileId, parent: Option<ModuleId>) -> ModuleId {
        let scope = self.scope(ScopeKind::Module, |s| {
            s.parent_module = parent;
        });
        self.modules.insert(Module {
            parent,
            scope,
            children: FxHashMap::default(),
            file,
        })
    }
}

impl Scope {
    pub fn new(kind: ScopeKind) -> Self {
        Self {
            parent_module: None,
            parent: None,
            kind,
            imports: vec![],
            values: FxHashMap::default(),
            types: FxHashMap::default(),
            implements: vec![],
        }
    }

    #[inline]
    pub fn with_parent_module(mut self, parent: ModuleId) -> Self {
        self.parent_module = Some(parent);
        self
    }

    #[inline]
    pub fn with_parent(mut self, parent: ScopeId) -> Self {
        self.parent = Some(parent);
        self
    }
}

impl std::ops::Index<ModuleId> for Env {
    type Output = Module;

    #[inline]
    fn index(&self, index: ModuleId) -> &Self::Output {
        &self.modules[index]
    }
}

impl std::ops::IndexMut<ModuleId> for Env {
    #[inline]
    fn index_mut(&mut self, index: ModuleId) -> &mut Self::Output {
        &mut self.modules[index]
    }
}

impl std::ops::Index<ScopeId> for Env {
    type Output = Scope;

    #[inline]
    fn index(&self, index: ScopeId) -> &Self::Output {
        &self.scopes[index]
    }
}

impl std::ops::IndexMut<ScopeId> for Env {
    #[inline]
    fn index_mut(&mut self, index: ScopeId) -> &mut Self::Output {
        &mut self.scopes[index]
    }
}

impl std::ops::Index<DefinitionId> for Env {
    type Output = Definition;

    #[inline]
    fn index(&self, index: DefinitionId) -> &Self::Output {
        &self.definitions[index]
    }
}

impl std::ops::Index<ImportId> for Env {
    type Output = Import;

    #[inline]
    fn index(&self, index: ImportId) -> &Self::Output {
        &self.imports[index]
    }
}

impl std::ops::Index<Symbol> for Module {
    type Output = ModuleId;

    #[inline]
    fn index(&self, index: Symbol) -> &Self::Output {
        &self.children[&index]
    }
}

impl std::ops::Index<Namespace> for Scope {
    type Output = FxHashMap<Symbol, Vec<DefinitionId>>;

    #[inline]
    fn index(&self, namespace: Namespace) -> &Self::Output {
        match namespace {
            Namespace::Value => &self.values,
            Namespace::Type => &self.types,
        }
    }
}

impl std::ops::IndexMut<Namespace> for Scope {
    #[inline]
    fn index_mut(&mut self, index: Namespace) -> &mut Self::Output {
        match index {
            Namespace::Value => &mut self.values,
            Namespace::Type => &mut self.types,
        }
    }
}

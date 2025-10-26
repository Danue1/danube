use danubec_hir::{Attribute, Binding, Import, ImportKind, Path, PathSegment, Visibility};
use danubec_symbol::{AttributeId, DefinitionId, FileId, ImplementId, ModuleId, ScopeId, Symbol};
use fxhash::FxHashMap;
use slotmap::SlotMap;

#[derive(Debug)]
pub struct Env {
    modules: SlotMap<ModuleId, Module>,
    scopes: SlotMap<ScopeId, Scope>,
    attributes: SlotMap<AttributeId, Attribute>,
    definitions: SlotMap<DefinitionId, Definition>,
    implements: SlotMap<ImplementId, Implement>,
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
    pub module: Option<ModuleId>,
    pub parent: Option<ScopeId>,
    pub kind: ScopeKind,
    pub types: FxHashMap<Symbol, Vec<DefinitionId>>,
    pub values: FxHashMap<Symbol, Vec<DefinitionId>>,
    pub imports: Vec<Import>,
    pub implements: Vec<ImplementId>,
}

#[derive(Debug)]
pub struct Definition {
    pub scope: ScopeId,
    pub definition: danubec_hir::Definition,
    pub file: FileId,
}

#[derive(Debug)]
pub struct Implement {
    pub scope: ScopeId,
    pub implement: danubec_hir::Implement,
    pub file: FileId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Module,
    Function,
    Block,
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
            attributes: SlotMap::with_key(),
            definitions: SlotMap::with_key(),
            implements: SlotMap::with_key(),
        }
    }

    pub fn module(&mut self, file: FileId, parent: Option<ModuleId>) -> ModuleId {
        let scope = self.scope(ScopeKind::Module, |s| {
            s.module = parent;
        });
        self.modules.insert(Module {
            parent,
            scope,
            children: FxHashMap::default(),
            file,
        })
    }

    pub fn scope<F>(&mut self, kind: ScopeKind, f: F) -> ScopeId
    where
        F: FnOnce(&mut Scope),
    {
        let id = self.scopes.insert(Scope::new(kind));
        f(&mut self.scopes[id]);
        id
    }

    pub fn attribute(&mut self, attribute: Attribute) -> AttributeId {
        self.attributes.insert(attribute)
    }

    pub fn definition(&mut self, definition: Definition) -> DefinitionId {
        self.definitions.insert(definition)
    }

    pub fn implement(&mut self, implement: Implement) -> ImplementId {
        self.implements.insert(implement)
    }
}

impl Scope {
    pub fn new(kind: ScopeKind) -> Self {
        Self {
            module: None,
            parent: None,
            kind,
            types: FxHashMap::default(),
            values: FxHashMap::default(),
            imports: vec![],
            implements: vec![],
        }
    }

    #[inline]
    pub fn with_parent_module(mut self, parent: ModuleId) -> Self {
        self.module = Some(parent);
        self
    }

    #[inline]
    pub fn with_parent(mut self, parent: ScopeId) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn import(
        &mut self,
        attributes: &[AttributeId],
        visibility: &Visibility,
        segments: &[PathSegment],
        kind: ImportKind,
    ) {
        if !segments.is_empty() {
            let path = Path {
                segments: segments.to_vec(),
                binding: Binding::Unresolved,
            };
            let import = Import {
                attributes: attributes.to_vec(),
                visibility: visibility.clone(),
                path,
                kind,
            };
            self.imports.push(import);
        }
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

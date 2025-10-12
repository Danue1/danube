use crate::{file_system::FileId, symbol::Symbol};
use std::collections::HashMap;

slotmap::new_key_type! {
    pub struct ModuleId;

    pub struct ScopeId;

    pub struct ImportId;

    pub struct DefinitionId;
}

#[derive(Debug)]
pub struct Table {
    modules: slotmap::SlotMap<ModuleId, Module>,
    scopes: slotmap::SlotMap<ScopeId, Scope>,
    imports: slotmap::SlotMap<ImportId, Import>,
    definitions: slotmap::SlotMap<DefinitionId, Definition>,
}

#[derive(Debug)]
pub struct Module {
    pub parent: Option<ModuleId>,
    pub scope: ScopeId,
    children: HashMap<Symbol, ModuleId>,
    pub imports: Vec<Import>,
    pub file: FileId,
}

#[derive(Debug)]
pub struct Scope {
    pub parent: Option<ScopeId>,
    pub kind: ScopeKind,
    pub values: HashMap<Symbol, DefinitionId>,
    pub types: HashMap<Symbol, DefinitionId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Module,
    Function,
    Block,
}

#[derive(Debug)]
pub struct Import {
    pub parent_scope: ScopeId,
    pub kind: ImportKind,
    pub file: FileId,
}

#[derive(Debug)]
pub enum ImportKind {
    Single {
        path: Vec<Symbol>,
        alias: Option<Symbol>,
    },
    Glob {
        path: Vec<Symbol>,
    },
    Group {
        base: Vec<Symbol>,
        members: Vec<ImportKind>,
    },
}

#[derive(Debug)]
pub struct Definition {
    pub parent_scope: ScopeId,
    pub name: Symbol,
    pub namespace: Namespace,
    pub kind: DefinitionKind,
    pub file: FileId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Namespace {
    Value,
    Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefinitionKind {
    Module,
    Function,
    Struct,
    Enum,
    Const,
    Static,
    Trait,
    EnumVariant,
    TypeAlias,
    Implement,
    Local,
}

impl Table {
    pub fn new() -> Self {
        Self {
            modules: slotmap::SlotMap::with_key(),
            scopes: slotmap::SlotMap::with_key(),
            imports: slotmap::SlotMap::with_key(),
            definitions: slotmap::SlotMap::with_key(),
        }
    }

    pub fn scope(&mut self, parent: Option<ScopeId>, kind: ScopeKind) -> ScopeId {
        self.scopes.insert(Scope {
            parent,
            kind,
            values: HashMap::new(),
            types: HashMap::new(),
        })
    }

    pub fn definition(&mut self, definition: Definition) -> DefinitionId {
        self.definitions.insert(definition)
    }

    pub fn import(&mut self, import: Import) -> ImportId {
        self.imports.insert(import)
    }

    pub fn module(&mut self, file: FileId, parent: Option<ModuleId>) -> ModuleId {
        let scope = self.scope(parent.map(|p| self.modules[p].scope), ScopeKind::Module);
        self.modules.insert(Module {
            parent,
            scope,
            children: HashMap::new(),
            imports: vec![],
            file,
        })
    }
}

impl std::ops::Index<ModuleId> for Table {
    type Output = Module;

    #[inline]
    fn index(&self, index: ModuleId) -> &Self::Output {
        &self.modules[index]
    }
}

impl std::ops::IndexMut<ModuleId> for Table {
    #[inline]
    fn index_mut(&mut self, index: ModuleId) -> &mut Self::Output {
        &mut self.modules[index]
    }
}

impl std::ops::Index<ScopeId> for Table {
    type Output = Scope;

    #[inline]
    fn index(&self, index: ScopeId) -> &Self::Output {
        &self.scopes[index]
    }
}

impl std::ops::IndexMut<ScopeId> for Table {
    #[inline]
    fn index_mut(&mut self, index: ScopeId) -> &mut Self::Output {
        &mut self.scopes[index]
    }
}

impl std::ops::Index<DefinitionId> for Table {
    type Output = Definition;

    #[inline]
    fn index(&self, index: DefinitionId) -> &Self::Output {
        &self.definitions[index]
    }
}

impl std::ops::IndexMut<DefinitionId> for Table {
    #[inline]
    fn index_mut(&mut self, index: DefinitionId) -> &mut Self::Output {
        &mut self.definitions[index]
    }
}

impl std::ops::Index<ImportId> for Table {
    type Output = Import;

    #[inline]
    fn index(&self, index: ImportId) -> &Self::Output {
        &self.imports[index]
    }
}

impl std::ops::IndexMut<ImportId> for Table {
    #[inline]
    fn index_mut(&mut self, index: ImportId) -> &mut Self::Output {
        &mut self.imports[index]
    }
}

impl std::ops::Index<Symbol> for Module {
    type Output = ModuleId;

    #[inline]
    fn index(&self, index: Symbol) -> &Self::Output {
        &self.children[&index]
    }
}

impl std::ops::IndexMut<Symbol> for Module {
    #[inline]
    fn index_mut(&mut self, index: Symbol) -> &mut Self::Output {
        self.children.get_mut(&index).unwrap()
    }
}

impl std::ops::Index<Namespace> for Scope {
    type Output = HashMap<Symbol, DefinitionId>;

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

impl std::ops::Index<Symbol> for HashMap<Symbol, DefinitionId> {
    type Output = DefinitionId;

    #[inline]
    fn index(&self, index: Symbol) -> &Self::Output {
        &self[&index]
    }
}

impl std::ops::IndexMut<Symbol> for HashMap<Symbol, DefinitionId> {
    #[inline]
    fn index_mut(&mut self, index: Symbol) -> &mut Self::Output {
        self.get_mut(&index).unwrap()
    }
}

use crate::{
    file_system::FileId,
    symbol::{Symbol, SymbolInterner},
    table::{ModuleId, Namespace, ScopeId, ScopeKind, Table},
};
use danubec_diagnostic::Diagnostic;
use danubec_syntax::{
    AstNode, ConstantDefinition, Definition, DefinitionKind, EnumDefinition, FunctionDefinition,
    Identifier, ImplementDefinition, ModuleDefinition, ModuleDefinitionKind, StaticDefinition,
    StructDefinition, SyntaxNode, TraitDefinition, TypeDefinition, UseDefinition, UseTree,
};

pub fn collect(
    diagnostic: &mut Diagnostic,
    file: FileId,
    module: ModuleId,
    symbols: &mut SymbolInterner,
    table: &mut Table,
    node: SyntaxNode,
) {
    let mut collector = DefinitionCollector::new(diagnostic, file, module, symbols, table);
    collector.root(node);
}

struct DefinitionCollector<'lowering> {
    diagnostic: &'lowering mut Diagnostic,
    module: ModuleId,
    symbols: &'lowering mut SymbolInterner,
    table: &'lowering mut Table,
    scopes: Vec<ScopeId>,
    file: FileId,
}

impl<'lowering> DefinitionCollector<'lowering> {
    fn new(
        diagnostic: &'lowering mut Diagnostic,
        file: FileId,
        module: ModuleId,
        symbols: &'lowering mut SymbolInterner,
        table: &'lowering mut Table,
    ) -> Self {
        let scopes = vec![table[module].scope];

        Self {
            diagnostic,
            file,
            module,
            symbols,
            table,
            scopes,
        }
    }

    fn with_scope<F>(&mut self, kind: ScopeKind, f: F)
    where
        F: FnOnce(&mut Self),
    {
        let scope = self.enter_scope(kind);
        f(self);
        assert_eq!(self.current_scope(), scope);
        self.exit_scope();
    }

    fn current_scope(&self) -> ScopeId {
        self.scopes.last().copied().unwrap()
    }

    fn enter_scope(&mut self, kind: ScopeKind) -> ScopeId {
        let parent = self.current_scope();
        let scope = self.table.scope(Some(parent), kind);
        self.scopes.push(scope);
        scope
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn root(&mut self, node: SyntaxNode) {
        for definition in node.children().filter_map(Definition::cast) {
            self.definition(definition);
        }
    }

    fn definition(&mut self, node: Definition) {
        match node.kind() {
            Some(DefinitionKind::Function(node)) => self.function_definition(node),
            Some(DefinitionKind::Struct(node)) => self.struct_definition(node),
            Some(DefinitionKind::Enum(node)) => self.enum_definition(node),
            Some(DefinitionKind::Use(node)) => self.use_definition(node),
            Some(DefinitionKind::Module(node)) => self.module_definition(node),
            Some(DefinitionKind::Trait(node)) => self.trait_definition(node),
            Some(DefinitionKind::Constant(node)) => self.constant_definition(node),
            Some(DefinitionKind::Static(node)) => self.static_definition(node),
            Some(DefinitionKind::Type(node)) => self.type_definition(node),
            Some(DefinitionKind::Implement(node)) => self.implement_definition(node),
            None => {
                //
            }
        }
    }

    fn function_definition(&mut self, node: FunctionDefinition) {
        let Some(name) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Function without a name"));
        };

        std::todo!();
    }

    fn struct_definition(&mut self, node: StructDefinition) {
        let Some(name) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Struct without a name"));
        };

        std::todo!();
    }

    fn enum_definition(&mut self, node: EnumDefinition) {
        let Some(name) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Enum without a name"));
        };

        std::todo!();
    }

    fn use_definition(&mut self, node: UseDefinition) {
        std::todo!();
    }

    fn module_definition(&mut self, node: ModuleDefinition) {
        let inline_module = match node.kind() {
            Some(ModuleDefinitionKind::Inline(inline)) => inline,
            _ => return,
        };

        let Some(name) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Module without a name"));
        };

        let definition = self.table.definition(crate::table::Definition {
            parent_scope: self.scopes.last().copied().unwrap(),
            name,
            namespace: Namespace::Type,
            kind: crate::table::DefinitionKind::Module,
            file: self.file,
        });
        let current = self.current_scope();
        self.table[current][Namespace::Type][name] = definition;

        let parent = self.module;
        let child = self.table.module(self.file, Some(parent));
        self.table[parent][name] = child;

        self.with_scope(ScopeKind::Module, |this| {
            for definition in inline_module.definitions() {
                this.definition(definition);
            }
        });
    }

    fn trait_definition(&mut self, node: TraitDefinition) {
        let Some(name) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Trait without a name"));
        };

        std::todo!();
    }

    fn constant_definition(&mut self, node: ConstantDefinition) {
        let Some(name) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Constant without a name"));
        };

        std::todo!();
    }

    fn static_definition(&mut self, node: StaticDefinition) {
        let Some(name) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Static without a name"));
        };

        std::todo!();
    }

    fn type_definition(&mut self, node: TypeDefinition) {
        let Some(name) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Type without a name"));
        };

        std::todo!();
    }

    fn implement_definition(&mut self, node: ImplementDefinition) {
        std::todo!();
    }

    fn identifier(&mut self, node: Identifier) -> Option<Symbol> {
        node.segment()
            .and_then(|s| s.identifier())
            .map(|name| self.symbols.intern(name.text()))
    }
}

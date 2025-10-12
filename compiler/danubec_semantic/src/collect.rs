use crate::{
    env::{Env, Namespace, ScopeKind},
    fs::Fs,
};
use danubec_ast::{
    ConstantDefinition, Definition, DefinitionKind, EnumDefinition, FunctionDefinition, Identifier,
    ImplementDefinition, ModuleDefinition, ModuleDefinitionKind, StaticDefinition,
    StructDefinition, TraitDefinition, TypeDefinition, UseDefinition,
};
use danubec_diagnostic::Diagnostic;
use danubec_parse::parse;
use danubec_symbol::{FileId, ModuleId, ScopeId, Symbol, SymbolInterner};
use danubec_syntax::{AstNode, SyntaxNode};
use std::collections::{HashSet, VecDeque};

pub fn collect(
    fs: &mut Fs,
    env: &mut Env,
    symbols: &mut SymbolInterner,
    diagnostic: &mut Diagnostic,
    root: FileId,
) {
    let root = env.module(root, None);

    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(module) = queue.pop_front() {
        let file = env[module].file;
        let Some(source) = fs.source(file) else {
            diagnostic.report(miette!("File not found: {:?}", fs.path(file)));
            continue;
        };
        let node = parse(&source, diagnostic);

        {
            let mut collector = DefinitionCollector::new(file, module, env, symbols, diagnostic);
            collector.root(node.clone());
        }

        for name in external_modules(&node, symbols) {
            let Some(child_file) = fs.module(file, &symbols[name]) else {
                diagnostic.report(miette!("Module '{}' not found", &symbols[name]));
                continue;
            };

            let definition = env.definition(crate::env::Definition {
                parent_scope: Some(env[module].scope),
                file,
            });
            let scope = env[module].scope;
            env[scope][Namespace::Type]
                .entry(name)
                .or_default()
                .push(definition);

            let parent = module;
            let child = env.module(child_file, Some(parent));
            env[parent].children.insert(name, child);

            queue.push_back(child);
        }
    }
}

fn external_modules(node: &SyntaxNode, symbols: &mut SymbolInterner) -> HashSet<Symbol> {
    node.children()
        .filter_map(Definition::cast)
        .filter_map(|d| match d.kind() {
            Some(DefinitionKind::Module(m))
                if matches!(m.kind(), Some(ModuleDefinitionKind::External(_))) =>
            {
                m.name()
                    .and_then(|n| n.segment())
                    .and_then(|s| s.identifier())
                    .map(|name| symbols.intern(name.text()))
            }
            _ => None,
        })
        .collect()
}

struct DefinitionCollector<'lowering> {
    file: FileId,
    module: ModuleId,
    scopes: Vec<ScopeId>,
    env: &'lowering mut Env,
    symbols: &'lowering mut SymbolInterner,
    diagnostic: &'lowering mut Diagnostic,
}

impl<'lowering> DefinitionCollector<'lowering> {
    fn new(
        file: FileId,
        module: ModuleId,
        env: &'lowering mut Env,
        symbols: &'lowering mut SymbolInterner,
        diagnostic: &'lowering mut Diagnostic,
    ) -> Self {
        let scopes = vec![env[module].scope];

        Self {
            file,
            module,
            scopes,
            env,
            symbols,
            diagnostic,
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
        let scope = self.env.scope(kind, |s| {
            s.parent = Some(parent);
        });
        self.scopes.push(scope);
        scope
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn root(&mut self, node: SyntaxNode) {
        self.with_scope(ScopeKind::Module, |this| {
            for definition in node.children().filter_map(Definition::cast) {
                this.definition(definition);
            }
        });
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
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Function without a name"));
        };

        std::todo!();
    }

    fn struct_definition(&mut self, node: StructDefinition) {
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Struct without a name"));
        };

        std::todo!();
    }

    fn enum_definition(&mut self, node: EnumDefinition) {
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Enum without a name"));
        };

        std::todo!();
    }

    fn use_definition(&mut self, _: UseDefinition) {
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

        let definition = self.env.definition(crate::env::Definition {
            parent_scope: self.scopes.last().copied(),
            file: self.file,
        });
        let current_scope = self.current_scope();
        self.env[current_scope][Namespace::Type]
            .entry(name)
            .or_default()
            .push(definition);

        let parent_module = self.module;
        let child = self.env.module(self.file, Some(parent_module));
        self.env[parent_module].children.insert(name, child);

        self.with_scope(ScopeKind::Module, |this| {
            for definition in inline_module.definitions() {
                this.definition(definition);
            }
        });
    }

    fn trait_definition(&mut self, node: TraitDefinition) {
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Trait without a name"));
        };

        std::todo!();
    }

    fn constant_definition(&mut self, node: ConstantDefinition) {
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Constant without a name"));
        };

        std::todo!();
    }

    fn static_definition(&mut self, node: StaticDefinition) {
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Static without a name"));
        };

        std::todo!();
    }

    fn type_definition(&mut self, node: TypeDefinition) {
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Type without a name"));
        };

        std::todo!();
    }

    fn implement_definition(&mut self, _: ImplementDefinition) {
        std::todo!();
    }

    fn identifier(&mut self, node: Identifier) -> Option<Symbol> {
        node.segment()
            .and_then(|s| s.identifier())
            .map(|name| self.symbols.intern(name.text()))
    }
}

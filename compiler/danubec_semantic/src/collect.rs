use crate::{
    env::{Env, Namespace, ScopeKind},
    fs::Fs,
};
use danubec_diagnostic::Diagnostic;
use danubec_hir::{ImportKind, PathSegment, PathSegmentKind};
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
        .filter_map(danubec_ast::Definition::cast)
        .filter_map(|d| match d.kind() {
            Some(danubec_ast::DefinitionKind::Module(m))
                if matches!(
                    m.kind(),
                    Some(danubec_ast::ModuleDefinitionKind::External(_))
                ) =>
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
}

impl<'lowering> DefinitionCollector<'lowering> {
    pub fn root(&mut self, node: SyntaxNode) {
        self.with_scope(ScopeKind::Module, |this| {
            for attribute in node
                .children()
                .filter_map(danubec_ast::TopLevelAttribute::cast)
            {
                this.top_level_attribute(attribute);
            }

            for definition in node.children().filter_map(danubec_ast::Definition::cast) {
                this.definition(definition);
            }
        });
    }

    fn top_level_attribute(&mut self, _: danubec_ast::TopLevelAttribute) {
        //
    }

    fn definition(&mut self, node: danubec_ast::Definition) {
        match node.kind() {
            Some(danubec_ast::DefinitionKind::Function(node)) => self.function_definition(node),
            Some(danubec_ast::DefinitionKind::Struct(node)) => self.struct_definition(node),
            Some(danubec_ast::DefinitionKind::Enum(node)) => self.enum_definition(node),
            Some(danubec_ast::DefinitionKind::Use(node)) => self.use_definition(node),
            Some(danubec_ast::DefinitionKind::Module(node)) => self.module_definition(node),
            Some(danubec_ast::DefinitionKind::Trait(node)) => self.trait_definition(node),
            Some(danubec_ast::DefinitionKind::Constant(node)) => self.constant_definition(node),
            Some(danubec_ast::DefinitionKind::Static(node)) => self.static_definition(node),
            Some(danubec_ast::DefinitionKind::Type(node)) => self.type_definition(node),
            Some(danubec_ast::DefinitionKind::Implement(node)) => self.implement_definition(node),
            None => {
                //
            }
        }
    }

    fn function_definition(&mut self, node: danubec_ast::FunctionDefinition) {
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Function without a name"));
        };

        std::todo!();
    }

    fn struct_definition(&mut self, node: danubec_ast::StructDefinition) {
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Struct without a name"));
        };

        std::todo!();
    }

    fn enum_definition(&mut self, node: danubec_ast::EnumDefinition) {
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Enum without a name"));
        };

        std::todo!();
    }

    fn use_definition(&mut self, node: danubec_ast::UseDefinition) {
        let Some(tree) = node.tree() else {
            return self.diagnostic.report(miette!("Use without a tree"));
        };
        let scope = self.current_scope();
        self.use_tree(tree, scope, &[]);
    }

    fn module_definition(&mut self, node: danubec_ast::ModuleDefinition) {
        let inline_module = match node.kind() {
            Some(danubec_ast::ModuleDefinitionKind::Inline(inline)) => inline,
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

    fn trait_definition(&mut self, node: danubec_ast::TraitDefinition) {
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Trait without a name"));
        };

        std::todo!();
    }

    fn constant_definition(&mut self, node: danubec_ast::ConstantDefinition) {
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Constant without a name"));
        };

        std::todo!();
    }

    fn static_definition(&mut self, node: danubec_ast::StaticDefinition) {
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Static without a name"));
        };

        std::todo!();
    }

    fn type_definition(&mut self, node: danubec_ast::TypeDefinition) {
        let Some(_) = node.name().and_then(|n| self.identifier(n)) else {
            return self.diagnostic.report(miette!("Type without a name"));
        };

        std::todo!();
    }

    fn implement_definition(&mut self, _: danubec_ast::ImplementDefinition) {
        std::todo!();
    }

    fn use_tree(&mut self, node: danubec_ast::UseTree, scope: ScopeId, segments: &[PathSegment]) {
        let Some(kind) = node.kind() else {
            return self.diagnostic.report(miette!("Use tree without a kind"));
        };

        match kind {
            danubec_ast::UseTreeKind::Glob(_) => {
                if segments.is_empty() {
                    self.diagnostic.report(miette!("Use glob without a path"));
                    return;
                }
                self.env[scope].import(segments, ImportKind::Glob);
            }
            danubec_ast::UseTreeKind::Element(element) => {
                let Some(path) = element.path() else {
                    self.diagnostic
                        .report(miette!("Use element without a path"));
                    return;
                };
                let Some(tail) = self.path(path) else {
                    self.diagnostic
                        .report(miette!("Use element with invalid path"));
                    return;
                };
                let segments = [segments, &tail].concat();
                match element.trailing() {
                    Some(trailing) => self.use_tree_trailing(trailing, scope, &segments),
                    None => self.env[scope].import(&segments, ImportKind::Symbol(None)),
                }
            }
            danubec_ast::UseTreeKind::List(list) => {
                for tree in list.trees() {
                    self.use_tree(tree, scope, &segments);
                }
            }
        }
    }

    fn use_tree_trailing(
        &mut self,
        node: danubec_ast::UseTreeTrailing,
        scope: ScopeId,
        segments: &[PathSegment],
    ) {
        match node {
            danubec_ast::UseTreeTrailing::Glob(_) => {
                if segments.is_empty() {
                    self.diagnostic.report(miette!("Use glob without a path"));
                    return;
                }
                self.env[scope].import(segments, ImportKind::Glob);
            }
            danubec_ast::UseTreeTrailing::Rename(element) => {
                let Some(name) = element.identifier() else {
                    self.diagnostic.report(miette!("Use rename without a name"));
                    return;
                };
                let Some(name) = self.identifier(name) else {
                    self.diagnostic
                        .report(miette!("Use rename with invalid name"));
                    return;
                };
                self.env[scope].import(segments, ImportKind::Symbol(Some(name)));
            }
            danubec_ast::UseTreeTrailing::Nested(nested) => {
                for tree in nested.trees() {
                    self.use_tree(tree, scope, &segments);
                }
            }
        }
    }

    fn path(&mut self, node: danubec_ast::Path) -> Option<Vec<PathSegment>> {
        let mut segments = vec![];
        for segment in node.segments() {
            let Some(segment) = self.path_segment(segment) else {
                return None;
            };
            segments.push(segment);
        }

        Some(segments)
    }

    fn path_segment(&mut self, node: danubec_ast::PathSegment) -> Option<PathSegment> {
        let kind = match node {
            danubec_ast::PathSegment::Krate(_) => PathSegmentKind::Krate,
            danubec_ast::PathSegment::Self_(_) => PathSegmentKind::Self_,
            danubec_ast::PathSegment::Super_(_) => PathSegmentKind::Super_,
            danubec_ast::PathSegment::Root(_) => PathSegmentKind::Root,
            danubec_ast::PathSegment::Identifier(ident) => {
                let Some(ident) = ident.identifier() else {
                    self.diagnostic
                        .report(miette!("Path segment without identifier"));
                    return None;
                };
                let Some(name) = self.identifier(ident) else {
                    self.diagnostic
                        .report(miette!("Path segment with invalid identifier"));
                    return None;
                };
                PathSegmentKind::Identifier(name)
            }
        };

        Some(PathSegment {
            kind,
            binding: danubec_hir::Binding::Unresolved,
        })
    }

    fn identifier(&mut self, node: danubec_ast::Identifier) -> Option<Symbol> {
        node.segment()
            .and_then(|s| s.identifier())
            .map(|name| self.symbols.intern(name.text()))
    }
}

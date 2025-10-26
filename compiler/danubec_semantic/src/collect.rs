use crate::{
    env::{Env, Namespace, ScopeKind},
    fs::Fs,
};
use danubec_ast as ast;
use danubec_diagnostic::Diagnostic;
use danubec_hir as hir;
use danubec_parse::parse;
use danubec_symbol::{
    AttributeId, DefinitionId, FileId, ModuleId, ScopeId, Symbol, SymbolInterner,
};
use danubec_syntax::{AstNode, Span};
use std::collections::{HashMap, VecDeque};

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
        let node = ast::Root::cast(node).unwrap();

        {
            let mut collector = DefinitionCollector::new(file, module, env, symbols, diagnostic);
            collector.root(node.clone());
        }

        for (definition, name) in external_modules(node, symbols) {
            let Some(child_file) = fs.module(file, &symbols[name.symbol]) else {
                diagnostic.report(miette!("Module '{}' not found", &symbols[name.symbol]));
                continue;
            };

            // let visibility = collector.visibility(definition.visibility());

            let definition = env.definition(crate::env::Definition {
                scope: env[module].scope,
                definition: hir::Definition {
                    attributes: vec![],
                    visibility: hir::Visibility::Private,
                    name,
                    kind: hir::DefinitionKind::Module {
                        kind: hir::ModuleDefinitionKind::External,
                    },
                    span: Span::new(definition.syntax()),
                },
                file,
            });
            let scope = env[module].scope;
            env[scope][Namespace::Type]
                .entry(name.symbol)
                .or_default()
                .push(definition);

            let parent = module;
            let child = env.module(child_file, Some(parent));
            env[parent].children.insert(name.symbol, child);

            queue.push_back(child);
        }
    }
}

fn external_modules(
    node: ast::Root,
    symbols: &mut SymbolInterner,
) -> Vec<(ast::Definition, hir::Identifier)> {
    use danubec_syntax::AstNode;

    node.definitions()
        .filter_map(|definition| match definition.kind() {
            Some(ast::DefinitionKind::Module(module))
                if matches!(module.kind(), Some(ast::ModuleDefinitionKind::External(_))) =>
            {
                let name = module.name()?;
                let segment = name.segment()?;
                let identifier = segment.identifier()?;
                let identifier = hir::Identifier {
                    symbol: symbols.intern(identifier.text()),
                    span: Span::new(name.syntax()),
                };

                Some((definition, identifier))
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

    fn with_scope<T, F>(&mut self, kind: ScopeKind, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        let scope = self.enter_scope(kind);
        let result = f(self);
        assert_eq!(self.current_scope(), scope);
        self.exit_scope();

        result
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
    pub fn root(&mut self, node: ast::Root) {
        let _ = self.with_scope(ScopeKind::Module, |this| -> Result<(), ()> {
            for attribute in node.attributes() {
                this.top_level_attribute(attribute);
            }

            for definition in node.definitions() {
                this.definition(definition)?;
            }

            Ok(())
        });
    }

    fn top_level_attribute(&mut self, _: ast::TopLevelAttribute) {
        //
    }

    fn attribute(&mut self, _: ast::Attribute) -> Result<AttributeId, ()> {
        std::todo!();
    }

    fn associated_definition(
        &mut self,
        node: ast::AssociatedDefinition,
    ) -> Result<(Symbol, DefinitionId), ()> {
        let mut attributes = vec![];
        for attribute in node.attributes() {
            attributes.push(self.attribute(attribute)?);
        }

        let visibility = self.visibility(node.visibility());

        let Some(definition) = node.kind() else {
            self.diagnostic
                .report(miette!("Trait definition without a kind"));
            return Err(());
        };
        let (symbol, definition) =
            self.associated_definition_kind(definition, attributes, visibility)?;

        Ok((symbol, definition))
    }

    fn associated_definition_kind(
        &mut self,
        node: ast::AssociatedDefinitionKind,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Result<(Symbol, DefinitionId), ()> {
        match node {
            ast::AssociatedDefinitionKind::Function(node) => {
                self.function_definition(node, attributes, visibility)
            }
            ast::AssociatedDefinitionKind::Constant(node) => {
                self.constant_definition(node, attributes, visibility)
            }
            ast::AssociatedDefinitionKind::Type(node) => {
                self.type_definition(node, attributes, visibility)
            }
        }
    }

    fn definition(&mut self, node: ast::Definition) -> Result<Option<DefinitionId>, ()> {
        let mut attributes = vec![];
        for attribute in node.attributes() {
            attributes.push(self.attribute(attribute)?);
        }

        let visibility = self.visibility(node.visibility());

        let Some(kind) = node.kind() else {
            self.diagnostic.report(miette!("Definition without a kind"));
            return Err(());
        };
        let definition = match kind {
            ast::DefinitionKind::Function(node) => {
                Some(self.function_definition(node, attributes, visibility)?.1)
            }
            ast::DefinitionKind::Struct(node) => {
                Some(self.struct_definition(node, attributes, visibility)?)
            }
            ast::DefinitionKind::Enum(node) => {
                Some(self.enum_definition(node, attributes, visibility)?)
            }
            ast::DefinitionKind::Module(node) => {
                Some(self.module_definition(node, attributes, visibility)?)
            }
            ast::DefinitionKind::Trait(node) => {
                Some(self.trait_definition(node, attributes, visibility)?)
            }
            ast::DefinitionKind::Constant(node) => {
                Some(self.constant_definition(node, attributes, visibility)?.1)
            }
            ast::DefinitionKind::Static(node) => {
                Some(self.static_definition(node, attributes, visibility)?)
            }
            ast::DefinitionKind::Type(node) => {
                Some(self.type_definition(node, attributes, visibility)?.1)
            }
            ast::DefinitionKind::Use(node) => {
                self.use_definition(node, attributes, visibility)?;
                None
            }
            ast::DefinitionKind::Implement(node) => {
                self.implement_definition(node, attributes, visibility)?;
                None
            }
        };

        Ok(definition)
    }

    fn function_definition(
        &mut self,
        node: ast::FunctionDefinition,
        _attributes: Vec<AttributeId>,
        _visibility: hir::Visibility,
    ) -> Result<(Symbol, DefinitionId), ()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Function without a name"));
            return Err(());
        };
        let _ = self.identifier(name)?;

        std::todo!();
    }

    fn struct_definition(
        &mut self,
        node: ast::StructDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Result<DefinitionId, ()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Struct without a name"));
            return Err(());
        };
        let name = self.identifier(name)?;

        let type_parameters = vec![];

        let type_bounds = vec![];

        let Some(body) = node.body() else {
            self.diagnostic.report(miette!("Struct without a body"));
            return Err(());
        };
        let body = self.struct_definition_body(body)?;

        let scope = self.current_scope();
        let definition = self.env.definition(crate::env::Definition {
            scope,
            definition: hir::Definition {
                attributes,
                visibility,
                name,
                kind: hir::DefinitionKind::Struct {
                    type_parameters,
                    type_bounds,
                    body,
                },
                span: Span::new(node.syntax()),
            },
            file: self.file,
        });

        self.env[scope][Namespace::Type]
            .entry(name.symbol)
            .or_default()
            .push(definition);

        Ok(definition)
    }

    fn enum_definition(
        &mut self,
        node: ast::EnumDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Result<DefinitionId, ()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Enum without a name"));
            return Err(());
        };
        let name = self.identifier(name)?;

        let type_parameters = vec![];

        let type_bounds = vec![];

        let variants = self.with_scope(
            ScopeKind::Block,
            |this| -> Result<Vec<hir::EnumVariant>, ()> {
                let mut variants = vec![];
                for variant in node.variants() {
                    variants.push(this.enum_variant(variant)?);
                }

                Ok(variants)
            },
        )?;

        let scope = self.current_scope();
        let definition = self.env.definition(crate::env::Definition {
            scope,
            definition: hir::Definition {
                attributes,
                visibility,
                name,
                kind: hir::DefinitionKind::Enum {
                    type_parameters,
                    type_bounds,
                    variants,
                },
                span: Span::new(node.syntax()),
            },
            file: self.file,
        });

        self.env[scope][Namespace::Type]
            .entry(name.symbol)
            .or_default()
            .push(definition);

        Ok(definition)
    }

    fn use_definition(
        &mut self,
        node: ast::UseDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Result<(), ()> {
        let Some(tree) = node.tree() else {
            self.diagnostic.report(miette!("Use without a tree"));
            return Err(());
        };
        let scope = self.current_scope();
        self.use_tree(tree, scope, &attributes, &visibility, &[])?;

        Ok(())
    }

    fn module_definition(
        &mut self,
        node: ast::ModuleDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Result<DefinitionId, ()> {
        let inline_module = match node.kind() {
            Some(ast::ModuleDefinitionKind::Inline(inline)) => inline,
            _ => return Err(()),
        };

        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Module without a name"));
            return Err(());
        };
        let name = self.identifier(name)?;

        let scope = self.current_scope();
        let definition = self.env.definition(crate::env::Definition {
            scope,
            definition: hir::Definition {
                attributes,
                visibility,
                name,
                kind: hir::DefinitionKind::Module {
                    kind: hir::ModuleDefinitionKind::Inline {
                        definitions: vec![],
                    },
                },
                span: Span::new(node.syntax()),
            },
            file: self.file,
        });
        let current_scope = self.current_scope();
        self.env[current_scope][Namespace::Type]
            .entry(name.symbol)
            .or_default()
            .push(definition);

        let parent_module = self.module;
        let child = self.env.module(self.file, Some(parent_module));
        self.env[parent_module].children.insert(name.symbol, child);

        self.with_scope(ScopeKind::Module, |this| -> Result<(), ()> {
            for definition in inline_module.definitions() {
                this.definition(definition)?;
            }

            Ok(())
        })?;

        Ok(definition)
    }

    fn trait_definition(
        &mut self,
        node: ast::TraitDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Result<DefinitionId, ()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Trait without a name"));
            return Err(());
        };
        let name = self.identifier(name)?;

        let type_parameters = vec![];

        let type_bounds = vec![];

        let definitions = self.with_scope(ScopeKind::Block, |this| {
            let mut definitions = HashMap::new();
            for definition in node.definitions() {
                let (symbol, definition) = this.associated_definition(definition)?;
                definitions
                    .entry(symbol)
                    .or_insert_with(Vec::new)
                    .push(definition);
            }

            Ok(definitions)
        })?;

        let scope = self.current_scope();
        let definition = self.env.definition(crate::env::Definition {
            scope,
            definition: hir::Definition {
                attributes,
                visibility,
                name,
                kind: hir::DefinitionKind::Trait {
                    type_parameters,
                    type_bounds,
                    definitions,
                },
                span: Span::new(node.syntax()),
            },
            file: self.file,
        });
        self.env[scope][Namespace::Type]
            .entry(name.symbol)
            .or_default()
            .push(definition);

        Ok(definition)
    }

    fn constant_definition(
        &mut self,
        node: ast::ConstantDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Result<(Symbol, DefinitionId), ()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Constant without a name"));
            return Err(());
        };
        let name = self.identifier(name)?;

        let r#type = match node.r#type() {
            Some(r#type) => Some(self.type_expression(r#type, false)?),
            None => None,
        };

        let initializer = match node.initializer() {
            Some(initializer) => Some(self.expression(initializer)?),
            None => None,
        };

        let scope = self.current_scope();
        let definition = self.env.definition(crate::env::Definition {
            scope,
            definition: hir::Definition {
                attributes,
                visibility,
                name,
                kind: hir::DefinitionKind::Constant {
                    r#type,
                    initializer,
                },
                span: Span::new(node.syntax()),
            },
            file: self.file,
        });
        self.env[scope][Namespace::Value]
            .entry(name.symbol)
            .or_default()
            .push(definition);

        Ok((name.symbol, definition))
    }

    fn static_definition(
        &mut self,
        node: ast::StaticDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Result<DefinitionId, ()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Static without a name"));
            return Err(());
        };
        let name = self.identifier(name)?;

        let Some(r#type) = node.r#type() else {
            self.diagnostic.report(miette!("Static without a type"));
            return Err(());
        };
        let r#type = self.type_expression(r#type, false)?;

        let Some(initializer) = node.initializer() else {
            self.diagnostic
                .report(miette!("Static without an initializer"));
            return Err(());
        };
        let initializer = self.expression(initializer)?;

        let scope = self.current_scope();
        let definition = self.env.definition(crate::env::Definition {
            scope,
            definition: hir::Definition {
                attributes,
                visibility,
                name,
                kind: hir::DefinitionKind::Static {
                    r#type,
                    initializer,
                },
                span: Span::new(node.syntax()),
            },
            file: self.file,
        });
        self.env[scope][Namespace::Value]
            .entry(name.symbol)
            .or_default()
            .push(definition);

        Ok(definition)
    }

    fn type_definition(
        &mut self,
        node: ast::TypeDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Result<(Symbol, DefinitionId), ()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Type without a name"));
            return Err(());
        };
        let name = self.identifier(name)?;

        let type_parameters = vec![];

        let type_bounds = vec![];

        let initializer = match node.initializer() {
            Some(initializer) => Some(self.type_expression(initializer, false)?),
            None => None,
        };

        let scope = self.current_scope();
        let definition = self.env.definition(crate::env::Definition {
            scope,
            definition: hir::Definition {
                attributes,
                visibility,
                name,
                kind: hir::DefinitionKind::Type {
                    type_parameters,
                    type_bounds,
                    initializer,
                },
                span: Span::new(node.syntax()),
            },
            file: self.file,
        });
        self.env[scope][Namespace::Value]
            .entry(name.symbol)
            .or_default()
            .push(definition);

        Ok((name.symbol, definition))
    }

    fn implement_definition(
        &mut self,
        node: ast::ImplementDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Result<(), ()> {
        let trait_type = match node.trait_type() {
            None => None,
            Some(trait_type) => Some(self.type_expression(trait_type, false)?),
        };

        let Some(for_type) = node.target_type() else {
            self.diagnostic
                .report(miette!("Implement without a for type"));
            return Err(());
        };
        let for_type = self.type_expression(for_type, false)?;

        let type_parameters = vec![];

        let type_bounds = vec![];

        let definitions = self.with_scope(ScopeKind::Block, |this| {
            let mut definitions = HashMap::new();
            for definition in node.definitions() {
                let (symbol, definition) = this.associated_definition(definition)?;
                definitions
                    .entry(symbol)
                    .or_insert_with(Vec::new)
                    .push(definition);
            }

            Ok(definitions)
        })?;

        self.env.implement(crate::env::Implement {
            scope: self.current_scope(),
            implement: hir::Implement {
                attributes,
                visibility,
                trait_type,
                for_type,
                type_parameters,
                type_bounds,
                definitions,
                span: Span::new(node.syntax()),
            },
            file: self.file,
        });

        Ok(())
    }

    fn use_tree(
        &mut self,
        node: ast::UseTree,
        scope: ScopeId,
        attributes: &[AttributeId],
        visibility: &hir::Visibility,
        segments: &[hir::PathSegment],
    ) -> Result<(), ()> {
        let Some(kind) = node.kind() else {
            self.diagnostic.report(miette!("Use tree without a kind"));
            return Err(());
        };

        match kind {
            ast::UseTreeKind::Glob(_) => {
                if segments.is_empty() {
                    self.diagnostic.report(miette!("Use glob without a path"));
                    return Err(());
                }

                self.env[scope].import(attributes, visibility, segments, hir::ImportKind::Glob);
            }
            ast::UseTreeKind::Element(element) => {
                let Some(path) = element.path() else {
                    self.diagnostic
                        .report(miette!("Use element without a path"));
                    return Err(());
                };
                let tail = self.path(path)?;
                let segments = [segments, &tail].concat();

                match element.trailing() {
                    Some(trailing) => {
                        self.use_tree_trailing(trailing, scope, attributes, visibility, &segments)?
                    }
                    None => self.env[scope].import(
                        attributes,
                        visibility,
                        &segments,
                        hir::ImportKind::Symbol(None),
                    ),
                }
            }
            ast::UseTreeKind::List(list) => {
                for tree in list.trees() {
                    self.use_tree(tree, scope, attributes, visibility, &segments)?;
                }
            }
        }

        Ok(())
    }

    fn use_tree_trailing(
        &mut self,
        node: ast::UseTreeTrailing,
        scope: ScopeId,
        attributes: &[AttributeId],
        visibility: &hir::Visibility,
        segments: &[hir::PathSegment],
    ) -> Result<(), ()> {
        match node {
            ast::UseTreeTrailing::Glob(_) => {
                if segments.is_empty() {
                    self.diagnostic.report(miette!("Use glob without a path"));
                    return Err(());
                }

                self.env[scope].import(attributes, visibility, segments, hir::ImportKind::Glob);
            }
            ast::UseTreeTrailing::Rename(element) => {
                let Some(name) = element.identifier() else {
                    self.diagnostic.report(miette!("Use rename without a name"));
                    return Err(());
                };
                let name = self.identifier(name)?;

                self.env[scope].import(
                    attributes,
                    visibility,
                    segments,
                    hir::ImportKind::Symbol(Some(name)),
                );
            }
            ast::UseTreeTrailing::Nested(nested) => {
                for tree in nested.trees() {
                    self.use_tree(tree, scope, &attributes, &visibility, &segments)?;
                }
            }
        }

        Ok(())
    }

    fn struct_definition_body(&mut self, node: ast::StructBody) -> Result<hir::StructBody, ()> {
        match node {
            ast::StructBody::Unit(_) => Ok(hir::StructBody::Unit),
            ast::StructBody::Named(record) => {
                let mut fields = vec![];
                for field in record.fields() {
                    let visibility = self.visibility(field.visibility());

                    let Some(name) = field.name() else {
                        self.diagnostic
                            .report(miette!("Struct field without a name"));
                        return Err(());
                    };
                    let name = self.identifier(name)?;

                    let Some(ty) = field.r#type() else {
                        self.diagnostic
                            .report(miette!("Struct field without a type"));
                        return Err(());
                    };
                    let ty = self.type_expression(ty, false)?;

                    fields.push((visibility, name, ty));
                }

                Ok(hir::StructBody::Named(fields))
            }
            ast::StructBody::Unnamed(unnamed) => {
                let mut fields = vec![];
                for field in unnamed.fields() {
                    let visibility = self.visibility(field.visibility());

                    let Some(ty) = field.r#type() else {
                        self.diagnostic
                            .report(miette!("Tuple struct field without a type"));
                        return Err(());
                    };
                    let ty = self.type_expression(ty, false)?;

                    fields.push((visibility, ty));
                }
                Ok(hir::StructBody::Unnamed(fields))
            }
        }
    }

    fn enum_variant(&mut self, node: ast::EnumVariant) -> Result<hir::EnumVariant, ()> {
        let span = Span::new(node.syntax());
        let (attributes, name, kind) = match node {
            ast::EnumVariant::Unit(node) => {
                let mut attributes = vec![];
                for attribute in node.attributes() {
                    attributes.push(self.attribute(attribute)?);
                }

                let Some(name) = node.name() else {
                    self.diagnostic
                        .report(miette!("Enum variant without a name"));
                    return Err(());
                };
                let name = self.identifier(name)?;

                (attributes, name, hir::EnumVariantKind::Unit)
            }
            ast::EnumVariant::Scalar(node) => {
                let mut attributes = vec![];
                for attribute in node.attributes() {
                    attributes.push(self.attribute(attribute)?);
                }

                let Some(name) = node.name() else {
                    self.diagnostic
                        .report(miette!("Enum variant without a name"));
                    return Err(());
                };
                let name = self.identifier(name)?;

                let Some(initializer) = node.initializer() else {
                    self.diagnostic
                        .report(miette!("Enum variant scalar without a value"));
                    return Err(());
                };
                let initializer = self.expression(initializer)?;

                (attributes, name, hir::EnumVariantKind::Scalar(initializer))
            }
            ast::EnumVariant::Named(node) => {
                let mut attributes = vec![];
                for attribute in node.attributes() {
                    attributes.push(self.attribute(attribute)?);
                }

                let Some(name) = node.name() else {
                    self.diagnostic
                        .report(miette!("Enum variant without a name"));
                    return Err(());
                };
                let name = self.identifier(name)?;

                let mut fields = vec![];
                for field in node.fields() {
                    let mut attributes = vec![];
                    for attribute in field.attributes() {
                        attributes.push(self.attribute(attribute)?);
                    }

                    let Some(name) = field.name() else {
                        self.diagnostic
                            .report(miette!("Enum variant field without a name"));
                        return Err(());
                    };
                    let name = self.identifier(name)?;

                    let Some(ty) = field.r#type() else {
                        self.diagnostic
                            .report(miette!("Enum variant field without a type"));
                        return Err(());
                    };
                    let ty = self.type_expression(ty, false)?;

                    fields.push((attributes, name, ty));
                }

                (attributes, name, hir::EnumVariantKind::Named(fields))
            }
            ast::EnumVariant::Unnamed(node) => {
                let mut attributes = vec![];
                for attribute in node.attributes() {
                    attributes.push(self.attribute(attribute)?);
                }

                let Some(name) = node.name() else {
                    self.diagnostic
                        .report(miette!("Enum variant without a name"));
                    return Err(());
                };
                let name = self.identifier(name)?;

                let mut fields = vec![];
                for field in node.fields() {
                    let mut attributes = vec![];
                    for attribute in field.attributes() {
                        attributes.push(self.attribute(attribute)?);
                    }

                    let Some(ty) = field.r#type() else {
                        self.diagnostic
                            .report(miette!("Enum variant field without a type"));
                        return Err(());
                    };
                    let ty = self.type_expression(ty, false)?;

                    fields.push((attributes, ty));
                }

                (attributes, name, hir::EnumVariantKind::Unnamed(fields))
            }
        };

        Ok(hir::EnumVariant {
            attributes,
            name,
            kind,
            span,
        })
    }

    fn visibility(&mut self, node: Option<ast::Visibility>) -> hir::Visibility {
        match node {
            None => hir::Visibility::Private,
            Some(visibility) => {
                if visibility.krate().is_some() {
                    hir::Visibility::Krate
                } else if visibility.super_().is_some() {
                    hir::Visibility::Super
                } else if visibility.self_().is_some() {
                    hir::Visibility::Self_
                } else {
                    self.diagnostic
                        .report(miette!("Item with invalid visibility"));
                    hir::Visibility::Private
                }
            }
        }
    }

    fn type_expression(
        &mut self,
        node: ast::TypeExpression,
        mutable: bool,
    ) -> Result<hir::TypeExpression, ()> {
        match node {
            ast::TypeExpression::Never(_) => Ok(hir::TypeExpression {
                mutable,
                kind: hir::TypeExpressionKind::Never,
                span: Span::new(node.syntax()),
            }),
            ast::TypeExpression::Mutable(mutable) => {
                let Some(inner) = mutable.r#type() else {
                    self.diagnostic
                        .report(miette!("Mutable type without an inner type"));
                    return Err(());
                };
                let mut inner = self.type_expression(inner, true)?;
                inner.mutable = true;

                Ok(inner)
            }
            ast::TypeExpression::Path(path) => {
                let span = Span::new(path.syntax());
                let Some(path) = path.path() else {
                    self.diagnostic.report(miette!("Path type without a path"));
                    return Err(());
                };
                let segments = self.path(path)?;

                Ok(hir::TypeExpression {
                    mutable,
                    kind: hir::TypeExpressionKind::Path {
                        path: hir::Path {
                            segments,
                            binding: hir::Binding::Unresolved,
                        },
                    },
                    span,
                })
            }
            ast::TypeExpression::Slice(slice) => {
                let Some(type_expression) = slice.r#type() else {
                    self.diagnostic
                        .report(miette!("Slice type without an element type"));
                    return Err(());
                };
                let type_expression = self.type_expression(type_expression, false)?;

                Ok(hir::TypeExpression {
                    mutable,
                    kind: hir::TypeExpressionKind::Slice {
                        element: Box::new(type_expression),
                    },
                    span: Span::new(slice.syntax()),
                })
            }
            ast::TypeExpression::Tuple(tuple) => {
                let mut arguments = vec![];
                for argument in tuple.arguments() {
                    let Some(type_expression) = argument.r#type() else {
                        self.diagnostic
                            .report(miette!("Tuple type without an element type"));
                        return Err(());
                    };
                    arguments.push(self.type_expression(type_expression, false)?);
                }

                Ok(hir::TypeExpression {
                    mutable,
                    kind: hir::TypeExpressionKind::Tuple {
                        elements: arguments,
                    },
                    span: Span::new(tuple.syntax()),
                })
            }
        }
    }

    fn expression(&mut self, node: ast::Expression) -> Result<hir::Expression, ()> {
        let kind = match node.clone() {
            ast::Expression::Break(_) => hir::ExpressionKind::Break,
            ast::Expression::Continue(_) => hir::ExpressionKind::Continue,
            ast::Expression::Return(node) => {
                let value = match node.expression() {
                    None => None,
                    Some(expr) => Some(Box::new(self.expression(expr)?)),
                };

                hir::ExpressionKind::Return { value }
            }
            ast::Expression::For(node) => {
                let Some(pattern) = node.pattern() else {
                    self.diagnostic
                        .report(miette!("For expression without a pattern"));
                    return Err(());
                };
                let pattern = self.pattern(pattern, false)?;

                let Some(iterable) = node.iterable() else {
                    self.diagnostic
                        .report(miette!("For expression without an iterable"));
                    return Err(());
                };
                let iterable = self.expression(iterable)?;
                let iterable = Box::new(iterable);

                let Some(body) = node.body() else {
                    self.diagnostic
                        .report(miette!("For expression without a body"));
                    return Err(());
                };
                let body = self.block_expression(body)?;

                hir::ExpressionKind::For {
                    pattern,
                    iterable,
                    body,
                }
            }
            ast::Expression::While(node) => {
                let Some(condition) = node.condition() else {
                    self.diagnostic
                        .report(miette!("While expression without a condition"));
                    return Err(());
                };
                let condition = self.expression(condition)?;
                let condition = Box::new(condition);

                let Some(body) = node.body() else {
                    self.diagnostic
                        .report(miette!("While expression without a body"));
                    return Err(());
                };
                let body = self.block_expression(body)?;

                hir::ExpressionKind::While { condition, body }
            }
            ast::Expression::Loop(node) => {
                let Some(body) = node.body() else {
                    self.diagnostic
                        .report(miette!("Loop expression without a body"));
                    return Err(());
                };
                let body = self.block_expression(body)?;

                hir::ExpressionKind::Loop { body }
            }
            ast::Expression::If(node) => {
                let Some(condition) = node.condition() else {
                    self.diagnostic
                        .report(miette!("If expression without a condition"));
                    return Err(());
                };
                let condition = self.expression(condition)?;
                let condition = Box::new(condition);

                let Some(then_branch) = node.then_branch() else {
                    self.diagnostic
                        .report(miette!("If expression without a then branch"));
                    return Err(());
                };
                let then_branch = self.block_expression(then_branch)?;

                let else_branch = match node.else_branch() {
                    None => None,
                    Some(else_branch) => Some(Box::new(self.expression(else_branch)?)),
                };

                hir::ExpressionKind::If {
                    condition,
                    then_branch,
                    else_branch,
                }
            }
            ast::Expression::Match(node) => {
                let Some(expression) = node.expression() else {
                    self.diagnostic
                        .report(miette!("Match expression without an expression"));
                    return Err(());
                };
                let expression = self.expression(expression)?;
                let expression = Box::new(expression);

                let mut arms = vec![];
                for arm in node.arms() {
                    let Some(pattern) = arm.pattern() else {
                        self.diagnostic
                            .report(miette!("Match arm without a pattern"));
                        return Err(());
                    };
                    let pattern = self.pattern(pattern, false)?;

                    let Some(expression) = arm.expression() else {
                        self.diagnostic
                            .report(miette!("Match arm without an expression"));
                        return Err(());
                    };
                    let expression = self.expression(expression)?;

                    arms.push((pattern, expression));
                }

                hir::ExpressionKind::Match { expression, arms }
            }
            ast::Expression::Let(node) => {
                let Some(pattern) = node.pattern() else {
                    self.diagnostic
                        .report(miette!("Let expression without a pattern"));
                    return Err(());
                };
                let pattern = self.pattern(pattern, false)?;

                let r#type = match node.r#type() {
                    None => None,
                    Some(ty) => Some(self.type_expression(ty, false)?),
                };

                let initializer = match node.initializer() {
                    None => None,
                    Some(expr) => Some(Box::new(self.expression(expr)?)),
                };

                // TODO: Add let expressions to scope

                hir::ExpressionKind::Let {
                    pattern,
                    r#type,
                    initializer,
                }
            }
            ast::Expression::Array(node) => {
                let mut elements = vec![];
                for element in node.elements() {
                    elements.push(self.expression(element)?);
                }

                hir::ExpressionKind::Array { elements }
            }
            ast::Expression::Tuple(node) => {
                let mut elements = vec![];
                for element in node.elements() {
                    elements.push(self.expression(element)?);
                }

                hir::ExpressionKind::Tuple { elements }
            }
            ast::Expression::Block(node) => {
                let mut attributes = vec![];
                for attribute in node.attributes() {
                    attributes.push(self.attribute(attribute)?);
                }

                let statements =
                    self.with_scope(ScopeKind::Block, |this| this.block_expression(node))?;

                hir::ExpressionKind::Block {
                    attributes,
                    statements,
                }
            }
            ast::Expression::Literal(node) => {
                let Some(value) = node.literal() else {
                    self.diagnostic
                        .report(miette!("Literal expression without a literal"));
                    return Err(());
                };
                let value = self.literal(value)?;

                hir::ExpressionKind::Literal { value }
            }
            ast::Expression::Path(node) => {
                let Some(path) = node.path() else {
                    self.diagnostic
                        .report(miette!("Path expression without a path"));
                    return Err(());
                };
                let path = self.path(path)?;
                let path = hir::Path {
                    segments: path,
                    binding: hir::Binding::Unresolved,
                };

                hir::ExpressionKind::Path { path }
            }
            ast::Expression::Unary(node) => {
                let operator = self.unary_operator(node.operator())?;

                let Some(operand) = node.operand() else {
                    self.diagnostic
                        .report(miette!("Unary expression without an operand"));
                    return Err(());
                };
                let operand = self.expression(operand)?;
                let operand = Box::new(operand);

                hir::ExpressionKind::Unary { operator, operand }
            }
            ast::Expression::Binary(node) => {
                let Some(left) = node.left() else {
                    self.diagnostic
                        .report(miette!("Binary expression without a left operand"));
                    return Err(());
                };
                let left = self.expression(left)?;
                let left = Box::new(left);

                let operator = self.binary_operator(node.operator())?;

                let Some(right) = node.right() else {
                    self.diagnostic
                        .report(miette!("Binary expression without a right operand"));
                    return Err(());
                };
                let right = self.expression(right)?;
                let right = Box::new(right);

                hir::ExpressionKind::Binary {
                    left,
                    operator,
                    right,
                }
            }
            ast::Expression::Assignment(node) => {
                let Some(left) = node.left() else {
                    self.diagnostic
                        .report(miette!("Assignment expression without a left operand"));
                    return Err(());
                };
                let left = self.expression(left)?;
                let left = Box::new(left);

                let operator = self.assignment_operator(node.operator())?;

                let Some(right) = node.right() else {
                    self.diagnostic
                        .report(miette!("Assignment expression without a right operand"));
                    return Err(());
                };
                let right = self.expression(right)?;
                let right = Box::new(right);

                hir::ExpressionKind::Assignment {
                    left,
                    operator,
                    right,
                }
            }
            ast::Expression::FunctionCall(node) => {
                let Some(callee) = node.callee() else {
                    self.diagnostic
                        .report(miette!("Function call expression without a callee"));
                    return Err(());
                };
                let callee = self.expression(callee)?;
                let callee = Box::new(callee);

                let type_arguments = vec![];

                let mut arguments = vec![];
                for argument in node.arguments() {
                    arguments.push(self.expression(argument)?);
                }

                hir::ExpressionKind::FunctionCall {
                    callee,
                    type_arguments,
                    arguments,
                }
            }
            ast::Expression::MethodCall(node) => {
                let Some(receiver) = node.receiver() else {
                    self.diagnostic
                        .report(miette!("Method call expression without a receiver"));
                    return Err(());
                };
                let receiver = self.expression(receiver)?;
                let receiver = Box::new(receiver);

                let Some(method) = node.method() else {
                    self.diagnostic
                        .report(miette!("Method call expression without a name"));
                    return Err(());
                };
                let method = self.identifier(method)?;

                let type_arguments = vec![];

                let mut arguments = vec![];
                for argument in node.arguments() {
                    arguments.push(self.expression(argument)?);
                }

                hir::ExpressionKind::MethodCall {
                    receiver,
                    method,
                    type_arguments,
                    arguments,
                }
            }
            ast::Expression::Field(node) => {
                let Some(receiver) = node.receiver() else {
                    self.diagnostic
                        .report(miette!("Field expression without a receiver"));
                    return Err(());
                };
                let receiver = self.expression(receiver)?;
                let receiver = Box::new(receiver);

                let Some(field) = node.field() else {
                    self.diagnostic
                        .report(miette!("Field expression without a field"));
                    return Err(());
                };
                let field = self.identifier(field)?;

                hir::ExpressionKind::Field { receiver, field }
            }
            ast::Expression::Index(node) => {
                let Some(receiver) = node.receiver() else {
                    self.diagnostic
                        .report(miette!("Index expression without a receiver"));
                    return Err(());
                };
                let receiver = self.expression(receiver)?;
                let receiver = Box::new(receiver);

                let Some(index) = node.index() else {
                    self.diagnostic
                        .report(miette!("Index expression without an index"));
                    return Err(());
                };
                let index = self.expression(index)?;
                let index = Box::new(index);

                hir::ExpressionKind::Index { receiver, index }
            }
            ast::Expression::Await(node) => {
                let Some(expression) = node.expression() else {
                    self.diagnostic
                        .report(miette!("Await expression without an expression"));
                    return Err(());
                };
                let expression = self.expression(expression)?;
                let expression = Box::new(expression);

                hir::ExpressionKind::Await { expression }
            }
            ast::Expression::Range(node) => {
                let range = match node {
                    ast::RangeExpression::Full(_) => hir::RangeExpression::Full,
                    ast::RangeExpression::To(node) => {
                        let Some(end) = node.end() else {
                            self.diagnostic
                                .report(miette!("Range to expression without an end"));
                            return Err(());
                        };
                        let end = self.expression(end)?;
                        let end = Box::new(end);

                        hir::RangeExpression::To { end }
                    }
                    ast::RangeExpression::FromTo(node) => {
                        let Some(start) = node.start() else {
                            self.diagnostic
                                .report(miette!("Range from-to expression without a start"));
                            return Err(());
                        };
                        let start = self.expression(start)?;
                        let start = Box::new(start);

                        let Some(end) = node.end() else {
                            self.diagnostic
                                .report(miette!("Range from-to expression without an end"));
                            return Err(());
                        };
                        let end = self.expression(end)?;
                        let end = Box::new(end);

                        hir::RangeExpression::FromTo { start, end }
                    }
                    ast::RangeExpression::From(node) => {
                        let Some(start) = node.start() else {
                            self.diagnostic
                                .report(miette!("Range from expression without a start"));
                            return Err(());
                        };
                        let start = self.expression(start)?;
                        let start = Box::new(start);

                        hir::RangeExpression::From { start }
                    }
                    ast::RangeExpression::FromToInclusive(node) => {
                        let Some(start) = node.start() else {
                            self.diagnostic.report(miette!(
                                "Range from-to-inclusive expression without a start"
                            ));
                            return Err(());
                        };
                        let start = self.expression(start)?;
                        let start = Box::new(start);

                        let Some(end) = node.end() else {
                            self.diagnostic.report(miette!(
                                "Range from-to-inclusive expression without an end"
                            ));
                            return Err(());
                        };
                        let end = self.expression(end)?;
                        let end = Box::new(end);

                        hir::RangeExpression::FromToInclusive { start, end }
                    }
                    ast::RangeExpression::ToInclusive(node) => {
                        let Some(end) = node.end() else {
                            self.diagnostic
                                .report(miette!("Range to-inclusive expression without an end"));
                            return Err(());
                        };
                        let end = self.expression(end)?;
                        let end = Box::new(end);

                        hir::RangeExpression::ToInclusive { end }
                    }
                };

                hir::ExpressionKind::Range { range }
            }
            ast::Expression::Struct(node) => {
                let Some(path) = node.path() else {
                    self.diagnostic
                        .report(miette!("Struct expression without a path"));
                    return Err(());
                };
                let path = self.path_expression(path)?;

                let type_arguments = vec![];

                let mut fields = vec![];
                for field in node.fields() {
                    let Some(name) = field.name() else {
                        self.diagnostic
                            .report(miette!("Struct field without a name"));
                        return Err(());
                    };
                    let name = self.identifier(name)?;

                    let Some(value) = field.value() else {
                        self.diagnostic
                            .report(miette!("Struct field without a value"));
                        return Err(());
                    };
                    let value = self.expression(value)?;

                    fields.push((name, value));
                }

                hir::ExpressionKind::Struct {
                    path,
                    type_arguments,
                    fields,
                }
            }
            ast::Expression::Try(node) => {
                let Some(value) = node.expression() else {
                    self.diagnostic
                        .report(miette!("Try expression without an expression"));
                    return Err(());
                };
                let value = self.expression(value)?;
                let value = Box::new(value);

                hir::ExpressionKind::Try { value }
            }
            ast::Expression::Yield(node) => {
                let value = match node.expression() {
                    Some(expr) => Some(Box::new(self.expression(expr)?)),
                    None => None,
                };

                hir::ExpressionKind::Yield { value }
            }
        };

        Ok(hir::Expression {
            kind,
            span: Span::new(node.syntax()),
        })
    }

    fn block_expression(&mut self, node: ast::BlockExpression) -> Result<Vec<hir::Statement>, ()> {
        let mut statements = vec![];
        for statement in node.statements() {
            match self.statement(statement)? {
                Some(statement) => statements.push(statement),
                None => continue,
            }
        }

        Ok(statements)
    }

    fn path_expression(&mut self, node: ast::PathExpression) -> Result<hir::Path, ()> {
        let Some(path) = node.path() else {
            self.diagnostic
                .report(miette!("Path expression without a path"));
            return Err(());
        };
        let segments = self.path(path)?;

        Ok(hir::Path {
            segments,
            binding: hir::Binding::Unresolved,
        })
    }

    fn statement(&mut self, node: ast::Statement) -> Result<Option<hir::Statement>, ()> {
        let kind = match node.clone() {
            ast::Statement::Definition(node) => {
                let Some(definition) = node.definition() else {
                    self.diagnostic
                        .report(miette!("Definition statement without a definition"));
                    return Err(());
                };
                let definition = match self.definition(definition)? {
                    Some(definition) => definition,
                    None => return Ok(None),
                };

                hir::StatementKind::Definition { definition }
            }
            ast::Statement::Expression(node) => {
                let Some(expression) = node.expression() else {
                    self.diagnostic
                        .report(miette!("Expression statement without an expression"));
                    return Err(());
                };
                let value = self.expression(expression)?;

                hir::StatementKind::Expression { value }
            }
            ast::Statement::Let(node) => {
                let Some(pattern) = node.pattern() else {
                    self.diagnostic
                        .report(miette!("Let statement without a pattern"));
                    return Err(());
                };
                let pattern = self.pattern(pattern, false)?;

                let r#type = match node.r#type() {
                    None => None,
                    Some(ty) => Some(self.type_expression(ty, false)?),
                };

                let initializer = match node.initializer() {
                    None => None,
                    Some(expr) => Some(self.expression(expr)?),
                };

                hir::StatementKind::Let {
                    pattern,
                    r#type,
                    initializer,
                }
            }
            ast::Statement::Semicolon(_) => hir::StatementKind::Semicolon,
        };

        Ok(Some(hir::Statement {
            kind,
            span: Span::new(node.syntax()),
        }))
    }

    fn pattern(&mut self, node: ast::Pattern, mutable: bool) -> Result<hir::Pattern, ()> {
        match node.clone() {
            ast::Pattern::Never(_) => Ok(hir::Pattern {
                mutable,
                kind: hir::PatternKind::Never,
                span: Span::new(node.syntax()),
            }),
            ast::Pattern::Placeholder(_) => Ok(hir::Pattern {
                mutable,
                kind: hir::PatternKind::Placeholder,
                span: Span::new(node.syntax()),
            }),
            ast::Pattern::Path(node) => {
                let Some(path) = node.path() else {
                    self.diagnostic
                        .report(miette!("Path pattern without a path"));
                    return Err(());
                };
                let segments = self.path(path)?;
                let path = hir::Path {
                    segments,
                    binding: hir::Binding::Unresolved,
                };

                Ok(hir::Pattern {
                    mutable,
                    kind: hir::PatternKind::Path { path },
                    span: Span::new(node.syntax()),
                })
            }
            ast::Pattern::Mutable(node) => {
                let Some(pattern) = node.pattern() else {
                    self.diagnostic
                        .report(miette!("Mutable pattern without an inner pattern"));
                    return Err(());
                };
                let mut pattern = self.pattern(pattern, true)?;
                pattern.mutable = true;

                Ok(pattern)
            }
            ast::Pattern::Tuple(node) => {
                let mut elements = vec![];
                for element in node.elements() {
                    elements.push(self.pattern(element, false)?);
                }

                Ok(hir::Pattern {
                    mutable,
                    kind: hir::PatternKind::Tuple { elements },
                    span: Span::new(node.syntax()),
                })
            }
            ast::Pattern::Array(node) => {
                let mut elements = vec![];
                for element in node.elements() {
                    elements.push(self.pattern(element, false)?);
                }

                Ok(hir::Pattern {
                    mutable,
                    kind: hir::PatternKind::Array { elements },
                    span: Span::new(node.syntax()),
                })
            }
            ast::Pattern::Literal(node) => {
                let Some(value) = node.literal() else {
                    self.diagnostic
                        .report(miette!("Literal pattern without a literal"));
                    return Err(());
                };
                let value = self.literal_expression(value)?;

                Ok(hir::Pattern {
                    mutable,
                    kind: hir::PatternKind::Literal { value },
                    span: Span::new(node.syntax()),
                })
            }
            ast::Pattern::Range(node) => match node {
                ast::RangePattern::To(node) => {
                    let Some(end) = node.end() else {
                        self.diagnostic
                            .report(miette!("Range to pattern without an end"));
                        return Err(());
                    };
                    let end = self.pattern(end, false)?;
                    let end = Box::new(end);

                    Ok(hir::Pattern {
                        mutable,
                        kind: hir::PatternKind::Range {
                            range: hir::RangePattern::To { end },
                        },
                        span: Span::new(node.syntax()),
                    })
                }
                ast::RangePattern::FromTo(node) => {
                    let Some(start) = node.start() else {
                        self.diagnostic
                            .report(miette!("Range from-to pattern without a start"));
                        return Err(());
                    };
                    let start = self.pattern(start, false)?;
                    let start = Box::new(start);

                    let Some(end) = node.end() else {
                        self.diagnostic
                            .report(miette!("Range from-to pattern without an end"));
                        return Err(());
                    };
                    let end = self.pattern(end, false)?;
                    let end = Box::new(end);

                    Ok(hir::Pattern {
                        mutable,
                        kind: hir::PatternKind::Range {
                            range: hir::RangePattern::FromTo { start, end },
                        },
                        span: Span::new(node.syntax()),
                    })
                }
                ast::RangePattern::From(node) => {
                    let Some(start) = node.start() else {
                        self.diagnostic
                            .report(miette!("Range from pattern without a start"));
                        return Err(());
                    };
                    let start = self.pattern(start, false)?;
                    let start = Box::new(start);

                    Ok(hir::Pattern {
                        mutable,
                        kind: hir::PatternKind::Range {
                            range: hir::RangePattern::From { start },
                        },
                        span: Span::new(node.syntax()),
                    })
                }
                ast::RangePattern::FromToInclusive(node) => {
                    let Some(start) = node.start() else {
                        self.diagnostic
                            .report(miette!("Range from-to-inclusive pattern without a start"));
                        return Err(());
                    };
                    let start = self.pattern(start, false)?;
                    let start = Box::new(start);

                    let Some(end) = node.end() else {
                        self.diagnostic
                            .report(miette!("Range from-to-inclusive pattern without an end"));
                        return Err(());
                    };
                    let end = self.pattern(end, false)?;
                    let end = Box::new(end);

                    Ok(hir::Pattern {
                        mutable,
                        kind: hir::PatternKind::Range {
                            range: hir::RangePattern::FromToInclusive { start, end },
                        },
                        span: Span::new(node.syntax()),
                    })
                }
                ast::RangePattern::ToInclusive(node) => {
                    let Some(end) = node.end() else {
                        self.diagnostic
                            .report(miette!("Range to-inclusive pattern without an end"));
                        return Err(());
                    };
                    let end = self.pattern(end, false)?;
                    let end = Box::new(end);

                    Ok(hir::Pattern {
                        mutable,
                        kind: hir::PatternKind::Range {
                            range: hir::RangePattern::ToInclusive { end },
                        },
                        span: Span::new(node.syntax()),
                    })
                }
            },
            ast::Pattern::At(node) => {
                let Some(name) = node.name() else {
                    self.diagnostic.report(miette!("At pattern without a name"));
                    return Err(());
                };
                let name = self.identifier(name)?;

                let Some(pattern) = node.pattern() else {
                    self.diagnostic
                        .report(miette!("At pattern without an inner pattern"));
                    return Err(());
                };
                let pattern = self.pattern(pattern, false)?;
                let pattern = Box::new(pattern);

                Ok(hir::Pattern {
                    mutable,
                    kind: hir::PatternKind::At { name, pattern },
                    span: Span::new(node.syntax()),
                })
            }
            ast::Pattern::Or(node) => {
                let mut patterns = vec![];
                for option in node.patterns() {
                    patterns.push(self.pattern(option, false)?);
                }

                Ok(hir::Pattern {
                    mutable,
                    kind: hir::PatternKind::Or { patterns },
                    span: Span::new(node.syntax()),
                })
            }
            ast::Pattern::Named(node) => {
                let Some(path) = node.path() else {
                    self.diagnostic
                        .report(miette!("Named pattern without a path"));
                    return Err(());
                };
                let path = self.path_pattern(path)?;

                let mut fields = vec![];
                for field in node.fields() {
                    let Some(name) = field.name() else {
                        self.diagnostic
                            .report(miette!("Named pattern field without a name"));
                        return Err(());
                    };
                    let name = self.identifier(name)?;

                    let Some(pattern) = field.pattern() else {
                        self.diagnostic
                            .report(miette!("Named pattern field without a pattern"));
                        return Err(());
                    };
                    let pattern = self.pattern(pattern, false)?;

                    fields.push((name, pattern));
                }

                Ok(hir::Pattern {
                    mutable,
                    kind: hir::PatternKind::Named { path, fields },
                    span: Span::new(node.syntax()),
                })
            }
            ast::Pattern::Unnamed(node) => {
                let Some(path) = node.path() else {
                    self.diagnostic
                        .report(miette!("Unnamed pattern without a path"));
                    return Err(());
                };
                let path = self.path_pattern(path)?;

                let mut elements = vec![];
                for element in node.elements() {
                    elements.push(self.pattern(element, false)?);
                }

                Ok(hir::Pattern {
                    mutable,
                    kind: hir::PatternKind::Unnamed { path, elements },
                    span: Span::new(node.syntax()),
                })
            }
        }
    }

    fn path_pattern(&mut self, node: ast::PathPattern) -> Result<hir::Path, ()> {
        let Some(path) = node.path() else {
            self.diagnostic
                .report(miette!("Path pattern without a path"));
            return Err(());
        };
        let segments = self.path(path)?;

        Ok(hir::Path {
            segments,
            binding: hir::Binding::Unresolved,
        })
    }

    fn literal_expression(&mut self, node: ast::LiteralExpression) -> Result<hir::Literal, ()> {
        let Some(literal) = node.literal() else {
            self.diagnostic
                .report(miette!("Literal expression without a literal"));
            return Err(());
        };
        self.literal(literal)
    }

    fn literal(&mut self, node: ast::Literal) -> Result<hir::Literal, ()> {
        let kind = match node.clone() {
            ast::Literal::Boolean(node) => {
                let value = if node.r#true().is_some() {
                    true
                } else if node.r#false().is_some() {
                    false
                } else {
                    self.diagnostic
                        .report(miette!("Boolean literal without a value"));
                    return Err(());
                };

                hir::LiteralKind::Boolean { value }
            }
            ast::Literal::Character(node) => {
                let Some(kind) = node.kind() else {
                    self.diagnostic
                        .report(miette!("Character literal without a value"));
                    return Err(());
                };
                let value = match kind {
                    ast::CharacterLiteralKind::One(node) => {
                        let Some(text) = node.character() else {
                            self.diagnostic
                                .report(miette!("Character literal without a character"));
                            return Err(());
                        };
                        let text = text.text();
                        let Some(c) = text.chars().next() else {
                            self.diagnostic
                                .report(miette!("Character literal is empty"));
                            return Err(());
                        };

                        c
                    }
                    ast::CharacterLiteralKind::Escape(node) => {
                        let Some(text) = node.segment() else {
                            self.diagnostic
                                .report(miette!("Character literal without an escape sequence"));
                            return Err(());
                        };
                        let text = text.text();
                        let Some(c) = text.chars().next() else {
                            self.diagnostic
                                .report(miette!("Character literal escape sequence is empty"));
                            return Err(());
                        };

                        match c {
                            '\\' => '\\',
                            '\'' => '\'',
                            '\"' => '\"',
                            'n' => '\n',
                            't' => '\t',
                            _ => {
                                self.diagnostic.report(miette!(
                                    "Character literal with unknown escape sequence: \\{}",
                                    c
                                ));
                                return Err(());
                            }
                        }
                    }
                    ast::CharacterLiteralKind::Unicode(node) => {
                        let code_point: String =
                            node.segments().map(|s| s.text().to_owned()).collect();
                        let Ok(value) = u32::from_str_radix(&code_point, 16) else {
                            self.diagnostic.report(miette!(
                                "Character literal with invalid unicode code point: {}",
                                code_point
                            ));
                            return Err(());
                        };
                        let Some(c) = std::char::from_u32(value) else {
                            self.diagnostic.report(miette!(
                                "Character literal with invalid unicode code point: {}",
                                code_point
                            ));
                            return Err(());
                        };

                        c
                    }
                };

                hir::LiteralKind::Character { value }
            }
            ast::Literal::Integer(node) => {
                let text = node.syntax().text().to_owned().to_string();
                let Some(value) = text.replace('_', "").parse().ok() else {
                    self.diagnostic
                        .report(miette!("Integer literal with invalid value: {}", text));
                    return Err(());
                };

                hir::LiteralKind::Integer { value }
            }
            ast::Literal::Float(node) => {
                let text = node.syntax().text().to_owned().to_string();
                let Some(value) = text.replace('_', "").parse().ok() else {
                    self.diagnostic
                        .report(miette!("Float literal with invalid value: {}", text));
                    return Err(());
                };

                hir::LiteralKind::Float { value }
            }
            ast::Literal::String(node) => {
                let mut segments = vec![];
                for segment in node.segments() {
                    let segment = match segment {
                        ast::StringSegment::Text(node) => {
                            let value = node.syntax().text().to_owned().to_string();

                            hir::StringSegment::Text { value }
                        }
                        ast::StringSegment::Escape(node) => {
                            let Some(value) = node.segment() else {
                                self.diagnostic.report(miette!(
                                    "String escape segment without an escape sequence"
                                ));
                                return Err(());
                            };
                            let value = match value.text().chars().next() {
                                Some('\\') => '\\',
                                Some('\'') => '\'',
                                Some('\"') => '\"',
                                Some('n') => '\n',
                                Some('t') => '\t',
                                Some(c) => {
                                    self.diagnostic.report(miette!(
                                        "String literal with unknown escape sequence: \\{}",
                                        c
                                    ));
                                    return Err(());
                                }
                                None => {
                                    self.diagnostic.report(miette!(
                                        "String escape segment with empty escape sequence"
                                    ));
                                    return Err(());
                                }
                            };

                            hir::StringSegment::Escape { value }
                        }
                        ast::StringSegment::Unicode(node) => {
                            let code_point: String =
                                node.segments().map(|s| s.text().to_owned()).collect();
                            let Some(value) = u32::from_str_radix(&code_point, 16).ok() else {
                                self.diagnostic.report(miette!(
                                    "String literal with invalid unicode code point: {}",
                                    code_point
                                ));
                                return Err(());
                            };
                            let Some(value) = std::char::from_u32(value) else {
                                self.diagnostic.report(miette!(
                                    "String literal with invalid unicode code point: {}",
                                    code_point
                                ));
                                return Err(());
                            };

                            hir::StringSegment::Unicode { value }
                        }
                        ast::StringSegment::Interpolation(node) => {
                            let Some(expression) = node.expression() else {
                                self.diagnostic.report(miette!(
                                    "String interpolation segment without an expression"
                                ));
                                return Err(());
                            };
                            let expression = self.expression(expression)?;

                            hir::StringSegment::Interpolation { expression }
                        }
                    };

                    segments.push(segment);
                }

                hir::LiteralKind::String { segments }
            }
            ast::Literal::Binary(node) => {
                let text = node.syntax().text().to_owned().to_string();
                let Some(value) = i128::from_str_radix(&text[2..], 2).ok() else {
                    self.diagnostic
                        .report(miette!("Binary literal with invalid value: {}", text));
                    return Err(());
                };

                hir::LiteralKind::Integer { value }
            }
            ast::Literal::Octal(node) => {
                let text = node.syntax().text().to_owned().to_string();
                let Some(value) = i128::from_str_radix(&text[2..], 8).ok() else {
                    self.diagnostic
                        .report(miette!("Octal literal with invalid value: {}", text));
                    return Err(());
                };

                hir::LiteralKind::Integer { value }
            }
            ast::Literal::Hex(node) => {
                let text = node.syntax().text().to_owned().to_string();
                let Some(value) = i128::from_str_radix(&text[2..], 16).ok() else {
                    self.diagnostic
                        .report(miette!("Hex literal with invalid value: {}", text));
                    return Err(());
                };

                hir::LiteralKind::Integer { value }
            }
        };

        Ok(hir::Literal {
            kind,
            span: Span::new(node.syntax()),
        })
    }

    fn path(&mut self, node: ast::Path) -> Result<Vec<hir::PathSegment>, ()> {
        let mut segments = vec![];
        for segment in node.segments() {
            segments.push(self.path_segment(segment)?);
        }

        Ok(segments)
    }

    fn path_segment(&mut self, node: ast::PathSegment) -> Result<hir::PathSegment, ()> {
        let kind = match node {
            ast::PathSegment::Krate(_) => hir::PathSegmentKind::Krate,
            ast::PathSegment::Self_(_) => hir::PathSegmentKind::Self_,
            ast::PathSegment::Super_(_) => hir::PathSegmentKind::Super_,
            ast::PathSegment::Root(_) => hir::PathSegmentKind::Root,
            ast::PathSegment::Identifier(ident) => {
                let Some(ident) = ident.identifier() else {
                    self.diagnostic
                        .report(miette!("Path segment without identifier"));
                    return Err(());
                };
                let name = self.identifier(ident)?;

                hir::PathSegmentKind::Identifier(name)
            }
        };

        Ok(hir::PathSegment {
            kind,
            binding: hir::Binding::Unresolved,
        })
    }

    fn identifier(&mut self, node: ast::Identifier) -> Result<hir::Identifier, ()> {
        let Some(segment) = node.segment() else {
            self.diagnostic
                .report(miette!("Identifier without a segment"));
            return Err(());
        };
        let Some(name) = segment.identifier() else {
            self.diagnostic
                .report(miette!("Identifier segment without an identifier"));
            return Err(());
        };

        Ok(hir::Identifier {
            symbol: self.symbols.intern(name.text()),
            span: Span::new(node.syntax()),
        })
    }

    fn unary_operator(
        &mut self,
        node: Option<ast::UnaryOperator>,
    ) -> Result<hir::UnaryOperator, ()> {
        use danubec_syntax::SyntaxKind::*;

        let Some(node) = node else {
            self.diagnostic
                .report(miette!("Binary operator is missing"));
            return Err(());
        };

        match node
            .syntax()
            .descendants_with_tokens()
            .find_map(|node| node.into_token())
            .map(|t| t.kind())
        {
            Some(PLUS) => Ok(hir::UnaryOperator::Positive),
            Some(HYPHEN) => Ok(hir::UnaryOperator::Negate),
            Some(EXCLAMATION) => Ok(hir::UnaryOperator::Not),
            Some(TILDE) => Ok(hir::UnaryOperator::BitwiseNot),
            _ => {
                self.diagnostic.report(miette!("Unknown unary operator"));
                Err(())
            }
        }
    }

    fn binary_operator(
        &mut self,
        node: Option<ast::BinaryOperator>,
    ) -> Result<hir::BinaryOperator, ()> {
        use danubec_syntax::SyntaxKind::*;

        let Some(node) = node else {
            self.diagnostic
                .report(miette!("Binary operator is missing"));
            return Err(());
        };

        match node
            .syntax()
            .descendants_with_tokens()
            .find_map(|node| node.into_token())
            .map(|t| t.kind())
        {
            Some(PLUS) => Ok(hir::BinaryOperator::Add),
            Some(PLUS__PIPE) => Ok(hir::BinaryOperator::SaturatingAdd),
            Some(PLUS__PERCENT) => Ok(hir::BinaryOperator::WrappingAdd),
            Some(HYPHEN) => Ok(hir::BinaryOperator::Subtract),
            Some(HYPHEN__PIPE) => Ok(hir::BinaryOperator::SaturatingSubtract),
            Some(HYPHEN__PERCENT) => Ok(hir::BinaryOperator::WrappingSubtract),
            Some(ASTERISK) => Ok(hir::BinaryOperator::Multiply),
            Some(ASTERISK__PIPE) => Ok(hir::BinaryOperator::SaturatingMultiply),
            Some(ASTERISK__PERCENT) => Ok(hir::BinaryOperator::WrappingMultiply),
            Some(SLASH) => Ok(hir::BinaryOperator::Divide),
            Some(PERCENT) => Ok(hir::BinaryOperator::Remainder),
            Some(ASTERISK__ASTERISK) => Ok(hir::BinaryOperator::Exponent),
            Some(ASTERISK__ASTERISK__PIPE) => Ok(hir::BinaryOperator::SaturatingExponent),
            Some(ASTERISK__ASTERISK__PERCENT) => Ok(hir::BinaryOperator::WrappingExponent),
            Some(CARET) => Ok(hir::BinaryOperator::BitwiseXor),
            Some(AMPERSAND) => Ok(hir::BinaryOperator::BitwiseAnd),
            Some(PIPE) => Ok(hir::BinaryOperator::BitwiseOr),
            Some(AMPERSAND__AMPERSAND) => Ok(hir::BinaryOperator::LogicalAnd),
            Some(PIPE__PIPE) => Ok(hir::BinaryOperator::LogicalOr),
            Some(EQUAL__EQUAL) => Ok(hir::BinaryOperator::Equal),
            Some(EXCLAMATION__EQUAL) => Ok(hir::BinaryOperator::NotEqual),
            Some(LEFT_CHEVRON) => Ok(hir::BinaryOperator::Less),
            Some(LEFT_CHEVRON__EQUAL) => Ok(hir::BinaryOperator::LessOrEqual),
            Some(RIGHT_CHEVRON) => Ok(hir::BinaryOperator::Greater),
            Some(RIGHT_CHEVRON__EQUAL) => Ok(hir::BinaryOperator::GreaterOrEqual),
            Some(LEFT_CHEVRON__LEFT_CHEVRON) => Ok(hir::BinaryOperator::LeftShift),
            Some(RIGHT_CHEVRON__RIGHT_CHEVRON) => Ok(hir::BinaryOperator::RightShift),
            Some(LEFT_CHEVRON__LEFT_CHEVRON__PIPE) => Ok(hir::BinaryOperator::LeftShift),
            Some(RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON) => {
                Ok(hir::BinaryOperator::RightShiftUnsigned)
            }
            _ => {
                self.diagnostic.report(miette!("Unknown binary operator"));
                Err(())
            }
        }
    }

    fn assignment_operator(
        &mut self,
        node: Option<ast::AssignmentOperator>,
    ) -> Result<hir::AssignmentOperator, ()> {
        use danubec_syntax::SyntaxKind::*;

        let Some(node) = node else {
            self.diagnostic
                .report(miette!("Binary operator is missing"));
            return Err(());
        };

        match node
            .syntax()
            .descendants_with_tokens()
            .find_map(|node| node.into_token())
            .map(|t| t.kind())
        {
            Some(EQUAL) => Ok(hir::AssignmentOperator::Assign),
            Some(PLUS__EQUAL) => Ok(hir::AssignmentOperator::Add),
            Some(PLUS__PIPE__EQUAL) => Ok(hir::AssignmentOperator::SaturatingAdd),
            Some(PLUS__PERCENT__EQUAL) => Ok(hir::AssignmentOperator::WrappingAdd),
            Some(HYPHEN__EQUAL) => Ok(hir::AssignmentOperator::Subtract),
            Some(HYPHEN__PIPE__EQUAL) => Ok(hir::AssignmentOperator::SaturatingSubtract),
            Some(HYPHEN__PERCENT__EQUAL) => Ok(hir::AssignmentOperator::WrappingSubtract),
            Some(ASTERISK__EQUAL) => Ok(hir::AssignmentOperator::Multiply),
            Some(ASTERISK__PIPE__EQUAL) => Ok(hir::AssignmentOperator::SaturatingMultiply),
            Some(ASTERISK__PERCENT__EQUAL) => Ok(hir::AssignmentOperator::WrappingMultiply),
            Some(SLASH__EQUAL) => Ok(hir::AssignmentOperator::Divide),
            Some(PERCENT__EQUAL) => Ok(hir::AssignmentOperator::Remainder),
            Some(ASTERISK__ASTERISK__EQUAL) => Ok(hir::AssignmentOperator::Exponent),
            Some(ASTERISK__ASTERISK__PIPE__EQUAL) => {
                Ok(hir::AssignmentOperator::SaturatingExponent)
            }
            Some(ASTERISK__ASTERISK__PERCENT__EQUAL) => {
                Ok(hir::AssignmentOperator::WrappingExponent)
            }
            Some(CARET__EQUAL) => Ok(hir::AssignmentOperator::BitwiseXor),
            Some(AMPERSAND__EQUAL) => Ok(hir::AssignmentOperator::BitwiseAnd),
            Some(PIPE__EQUAL) => Ok(hir::AssignmentOperator::BitwiseOr),
            Some(AMPERSAND__AMPERSAND__EQUAL) => Ok(hir::AssignmentOperator::LogicalAnd),
            Some(PIPE__PIPE__EQUAL) => Ok(hir::AssignmentOperator::LogicalOr),
            Some(LEFT_CHEVRON__LEFT_CHEVRON__EQUAL) => Ok(hir::AssignmentOperator::LeftShift),
            Some(LEFT_CHEVRON__LEFT_CHEVRON__PIPE__EQUAL) => {
                Ok(hir::AssignmentOperator::SaturatingLeftShift)
            }
            Some(RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL) => Ok(hir::AssignmentOperator::RightShift),
            Some(RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL) => {
                Ok(hir::AssignmentOperator::RightShiftUnsigned)
            }
            _ => {
                self.diagnostic
                    .report(miette!("Unknown assignment operator"));
                return Err(());
            }
        }
    }
}

use crate::{
    env::{Env, Namespace, ScopeKind},
    fs::Fs,
};
use danubec_ast as ast;
use danubec_diagnostic::Diagnostic;
use danubec_hir as hir;
use danubec_parse::parse;
use danubec_symbol::{AttributeId, FileId, ModuleId, ScopeId, SymbolInterner};
use danubec_syntax::{AstNode, Span, SyntaxNode};
use std::collections::VecDeque;

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
                    kind: hir::DefinitionKind::Module {
                        name,
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
    node: SyntaxNode,
    symbols: &mut SymbolInterner,
) -> Vec<(ast::Definition, hir::Identifier)> {
    use danubec_syntax::AstNode;

    node.children()
        .filter_map(ast::Definition::cast)
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
            for attribute in node.children().filter_map(ast::TopLevelAttribute::cast) {
                this.top_level_attribute(attribute);
            }

            for definition in node.children().filter_map(ast::Definition::cast) {
                this.definition(definition);
            }
        });
    }

    fn top_level_attribute(&mut self, _: ast::TopLevelAttribute) {
        //
    }

    fn attribute(&mut self, _: ast::Attribute) -> Option<AttributeId> {
        std::todo!();
    }

    fn definition(&mut self, node: ast::Definition) -> Option<()> {
        let mut attributes = vec![];
        for attribute in node.syntax().children().filter_map(ast::Attribute::cast) {
            attributes.push(self.attribute(attribute)?);
        }

        let visibility = self.visibility(node.visibility());

        match node.kind()? {
            ast::DefinitionKind::Function(node) => {
                self.function_definition(node, attributes, visibility)
            }
            ast::DefinitionKind::Struct(node) => {
                self.struct_definition(node, attributes, visibility)
            }
            ast::DefinitionKind::Enum(node) => self.enum_definition(node, attributes, visibility),
            ast::DefinitionKind::Module(node) => {
                self.module_definition(node, attributes, visibility)
            }
            ast::DefinitionKind::Trait(node) => self.trait_definition(node, attributes, visibility),
            ast::DefinitionKind::Constant(node) => {
                self.constant_definition(node, attributes, visibility)
            }
            ast::DefinitionKind::Static(node) => {
                self.static_definition(node, attributes, visibility)
            }
            ast::DefinitionKind::Type(node) => self.type_definition(node, attributes, visibility),
            ast::DefinitionKind::Use(node) => self.use_definition(node, attributes, visibility),
            ast::DefinitionKind::Implement(node) => {
                self.implement_definition(node, attributes, visibility)
            }
        }
    }

    fn function_definition(
        &mut self,
        node: ast::FunctionDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Option<()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Function without a name"));
            return None;
        };
        let _ = self.identifier(name)?;

        std::todo!();
    }

    fn struct_definition(
        &mut self,
        node: ast::StructDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Option<()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Struct without a name"));
            return None;
        };
        let name = self.identifier(name)?;

        let type_parameters = vec![];

        let type_bounds = vec![];

        let Some(body) = node.body() else {
            self.diagnostic.report(miette!("Struct without a body"));
            return None;
        };
        let body = self.struct_definition_body(body)?;

        let scope = self.current_scope();
        let definition = self.env.definition(crate::env::Definition {
            scope,
            definition: hir::Definition {
                attributes,
                visibility,
                kind: hir::DefinitionKind::Struct {
                    name,
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

        Some(())
    }

    fn enum_definition(
        &mut self,
        node: ast::EnumDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Option<()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Enum without a name"));
            return None;
        };
        let name = self.identifier(name)?;

        let type_parameters = vec![];

        let type_bounds = vec![];

        let mut variants = vec![];
        for variant in node.variants() {
            variants.push(self.enum_variant(variant)?);
        }

        let scope = self.current_scope();
        let definition = self.env.definition(crate::env::Definition {
            scope,
            definition: hir::Definition {
                attributes,
                visibility,
                kind: hir::DefinitionKind::Enum {
                    name,
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

        Some(())
    }

    fn use_definition(
        &mut self,
        node: ast::UseDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Option<()> {
        let Some(tree) = node.tree() else {
            self.diagnostic.report(miette!("Use without a tree"));
            return None;
        };
        let scope = self.current_scope();
        self.use_tree(tree, scope, &attributes, &visibility, &[])?;

        Some(())
    }

    fn module_definition(
        &mut self,
        node: ast::ModuleDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Option<()> {
        let inline_module = match node.kind()? {
            ast::ModuleDefinitionKind::Inline(inline) => inline,
            _ => return None,
        };

        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Module without a name"));
            return None;
        };
        let name = self.identifier(name)?;

        let scope = self.current_scope();
        let definition = self.env.definition(crate::env::Definition {
            scope,
            definition: hir::Definition {
                attributes,
                visibility,
                kind: hir::DefinitionKind::Module {
                    name,
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

        self.with_scope(ScopeKind::Module, |this| {
            for definition in inline_module.definitions() {
                this.definition(definition);
            }
        });

        Some(())
    }

    fn trait_definition(
        &mut self,
        node: ast::TraitDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Option<()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Trait without a name"));
            return None;
        };
        let _ = self.identifier(name)?;

        std::todo!();
    }

    fn constant_definition(
        &mut self,
        node: ast::ConstantDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Option<()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Constant without a name"));
            return None;
        };
        let _ = self.identifier(name)?;

        std::todo!();
    }

    fn static_definition(
        &mut self,
        node: ast::StaticDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Option<()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Static without a name"));
            return None;
        };
        let _ = self.identifier(name)?;

        std::todo!();
    }

    fn type_definition(
        &mut self,
        node: ast::TypeDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Option<()> {
        let Some(name) = node.name() else {
            self.diagnostic.report(miette!("Type without a name"));
            return None;
        };
        let _ = self.identifier(name)?;

        std::todo!();
    }

    fn implement_definition(
        &mut self,
        _: ast::ImplementDefinition,
        attributes: Vec<AttributeId>,
        visibility: hir::Visibility,
    ) -> Option<()> {
        std::todo!();
    }

    fn use_tree(
        &mut self,
        node: ast::UseTree,
        scope: ScopeId,
        attributes: &[AttributeId],
        visibility: &hir::Visibility,
        segments: &[hir::PathSegment],
    ) -> Option<()> {
        let Some(kind) = node.kind() else {
            self.diagnostic.report(miette!("Use tree without a kind"));
            return None;
        };

        match kind {
            ast::UseTreeKind::Glob(_) => {
                if segments.is_empty() {
                    self.diagnostic.report(miette!("Use glob without a path"));
                    return None;
                }

                self.env[scope].import(attributes, visibility, segments, hir::ImportKind::Glob);
            }
            ast::UseTreeKind::Element(element) => {
                let Some(path) = element.path() else {
                    self.diagnostic
                        .report(miette!("Use element without a path"));
                    return None;
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

        Some(())
    }

    fn use_tree_trailing(
        &mut self,
        node: ast::UseTreeTrailing,
        scope: ScopeId,
        attributes: &[AttributeId],
        visibility: &hir::Visibility,
        segments: &[hir::PathSegment],
    ) -> Option<()> {
        match node {
            ast::UseTreeTrailing::Glob(_) => {
                if segments.is_empty() {
                    self.diagnostic.report(miette!("Use glob without a path"));
                    return None;
                }

                self.env[scope].import(attributes, visibility, segments, hir::ImportKind::Glob);
            }
            ast::UseTreeTrailing::Rename(element) => {
                let Some(name) = element.identifier() else {
                    self.diagnostic.report(miette!("Use rename without a name"));
                    return None;
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

        Some(())
    }

    fn struct_definition_body(&mut self, node: ast::StructBody) -> Option<hir::StructBody> {
        match node {
            ast::StructBody::Unit(_) => Some(hir::StructBody::Unit),
            ast::StructBody::Named(record) => {
                let mut fields = vec![];
                for field in record.fields() {
                    let visibility = self.visibility(field.visibility());

                    let Some(name) = field.name() else {
                        self.diagnostic
                            .report(miette!("Struct field without a name"));
                        return None;
                    };
                    let name = self.identifier(name)?;

                    let Some(ty) = field.r#type() else {
                        self.diagnostic
                            .report(miette!("Struct field without a type"));
                        return None;
                    };
                    let ty = self.type_expression(ty, false)?;

                    fields.push((visibility, name, ty));
                }

                Some(hir::StructBody::Named(fields))
            }
            ast::StructBody::Unnamed(unnamed) => {
                let mut fields = vec![];
                for field in unnamed.fields() {
                    let visibility = self.visibility(field.visibility());

                    let Some(ty) = field.r#type() else {
                        self.diagnostic
                            .report(miette!("Tuple struct field without a type"));
                        return None;
                    };
                    let ty = self.type_expression(ty, false)?;

                    fields.push((visibility, ty));
                }
                Some(hir::StructBody::Unnamed(fields))
            }
        }
    }

    fn enum_variant(&mut self, node: ast::EnumVariant) -> Option<hir::EnumVariant> {
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
                    return None;
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
                    return None;
                };
                let name = self.identifier(name)?;

                let Some(initializer) = node.initializer() else {
                    self.diagnostic
                        .report(miette!("Enum variant scalar without a value"));
                    return None;
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
                    return None;
                };
                let name = self.identifier(name)?;

                let mut fields = vec![];
                for field in node.fields() {
                    let mut attributes = vec![];
                    for attribute in field.syntax().children().filter_map(ast::Attribute::cast) {
                        attributes.push(self.attribute(attribute)?);
                    }

                    let Some(name) = field.name() else {
                        self.diagnostic
                            .report(miette!("Enum variant field without a name"));
                        return None;
                    };
                    let name = self.identifier(name)?;

                    let Some(ty) = field.r#type() else {
                        self.diagnostic
                            .report(miette!("Enum variant field without a type"));
                        return None;
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
                    return None;
                };
                let name = self.identifier(name)?;

                let mut fields = vec![];
                for field in node.fields() {
                    let mut attributes = vec![];
                    for attribute in field.syntax().children().filter_map(ast::Attribute::cast) {
                        attributes.push(self.attribute(attribute)?);
                    }

                    let Some(ty) = field.r#type() else {
                        self.diagnostic
                            .report(miette!("Enum variant field without a type"));
                        return None;
                    };
                    let ty = self.type_expression(ty, false)?;

                    fields.push((attributes, ty));
                }

                (attributes, name, hir::EnumVariantKind::Unnamed(fields))
            }
        };

        Some(hir::EnumVariant {
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
    ) -> Option<hir::TypeExpression> {
        match node {
            ast::TypeExpression::Never(_) => Some(hir::TypeExpression {
                mutable,
                kind: hir::TypeExpressionKind::Never,
                span: Span::new(node.syntax()),
            }),
            ast::TypeExpression::Mutable(mutable) => {
                let Some(inner) = mutable.r#type() else {
                    self.diagnostic
                        .report(miette!("Mutable type without an inner type"));
                    return None;
                };
                let mut inner = self.type_expression(inner, true)?;
                inner.mutable = true;

                Some(inner)
            }
            ast::TypeExpression::Path(path) => {
                let span = Span::new(path.syntax());
                let Some(path) = path.path() else {
                    self.diagnostic.report(miette!("Path type without a path"));
                    return None;
                };
                let segments = self.path(path)?;

                Some(hir::TypeExpression {
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
                    return None;
                };
                let type_expression = self.type_expression(type_expression, false)?;

                Some(hir::TypeExpression {
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
                        return None;
                    };
                    let argument = self.type_expression(type_expression, false)?;
                    arguments.push(argument);
                }

                Some(hir::TypeExpression {
                    mutable,
                    kind: hir::TypeExpressionKind::Tuple {
                        elements: arguments,
                    },
                    span: Span::new(tuple.syntax()),
                })
            }
        }
    }

    fn expression(&mut self, node: ast::Expression) -> Option<hir::Expression> {
        std::todo!();
    }

    fn path(&mut self, node: ast::Path) -> Option<Vec<hir::PathSegment>> {
        let mut segments = vec![];
        for segment in node.segments() {
            segments.push(self.path_segment(segment)?);
        }

        Some(segments)
    }

    fn path_segment(&mut self, node: ast::PathSegment) -> Option<hir::PathSegment> {
        let kind = match node {
            ast::PathSegment::Krate(_) => hir::PathSegmentKind::Krate,
            ast::PathSegment::Self_(_) => hir::PathSegmentKind::Self_,
            ast::PathSegment::Super_(_) => hir::PathSegmentKind::Super_,
            ast::PathSegment::Root(_) => hir::PathSegmentKind::Root,
            ast::PathSegment::Identifier(ident) => {
                let Some(ident) = ident.identifier() else {
                    self.diagnostic
                        .report(miette!("Path segment without identifier"));
                    return None;
                };
                let name = self.identifier(ident)?;

                hir::PathSegmentKind::Identifier(name)
            }
        };

        Some(hir::PathSegment {
            kind,
            binding: hir::Binding::Unresolved,
        })
    }

    fn identifier(&mut self, node: ast::Identifier) -> Option<hir::Identifier> {
        let Some(segment) = node.segment() else {
            self.diagnostic
                .report(miette!("Identifier without a segment"));
            return None;
        };
        let Some(name) = segment.identifier() else {
            self.diagnostic
                .report(miette!("Identifier segment without an identifier"));
            return None;
        };

        Some(hir::Identifier {
            symbol: self.symbols.intern(name.text()),
            span: Span::new(node.syntax()),
        })
    }
}

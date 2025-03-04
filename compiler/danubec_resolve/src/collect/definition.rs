use super::{Collector, Namespace, RibKind, ScopeIndex};
use danubec_middle::ast;
use danubec_symbol::Symbol;

impl Collector {
    pub fn collect_krate(&mut self, name: Symbol, krate: &ast::Krate) {
        let scope = self.new_krate(name);
        for definition in &krate.definitions {
            self.collect_definition(scope, &definition);
        }
    }

    fn collect_definition(&mut self, scope: ScopeIndex, definition: &ast::Definition) {
        match &definition.kind {
            ast::DefinitionKind::Use { tree, .. } => {
                self.collect_use_definition(scope, &tree);
            }

            ast::DefinitionKind::Module {
                ident, definitions, ..
            } => {
                let scope = self.new_module(*ident, self[scope].module);

                for definition in definitions {
                    self.collect_definition(scope, &definition);
                }
            }

            ast::DefinitionKind::Struct {
                ident,
                type_parameters,
                kind,
                ..
            } => {
                let struct_scope = self.new_scope(scope, RibKind::Struct);
                self.add_symbol(scope, Namespace::Type, *ident, Some(struct_scope));

                for ty in type_parameters {
                    self.add_symbol(struct_scope, Namespace::Type, ty.ident, None);
                }

                if let Some(ast::StructKind::Named(fields)) = kind {
                    for (field, _) in fields {
                        self.add_symbol(struct_scope, Namespace::Value, *field, None);
                    }
                }
            }
            ast::DefinitionKind::Enum {
                ident,
                type_parameters,
                variants,
                ..
            } => {
                let enum_scope = self.new_scope(scope, RibKind::Enum);
                self.add_symbol(scope, Namespace::Type, *ident, Some(enum_scope));

                for ty in type_parameters {
                    self.add_symbol(enum_scope, Namespace::Type, ty.ident, None);
                }

                for variant in variants {
                    match &variant.kind {
                        ast::EnumVariantKind::Unit | ast::EnumVariantKind::Sequence(_) => {
                            self.add_symbol(enum_scope, Namespace::Value, variant.ident, None);
                        }
                        ast::EnumVariantKind::Unnamed(_) => {
                            let variant_scope = self.new_scope(enum_scope, RibKind::Constructor);
                            self.add_symbol(
                                enum_scope,
                                Namespace::Value,
                                variant.ident,
                                Some(variant_scope),
                            );
                        }
                        ast::EnumVariantKind::Named(fields) => {
                            let variant_scope = self.new_scope(enum_scope, RibKind::Constructor);
                            self.add_symbol(
                                enum_scope,
                                Namespace::Value,
                                variant.ident,
                                Some(variant_scope),
                            );

                            for (field, _) in fields {
                                self.add_symbol(variant_scope, Namespace::Value, *field, None);
                            }
                        }
                    }
                }
            }
            ast::DefinitionKind::Trait {
                ident,
                type_parameters,
                definitions,
                ..
            } => {
                let trait_scope = self.new_scope(scope, RibKind::Trait);
                self.add_symbol(scope, Namespace::Type, *ident, Some(trait_scope));

                for ty in type_parameters {
                    self.add_symbol(trait_scope, Namespace::Type, ty.ident, None);
                }

                let trait_item_scope = self.new_scope(trait_scope, RibKind::Block);
                self.add_symbol(
                    trait_scope,
                    Namespace::Value,
                    Symbol::EMPTY,
                    Some(trait_item_scope),
                );
                for definition in definitions {
                    self.collect_trait_item(trait_scope, &definition);
                }
            }
            ast::DefinitionKind::Type {
                ident,
                type_parameters,
                ..
            } => {
                self.add_symbol(scope, Namespace::Type, *ident, None);
                for ty in type_parameters {
                    self.add_symbol(scope, Namespace::Type, ty.ident, None);
                }
            }

            ast::DefinitionKind::Function(ast::FunctionDef {
                ident,
                type_parameters,
                parameters,
                ..
            }) => {
                let function_scope = self.new_scope(scope, RibKind::Function);
                self.add_symbol(scope, Namespace::Type, *ident, Some(function_scope));
                self.add_symbol(scope, Namespace::Value, *ident, Some(function_scope));

                for ty in type_parameters {
                    self.add_symbol(function_scope, Namespace::Type, ty.ident, None);
                }

                for parameter in parameters {
                    self.add_symbol(function_scope, Namespace::Value, parameter.ident, None);
                }
            }

            ast::DefinitionKind::Const { ident, .. }
            | ast::DefinitionKind::Static { ident, .. } => {
                self.add_symbol(scope, Namespace::Value, *ident, None);
            }

            ast::DefinitionKind::Impl {
                type_parameters,
                definitions,
                ..
            } => {
                let impl_scope = self.new_scope(scope, RibKind::Implement);
                for ty in type_parameters {
                    self.add_symbol(impl_scope, Namespace::Type, ty.ident, None);
                }

                let impl_item_scope = self.new_scope(impl_scope, RibKind::Block);
                self.add_symbol(
                    impl_scope,
                    Namespace::Value,
                    Symbol::EMPTY,
                    Some(impl_item_scope),
                );
                for definition in definitions {
                    self.collect_impl_item(impl_item_scope, definition);
                }
            }
        }
    }

    fn collect_use_definition(&mut self, scope: ScopeIndex, tree: &ast::UseTree) {
        self.collect_use_tree(scope, &[], tree);
    }

    fn collect_use_tree(&mut self, scope: ScopeIndex, segments: &[Symbol], tree: &ast::UseTree) {
        let segments: Vec<_> = segments
            .iter()
            .cloned()
            .chain(tree.path.segments.iter().map(|segment| segment.ident))
            .collect();

        match &tree.kind {
            Some(ast::UseTreeKind::Barrel) => {
                self.add_glob_import(scope, segments);
            }
            Some(ast::UseTreeKind::Alias(alias)) => {
                self.add_named_import(scope, segments, Some(*alias));
            }
            Some(ast::UseTreeKind::Nested(trees)) => {
                for tree in trees {
                    self.collect_use_tree(scope, &segments, &tree);
                }
            }
            None => {
                if let Some(&alias) = segments.last() {
                    self.add_named_import(scope, segments, Some(alias));
                }
            }
        }
    }

    fn collect_trait_item(&mut self, scope: ScopeIndex, item: &ast::TraitItem) {
        match item {
            ast::TraitItem::Const { ident, .. } => {
                self.add_symbol(scope, Namespace::Value, *ident, None);
            }
            ast::TraitItem::Function(ast::FunctionDef {
                ident,
                type_parameters,
                parameters,
                ..
            }) => {
                let function_scope = self.new_scope(scope, RibKind::Function);
                self.add_symbol(scope, Namespace::Value, *ident, Some(function_scope));

                for ty in type_parameters {
                    self.add_symbol(function_scope, Namespace::Type, ty.ident, None);
                }

                for parameter in parameters {
                    self.add_symbol(function_scope, Namespace::Value, parameter.ident, None);
                }
            }
            ast::TraitItem::Type {
                ident,
                type_parameters,
                ..
            } => {
                let type_scope = self.new_scope(scope, RibKind::TypeAlias);
                self.add_symbol(scope, Namespace::Type, *ident, Some(type_scope));

                for ty in type_parameters {
                    self.add_symbol(type_scope, Namespace::Type, ty.ident, None);
                }
            }
        }
    }

    fn collect_impl_item(&mut self, scope: ScopeIndex, item: &ast::ImplItem) {
        match item {
            ast::ImplItem::Const { ident, .. } => {
                self.add_symbol(scope, Namespace::Value, *ident, None);
            }
            ast::ImplItem::Function(ast::FunctionDef {
                ident,
                type_parameters,
                parameters,
                ..
            }) => {
                let function_scope = self.new_scope(scope, RibKind::Function);
                self.add_symbol(scope, Namespace::Value, *ident, Some(function_scope));

                for ty in type_parameters {
                    self.add_symbol(function_scope, Namespace::Type, ty.ident, None);
                }

                for parameter in parameters {
                    self.add_symbol(function_scope, Namespace::Value, parameter.ident, None);
                }
            }
            ast::ImplItem::Type {
                ident,
                type_parameters,
                ..
            } => {
                let type_scope = self.new_scope(scope, RibKind::TypeAlias);
                self.add_symbol(scope, Namespace::Type, *ident, Some(type_scope));

                for ty in type_parameters {
                    self.add_symbol(type_scope, Namespace::Type, ty.ident, None);
                }
            }
        }
    }
}

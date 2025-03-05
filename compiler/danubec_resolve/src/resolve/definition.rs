use super::Resolver;
use crate::collect::{Namespace, ScopeIndex};
use danubec_middle::{ast, hir};
use danubec_symbol::Symbol;

impl Resolver {
    pub fn resolve_krate(&mut self, name: Symbol, krate: &ast::Krate) {
        if let Some(module) = self.collector.find_krate(name) {
            let scope = self.collector[module].scope();
            for definition in &krate.definitions {
                self.resolve_definition(scope, &definition);
            }
        }
    }

    fn resolve_definition(&mut self, scope: ScopeIndex, definition: &ast::Definition) {
        match &definition.kind {
            ast::DefinitionKind::Use { .. } => {
                //
            }

            ast::DefinitionKind::Module { .. } => {
                //
            }

            ast::DefinitionKind::Struct {
                ident,
                type_parameters,
                predicates,
                kind,
                ..
            } => {
                let struct_binding = self
                    .collector
                    .find_binding(scope, Namespace::Type, *ident)
                    .expect("ICE: struct binding not found");
                let def_id = struct_binding.def_id();
                let struct_scope = struct_binding.scope().expect("ICE: struct scope not found");

                let type_parameters = type_parameters
                    .iter()
                    .map(|type_parameter| self.resolve_type_parameter(struct_scope, type_parameter))
                    .collect();
                let predicates = predicates
                    .iter()
                    .map(|predicate| self.resolve_predicate(struct_scope, predicate))
                    .collect();

                let kind = match kind {
                    Some(ast::StructKind::Named(fields)) => {
                        let fields = fields
                            .iter()
                            .map(|(_, field, ty)| {
                                let hir_id = self
                                    .collector
                                    .find_def_id(struct_scope, Namespace::Value, *field)
                                    .expect("ICE: field not found")
                                    .as_hir_id();

                                (hir_id, *field, self.resolve_type(struct_scope, ty))
                            })
                            .collect();

                        Some(hir::StructKind::Named { fields })
                    }
                    Some(ast::StructKind::Unnamed(fields)) => {
                        let fields = fields
                            .iter()
                            .map(|(_, ty)| self.resolve_type(struct_scope, ty))
                            .collect();

                        Some(hir::StructKind::Unnamed { fields })
                    }
                    None => None,
                };

                let definition = hir::Definition {
                    kind: hir::DefinitionKind::Struct(hir::StructDef {
                        def_id,
                        ident: *ident,
                        type_parameters,
                        predicates,
                        kind,
                    }),
                };
                self.add_definition(def_id, definition);
            }

            _ => {
                //
            }
        }
    }

    fn resolve_type_parameter(
        &mut self,
        scope: ScopeIndex,
        type_parameter: &ast::TypeParameter,
    ) -> hir::TypeParameter {
        let ty = self.resolve_ident_as_type(scope, type_parameter.ident);
        let bounds = type_parameter
            .bounds
            .iter()
            .map(|ty| self.resolve_type(scope, ty))
            .collect();

        hir::TypeParameter { ty, bounds }
    }

    fn resolve_predicate(
        &mut self,
        scope: ScopeIndex,
        predicate: &ast::Predicate,
    ) -> hir::Predicate {
        let ty = self.resolve_type(scope, &predicate.ty);
        let bounds = predicate
            .bounds
            .iter()
            .map(|bound| self.resolve_type(scope, bound))
            .collect();

        hir::Predicate { ty, bounds }
    }
}

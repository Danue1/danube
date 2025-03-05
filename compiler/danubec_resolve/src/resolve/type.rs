use super::Resolver;
use crate::collect::{Namespace, ScopeIndex};
use danubec_middle::{ast, hir};
use danubec_symbol::Symbol;

impl Resolver {
    pub fn resolve_type(&mut self, scope: ScopeIndex, ty: &ast::Type) -> hir::Type {
        match &ty.kind {
            ast::TypeKind::Path(path) => {
                let path = self.resolve_path(scope, path);

                hir::Type {
                    kind: hir::TypeKind::Path(path),
                }
            }
            _ => std::todo!(),
        }
    }

    pub fn resolve_ident_as_type(&mut self, scope: ScopeIndex, ident: Symbol) -> hir::Type {
        hir::Type {
            kind: hir::TypeKind::Path(hir::Path {
                segments: vec![hir::PathSegment {
                    hir_id: self
                        .collector
                        .find_def_id(scope, Namespace::Type, ident)
                        .expect("ICE: type not found")
                        .as_hir_id(),
                    ident,
                    type_arguments: vec![],
                }],
            }),
        }
    }
}

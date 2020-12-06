use super::*;
use danube_hir::ItemId;

impl HirContext {
    pub fn resolve_type(&self, type_kind: &danube_ast::TypeKind) -> HirResult<ItemId> {
        match type_kind {
            danube_ast::TypeKind::Path(path) => {
                //
            }
        }

        std::unimplemented!()
    }
}

use super::*;

impl crate::HirContext {
    pub(super) fn lower_ty(&mut self, ty: &TypeKind) -> crate::Ty {
        let id = self.next_id();
        let kind = match ty {
            TypeKind::TypeSelf(immutablity) => {
                std::todo!();
            }
            TypeKind::Array(immutablity, inner_ty) => {
                std::todo!();
            }
            TypeKind::Tuple(immutablity, type_list) => {
                std::todo!();
            }
            TypeKind::Path(immutablity, path) => {
                std::todo!();
            }
            TypeKind::Generic(immutablity, path, path_list) => {
                std::todo!();
            }
        };

        crate::Ty { id, kind }
    }
}

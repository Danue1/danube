use super::*;

impl crate::HirContext {
    pub(super) fn lower_visibility(&mut self, kind: &VisibilityKind) -> crate::VisibilityKind {
        match kind {
            VisibilityKind::Current => crate::VisibilityKind::Current,
            VisibilityKind::Public => crate::VisibilityKind::Public,
            VisibilityKind::Restricted(path) => {
                let id = self.next_id();
                let path = self.lower_path(path);

                crate::VisibilityKind::Restricted { id, path }
            }
            VisibilityKind::Super => std::todo!(),
            VisibilityKind::Module => std::todo!(),
        }
    }
}

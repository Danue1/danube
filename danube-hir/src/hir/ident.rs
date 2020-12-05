use super::*;

impl crate::HirContext {
    pub(super) fn lower_ident(&mut self, ident: &IdentNode) -> crate::Ident {
        let id = self.next_id();

        std::todo!();
    }

    pub(super) fn lower_path(&mut self, path: &PathNode) -> crate::Path {
        let id = self.next_id();

        std::todo!();
    }

    pub(super) fn lower_pattern(&mut self, pattern: &PatternKind) -> crate::PatternKind {
        match pattern {
            PatternKind::Placeholder => crate::PatternKind::Placeholder,
            _ => std::todo!(),
        }
    }
}

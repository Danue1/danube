use super::Resolver;
use crate::collect::{Namespace, ScopeIndex};
use danubec_middle::{ast, hir};

impl Resolver {
    pub fn resolve_path(&mut self, scope: ScopeIndex, path: &ast::Path) -> hir::Path {
        std::todo!();
    }
}

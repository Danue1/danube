use super::*;

impl HirContext {
    pub fn lower_crate(mut self, ast: &danube_ast::Module) -> HirResult<Crate> {
        self.visit_module(ast)?;

        Ok(self.into())
    }
}

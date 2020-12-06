use super::*;

impl HirContext {
    pub fn visit_module(&mut self, module: &danube_ast::Module) -> HirResult<()> {
        self.visit_item_list(module.item_list.as_slice())?;

        Ok(())
    }
}

use danube_hir::{FunctionBody, FunctionDeclaration};

use super::*;
use std::collections::HashSet;

impl HirContext {
    pub fn visit_function(&mut self, function: &danube_ast::FunctionNode) -> HirResult<()> {
        self.check_to_duplicating_ident(function)?;
        self.check_to_duplicating_argument_list(function)?;
        self.check_to_duplicating_parameter_list(function)?;
        self.check_return_type(function)?;

        let function_declaration = self.lower_function_declaration(function)?;
        let function_body = self.lower_function_body(function)?;

        Ok(())
    }

    fn check_to_duplicating_ident(&self, function: &danube_ast::FunctionNode) -> HirResult<()> {
        let ident = function.ident.raw.as_ref();
        if self.current_scope.has_item(ident) {
            Err(Error::Diagnogistic(format!(
                "duplicate definitions with name `{}`",
                ident
            )))
        } else {
            Ok(())
        }
    }

    fn check_to_duplicating_argument_list(
        &self,
        function: &danube_ast::FunctionNode,
    ) -> HirResult<()> {
        let mut set = HashSet::new();
        for parameter in function.parameter_list.iter() {
            let ident: &str = parameter.argument_label.raw.as_ref();
            if set.contains(ident) {
                return Err(Error::Diagnogistic(format!(
                    "identifier `{}` is bound more than once in this parameter list.",
                    ident
                )));
            }

            set.insert(ident);
        }

        Ok(())
    }

    fn check_to_duplicating_parameter_list(
        &self,
        function: &danube_ast::FunctionNode,
    ) -> HirResult<()> {
        let mut set = HashSet::new();
        for parameter in function.parameter_list.iter() {
            let ident: &str = parameter.label.raw.as_ref();
            if set.contains(ident) {
                return Err(Error::Diagnogistic(format!(
                    "identifier `{}` is bound more than once in this parameter list.",
                    ident
                )));
            }

            set.insert(ident);
        }

        Ok(())
    }

    fn check_return_type(&self, function: &danube_ast::FunctionNode) -> HirResult<()> {
        if let Some(return_type) = &function.return_type {
            if self.resolve_type(return_type).is_err() {
                return Err(Error::Diagnogistic(format!(
                    "failed to resolve: use of undeclared type or module `{:?}`",
                    return_type
                )));
            }
        }

        Ok(())
    }

    fn lower_function_declaration(&self, function: &danube_ast::FunctionNode) -> HirResult<()> {
        Ok(())
    }

    fn lower_function_body(&self, function: &danube_ast::FunctionNode) -> HirResult<()> {
        Ok(())
    }
}

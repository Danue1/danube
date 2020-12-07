use super::*;
use danube_hir::*;
use std::collections::HashSet;

impl HirContext {
    pub fn visit_function(&mut self, function: &danube_ast::FunctionNode) -> HirResult<()> {
        self.check_to_duplicating_ident(function)?;
        self.check_to_duplicating_argument_list(function)?;
        self.check_to_duplicating_parameter_list(function)?;
        self.check_return_type(function)?;

        let function_declaration = self.lower_function_declaration(function)?;
        let function_body = self.lower_function_body(function)?;

        self.insert_function_declaration(function.ident.raw.as_ref(), function_declaration);
        self.insert_function_body(function_body);

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

    fn lower_function_declaration(
        &self,
        function: &danube_ast::FunctionNode,
    ) -> HirResult<FunctionDeclaration> {
        Ok(FunctionDeclaration {
            argument_list: function
                .parameter_list
                .iter()
                .map(|parameter| FunctionArgument {
                    ident: Ident {
                        name: parameter.argument_label.raw.to_owned(),
                    },
                    ty: TypeKind::Path(Path {
                        resolve: ResolveKind::Primitive(PrimitiveKind::Int),
                    }),
                })
                .collect(),
            return_type: FunctionReturnTypeKind::Unit,
        })
    }

    fn lower_function_body(
        &mut self,
        function: &danube_ast::FunctionNode,
    ) -> HirResult<FunctionBody> {
        Ok(FunctionBody {
            parameter_list: function
                .parameter_list
                .iter()
                .map(|parameter| FunctionParameter {
                    ident: Ident {
                        name: parameter.label.raw.to_owned(),
                    },
                    ty: TypeKind::Path(Path {
                        resolve: ResolveKind::Primitive(PrimitiveKind::Int),
                    }),
                })
                .collect(),
            value: ExpressionKind::Literal(LiteralKind::Int(0)),
        })
    }

    pub fn insert_function_declaration(
        &mut self,
        name: &str,
        function_declaration: FunctionDeclaration,
    ) {
        self.insert_item(Item {
            ident: Ident {
                name: name.to_owned(),
            },
            attribute_list: Default::default(),
            kind: ItemKind::Function(function_declaration),
        });
    }

    pub fn insert_function_body(&mut self, function_body: FunctionBody) {
        let id = self.next_id().into();
        self.function_bodies.insert(id, function_body);
    }
}

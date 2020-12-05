use super::*;

impl crate::HirContext {
    pub(super) fn visit_item_list(&mut self, item_list: &[Item]) {
        for item in item_list {
            self.visit_item(item)
        }
    }

    fn visit_item(&mut self, item: &Item) {
        let attribute_list = self.lower_attribute_list(&item.attribute_list);

        match &item.node {
            ItemKind::Use(node) => self.visit_use(node, attribute_list),
            ItemKind::Module(node) => self.visit_module(node, attribute_list),
            ItemKind::Struct(node) => self.visit_struct(node, attribute_list),
            ItemKind::Enum(node) => self.visit_enum(node, attribute_list),
            ItemKind::Function(node) => self.visit_function(node, attribute_list),
            ItemKind::TypeAlias(node) => self.visit_type_alias(node, attribute_list),
            ItemKind::Trait(node) => self.visit_trait(node, attribute_list),
            ItemKind::Constant(node) => self.visit_constant(node, attribute_list),
            ItemKind::Static(node) => self.visit_static(node, attribute_list),
            ItemKind::Implement(node) => self.visit_implement(node, attribute_list),
            ItemKind::ImplementTrait(node) => self.visit_implement_trait(node, attribute_list),
        }
    }

    fn lower_attribute_list(&mut self, attribute_list: &[AttributeNode]) -> Vec<crate::Attribute> {
        attribute_list
            .iter()
            .map(|attribute| self.lower_attribute(attribute))
            .collect()
    }

    fn lower_attribute(&mut self, attribute: &AttributeNode) -> crate::Attribute {
        std::todo!();
    }

    fn visit_use(&mut self, node: &UseNode, attribute_list: Vec<crate::Attribute>) {
        std::todo!();
    }

    fn visit_module(&mut self, node: &ModuleNode, attribute_list: Vec<crate::Attribute>) {
        std::todo!();
    }

    fn visit_struct(&mut self, node: &StructNode, attribute_list: Vec<crate::Attribute>) {
        let visibility = self.lower_visibility(&node.visibility);
        let ident = self.lower_ident(&node.ident);

        match &node.fields {
            StructFieldKind::Named(node) => {
                let field_list: Vec<_> = node
                    .iter()
                    .map(|node| crate::StructField {
                        visibility: self.lower_visibility(&node.0),
                        id: self.next_id(),
                        ident: self.lower_ident(&node.1),
                        ty: self.lower_ty(&node.2),
                    })
                    .collect();
                self.add_named_struct(visibility, ident, attribute_list, field_list);
            }
            StructFieldKind::Unnamed(node) => {
                let field_list: Vec<_> = node
                    .iter()
                    .enumerate()
                    .map(|(index, node)| crate::StructField {
                        visibility: self.lower_visibility(&node.0),
                        id: self.next_id(),
                        ident: crate::Ident::from_usize(index),
                        ty: self.lower_ty(&node.1),
                    })
                    .collect();
                self.add_unnamed_struct(visibility, ident, attribute_list, field_list);
            }
        }

        std::todo!();
    }

    fn visit_enum(&mut self, node: &EnumNode, attribute_list: Vec<crate::Attribute>) {
        std::todo!();
    }

    fn visit_function(&mut self, node: &FunctionNode, attribute_list: Vec<crate::Attribute>) {
        let visibility = self.lower_visibility(&node.visibility);
        let argument_list = self.lower_function_argument_list(&node.argument_list);
        let block = self.lower_block(&node.block, argument_list.clone());

        std::todo!();
    }

    fn visit_type_alias(&mut self, node: &TypeAliasNode, attribute_list: Vec<crate::Attribute>) {
        std::todo!();
    }

    fn visit_trait(&mut self, node: &TraitNode, attribute_list: Vec<crate::Attribute>) {
        std::todo!();
    }

    fn visit_constant(&mut self, node: &ConstantNode, attribute_list: Vec<crate::Attribute>) {
        std::todo!();
    }

    fn visit_static(&mut self, node: &StaticNode, attribute_list: Vec<crate::Attribute>) {
        std::todo!();
    }

    fn visit_implement(&mut self, node: &ImplementNode, attribute_list: Vec<crate::Attribute>) {
        std::todo!();
    }

    fn visit_implement_trait(
        &mut self,
        node: &ImplementTraitNode,
        attribute_list: Vec<crate::Attribute>,
    ) {
        std::todo!();
    }

    fn lower_function_argument_list(
        &mut self,
        node_list: &[FunctionArgumentNode],
    ) -> Vec<crate::FunctionArgument> {
        node_list
            .iter()
            .map(|node| self.lower_function_argument(node))
            .collect()
    }

    fn lower_function_argument(&mut self, node: &FunctionArgumentNode) -> crate::FunctionArgument {
        let immutablity = self.lower_immutablity_kind(&node.immutablity);
        let pattern = self.lower_pattern(&node.pattern);

        crate::FunctionArgument {
            immutablity,
            pattern,
        }
    }

    fn lower_immutablity_kind(&mut self, node: &ImmutablityKind) -> crate::ImmutablityKind {
        match node {
            ImmutablityKind::Yes => crate::ImmutablityKind::Yes,
            ImmutablityKind::Nope => crate::ImmutablityKind::Nope,
        }
    }

    fn lower_block(
        &mut self,
        node: &BlockNode,
        parameter_list: Vec<crate::FunctionArgument>,
    ) -> crate::FunctionBody {
        let value = self.with_new_scope(|this| this.lower_function_block(&node.statement_list));

        crate::FunctionBody {
            parameter_list,
            value,
        }
    }

    fn lower_block_expression(&mut self) {
        //
    }

    fn with_new_scope<T, F: FnOnce(&mut Self) -> T>(&mut self, f: F) -> T {
        let is_in_loop_condition = self.is_in_loop_condition;
        self.is_in_loop_condition = false;

        let ret = f(self);

        self.is_in_loop_condition = is_in_loop_condition;

        ret
    }
}

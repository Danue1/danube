use super::*;

impl crate::HirContext {
    pub(super) fn lower_program(
        mut self,
        program: &Attributed<danube_parse::ast::ProgramNode>,
    ) -> crate::Program {
        let feature_list = self.lower_feature_list(&program.attribute_list);

        self.visit_item_list(&program.node.item_list);

        crate::Program {
            feature_list,
            items: self.items,
        }
    }

    fn lower_feature_list(&mut self, feature_list: &[AttributeNode]) -> Vec<crate::Feature> {
        feature_list
            .iter()
            .map(|feature| self.lower_feature(feature))
            .collect()
    }

    fn lower_feature(&mut self, feature: &AttributeNode) -> crate::Feature {
        std::todo!();
    }
}

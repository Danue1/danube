use crate::*;
use danube_hir::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct HirContext {
    pub resolver: Resolver,
    pub items: BTreeMap<ItemId, Item>,
    pub function_bodies: BTreeMap<FunctionBodyId, FunctionBody>,
    pub modules: BTreeMap<ItemId, Module>,
    pub current_module: ItemId,
    pub current_scope: Scope,
    pub is_in_loop_condition: bool,
}

impl Default for HirContext {
    fn default() -> Self {
        let mut resolver = Resolver::default();
        let current_module = resolver.next_id().into();

        HirContext {
            resolver,
            current_module,
            ..Default::default()
        }
    }
}

impl From<HirContext> for Crate {
    fn from(
        HirContext {
            items,
            function_bodies,
            modules,
            ..
        }: HirContext,
    ) -> Self {
        Crate {
            items,
            function_bodies,
            modules,
        }
    }
}

impl HirContext {
    pub fn next_id(&mut self) -> Id {
        self.resolver.next_id()
    }

    pub fn get_item(&self, id: &ItemId) -> Option<&Item> {
        self.items.get(id)
    }

    pub fn get_function_body(&self, id: &FunctionBodyId) -> Option<&FunctionBody> {
        self.function_bodies.get(id)
    }
}

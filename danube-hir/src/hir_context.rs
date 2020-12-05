use crate::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct HirContext {
    pub(crate) resolver: Resolver,
    pub(crate) items: BTreeMap<Id, Item>,
    pub(crate) bodies: BTreeMap<BodyId, FunctionBody>,
    pub(crate) is_in_loop_condition: bool,
}

impl HirContext {
    pub(super) fn next_id(&mut self) -> crate::Id {
        self.resolver.next_id()
    }

    pub(super) fn find_item(&self, id: &Id) -> Option<&Item> {
        self.items.get(id)
    }
}

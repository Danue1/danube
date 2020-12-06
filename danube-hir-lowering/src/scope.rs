use danube_hir::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Scope {
    pub parent: Option<Box<Scope>>,
    pub items: BTreeMap<String, ItemId>,
    pub variables: BTreeMap<String, VariableId>,
}

impl Scope {
    pub fn has_item(&self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    pub fn get_item(&self, key: &str) -> Option<&ItemId> {
        if let Some(id) = self.items.get(key) {
            Some(id)
        } else if let Some(symbol_table) = &self.parent {
            symbol_table.get_item(key)
        } else {
            None
        }
    }
}

impl Scope {
    pub fn has_variable(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    pub fn get_variable(&self, key: &str) -> Option<&VariableId> {
        if let Some(id) = &self.variables.get(key) {
            Some(id)
        } else if let Some(symbol_table) = &self.parent {
            symbol_table.get_variable(key)
        } else {
            None
        }
    }
}

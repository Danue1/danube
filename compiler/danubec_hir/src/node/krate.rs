use super::Definition;
use crate::HirId;
use std::collections::HashMap;

pub struct Krate {
    pub definitions: HashMap<HirId, Definition>,
}

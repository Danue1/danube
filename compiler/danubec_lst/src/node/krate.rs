use super::Root;
use std::collections::HashMap;

pub struct Krate {
    root: Root,
    submodules: HashMap<String, Module>,
}

pub struct Module {
    root: Root,
    submodules: HashMap<String, Module>,
}

impl Krate {
    #[inline]
    pub fn new((root, submodules): (Root, HashMap<String, Module>)) -> Self {
        Self { root, submodules }
    }

    #[inline]
    pub fn root(&self) -> Root {
        self.root.clone()
    }

    #[inline]
    pub fn modules(&self) -> &HashMap<String, Module> {
        &self.submodules
    }
}

impl Module {
    #[inline]
    pub fn new((root, submodules): (Root, HashMap<String, Module>)) -> Self {
        Self { root, submodules }
    }

    #[inline]
    pub fn root(&self) -> Root {
        self.root.clone()
    }

    #[inline]
    pub fn modules(&self) -> &HashMap<String, Module> {
        &self.submodules
    }
}

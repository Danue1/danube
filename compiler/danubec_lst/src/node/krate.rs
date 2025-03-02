use super::Root;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Krate {
    root: Root,
    submodules: HashMap<String, Module>,
}

#[derive(Debug)]
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
    pub fn modules(&self) -> impl Iterator<Item = (&str, &Module)> {
        self.submodules.iter().map(|(k, v)| (k.as_str(), v))
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
    pub fn submodules(&self) -> impl Iterator<Item = (&str, &Module)> {
        self.submodules.iter().map(|(k, v)| (k.as_str(), v))
    }
}

#![warn(clippy::all)]
#![allow(unused)]

mod definition;

use definition::*;

use danubec_middle::ast::{self, Visitor};

pub fn resolve(root: ast::Root) {
    let mut visitor = DefinitionCollection::new();
    visitor.visit_root(root);
}

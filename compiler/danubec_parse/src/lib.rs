#![warn(clippy::all)]

#[macro_use]
pub mod context;

#[macro_use]
mod tokens;

mod node;
mod pratt;

pub use context::*;

use pratt::*;

use danubec_lex::Lex;
use danubec_lst::{DefinitionKind, Krate, Module, Root};
use danubec_middle::EntryKind;
use danubec_syntax::{AstNode, SyntaxNode};
use std::{collections::HashMap, path::PathBuf};

pub fn parse_crate(path: PathBuf, kind: EntryKind) -> Krate {
    let path = path.join("src");
    let entry = match kind {
        EntryKind::Lib => path.join("lib.dnb"),
        EntryKind::Main => path.join("main.dnb"),
    };
    let source = std::fs::read_to_string(&entry).expect("ICE: failed to read file");

    Krate::new(parse_submodule(path, &source))
}

fn parse_submodule(path: PathBuf, source: &str) -> (Root, HashMap<String, Module>) {
    let mut context = Context::new();
    let mut lex = Lex::new(&source);
    context.root(&mut lex);

    let node = SyntaxNode::new_root(context.finish());
    let root = Root::cast(node).expect("ICE: root not found");

    let mut submodules = HashMap::new();
    for definition in root.definitions() {
        let name = match definition.kind() {
            Some(DefinitionKind::Module(definition)) if definition.semicolon().is_some() => {
                let name = definition.identifier().expect("ICE: module name not found");
                name.to_string()
            }
            _ => continue,
        };
        let file = match path.join(format!("{}.dnb", name)) {
            path if path.is_file() => path,
            _ => path.join(format!("{}/mod.dnb", name)),
        };
        let source = std::fs::read_to_string(&file).expect("ICE: failed to read file");
        let entry = path.join(&name);
        submodules.insert(name, Module::new(parse_submodule(entry, &source)));
    }

    (root, submodules)
}

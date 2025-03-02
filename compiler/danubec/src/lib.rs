#![warn(clippy::all)]

use danubec_compile::{CompileConfig, EntryKind};
use std::{collections::HashMap, path::PathBuf};

pub fn build(working_directory: PathBuf, crates: HashMap<String, EntryKind>) {
    let config = CompileConfig {
        working_directory,
        crates,
    };
    config.compile();
}

#[test]
fn test_compile() {
    let working_directory = std::env::current_dir()
        .expect("failed to get current directory")
        .join("../../library")
        .canonicalize()
        .expect("failed to canonicalize path");
    let crates = HashMap::from_iter([("core".to_string(), EntryKind::Lib)]);
    build(working_directory, crates);
}

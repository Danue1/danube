#![warn(clippy::all)]

use danubec_compile::CompileConfig;
use danubec_diagnostic::Diagnostic;
use danubec_middle::EntryKind;
use danubec_symbol::Symbol;
use std::{collections::HashMap, path::PathBuf};

pub fn build(
    working_directory: PathBuf,
    entry: String,
    crates: HashMap<String, EntryKind>,
) -> Result<(), Diagnostic> {
    let crates = crates
        .into_iter()
        .map(|(k, v)| (Symbol::new(&k), v))
        .collect();
    let config = CompileConfig {
        working_directory,
        entry: Symbol::new(&entry),
        crates,
    };
    config.compile()?;

    Ok(())
}

#[test]
fn test_compile() {
    let working_directory = std::env::current_dir()
        .expect("failed to get current directory")
        .join("../../library")
        .canonicalize()
        .expect("failed to canonicalize path");
    let crates = HashMap::from_iter([("core".to_string(), EntryKind::Lib)]);
    build(working_directory, "core".to_string(), crates).unwrap();
}

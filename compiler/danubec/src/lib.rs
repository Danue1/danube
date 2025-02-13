#![warn(clippy::all)]

use danubec_compile::CompileConfig;

pub fn compile(working_directory: String, crates: Vec<String>) {
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
        .expect("failed to canonicalize path")
        .to_string_lossy()
        .to_string();
    let crates = vec!["core".to_string()];
    compile(working_directory, crates);
}

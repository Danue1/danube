use crate::{ConfigBuilder, Context};
use std::path::PathBuf;

#[test]
fn valid_std_path() {
    let config = ConfigBuilder::new()
        .std_path(PathBuf::from("../../danube-library/danube-std"))
        .build();
    let context = Context::new(config);

    assert!(context.diagnostics.is_empty());
}

#[test]
fn invalid_std_path() {
    let config = ConfigBuilder::new()
        .std_path(PathBuf::from("invalid"))
        .build();
    let context = Context::new(config);

    assert_eq!(
        context.diagnostics.to_string(),
        "error: Source path 'invalid/src/lib.dnb' does not found"
    );
}

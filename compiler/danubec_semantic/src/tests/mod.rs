use crate::{Context, semantic};

#[test]
fn all_files() {
    let context = Context {
        path: std::path::PathBuf::from("src/tests/fixtures/lib.dnb"),
    };
    let (fs, table, symbols, diagnostic) = semantic(context);

    insta::assert_debug_snapshot!(fs);
    insta::assert_debug_snapshot!(table);
    insta::assert_debug_snapshot!(symbols);
    insta::assert_debug_snapshot!(diagnostic);
}

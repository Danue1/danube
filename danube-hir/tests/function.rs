use danube_hir::*;

#[test]
fn function() {
    let source = r#"
        fn foo() {
            print("Hello, World!");
        }
    "#;
    let program = source.parse().unwrap();

    assert_eq!(
        hir(&program),
        Program {
            feature_list: Default::default(),
            items: Default::default()
        }
    );
}

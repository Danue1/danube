use crate::{grammar::krate, lower::lower_krate};
use danubec_diagnostic::Diagnostic;
use danubec_syntax::AstNode;

fn parse(source: &str) -> (Option<danubec_ast::Krate>, Diagnostic) {
    let diagnostic = Diagnostic::new();
    let (node, mut diagnostic) = crate::parse(source, diagnostic, krate);
    let node =
        danubec_syntax::Krate::cast(node).and_then(|node| lower_krate(node, &mut diagnostic).ok());

    (node, diagnostic)
}

#[test]
fn top_level_attribute() {
    let source = r#"
#![a("value")]
#![a(key)]
#![a(key1, key2)]
#![a(key = "value")]
#![a::b(key = "value")]
"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn attribute() {
    let source = r#"
#[a("value")]
struct Foo;"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn zero_sized_struct() {
    let source = r#"
struct Foo;"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn named_struct() {
    let source = r#"
struct Foo { }
struct Bar {
    a: i32,
    b: String,
}"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn unnamed_struct() {
    let source = r#"
struct Foo();
struct Bar(());
struct Baz(i32, String);"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn mixed_enum() {
    let source = r#"
enum Foo {}
enum Bar {
    A,
    B { a: i32, b: String },
    C(i32, String),
    D = 42,
}"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn module() {
    let source = r#"
mod foo;
mod bar {
    struct Foo;
    enum Bar {
        A,
        B { a: i32, b: String },
        C(i32, String),
        D = 42,
    }
}"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn constant() {
    let source = r#"
const FOO: i32 = 42;
const BAR: str = "Hello, world!";
"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn static_variable() {
    let source = r#"
static FOO: i32 = 42;
static BAR: str = "Hello, world!";"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn type_alias() {
    let source = r#"
type Foo = i32;
type Bar = (i32, String);"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn function() {
    let source = r#"
fn foo();
fn bar() {}
fn baz(a: i32, b: String) -> (i32, String) {
    (a, b)
}"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn trait_definition() {
    let source = r#"
trait Foo { }
trait Bar {
    fn foo();
    const BAR: i32;
    type Baz;
}"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn anonymous_implement() {
    let source = r#"
impl Foo { }
impl Foo {
    fn foo() {}
    const BAR: i32 = 42;
    type Baz = i32;
}"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn targeted_implement() {
    let source = r#"
impl Foo for Bar { }
impl Foo for Bar {
    fn foo() {}
    const BAR: i32 = 42;
    type Baz = i32;
}"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn use_definition() {
    let source = r#"
use {};
use {{},*,foo,bar as baz,::{},::*,::foo,::bar as baz,};
use *;
use foo;
use foo as bar;
use foo::{};
use foo::{*,foo,bar as baz,{},::*,::foo,::bar as baz,::{}};

use ::{};
use ::{{},*,foo,bar as baz,::{},::*,::foo,::bar as baz,};
use ::*;
use ::foo;
use ::foo as bar;
use ::foo::{};
use ::foo::{*,foo,bar as baz,{},::*,::foo,::bar as baz,::{}};
"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn unary_expression() {
    let source = r#"
fn foo() {
    -42;
    !true;
    ~0;
}
"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

#[test]
fn primary_expression() {
    let source = r#"
fn foo() {
    break;
    continue;
    return;
    for i in expr {}
    while true {}
    loop {}
    if true {}
    if true {} else {}
    match 42 {
        0 => {},
        1 | 2 => {},
        foo @ 3 => {},
        _ => {},
    }
    let x;
    let x: i32;
    let x = 42;
    let x: i32 = 42;
    [0, 1, 2];
    (42, "hello", true);
    { 42 };
    foo();
    foo.bar;
    foo.bar();
    foo.bar.baz;
    foo.bar().baz();
    foo[0];
    foo += 1;
}"#;
    let (node, diagnostic) = parse(&source);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(node);
}

use crate::parse;
use danubec_diagnostic::Diagnostic;

#[test]
fn top_level_attribute() {
    let source = r#"
#![a("value")]
#![a(key)]
#![a(key1, key2)]
#![a(key = "value")]
#![a::b(key = "value")]"#;
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
}

#[test]
fn attribute() {
    let source = r#"
#[a("value")]
struct Foo;"#;
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
}

#[test]
fn zero_sized_struct() {
    let source = r#"
struct Foo;"#;
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
}

#[test]
fn named_struct() {
    let source = r#"
struct Foo { }
struct Bar {
    a: i32,
    b: String,
}"#;
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
}

#[test]
fn unnamed_struct() {
    let source = r#"
struct Foo();
struct Bar(());
struct Baz(i32, String);"#;
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
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
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
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
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
}

#[test]
fn constant() {
    let source = r#"
const FOO: i32 = 42;
const BAR: str = "Hello, world!";"#;
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
}

#[test]
fn static_variable() {
    let source = r#"
static FOO: i32 = 42;
static BAR: str = "Hello, world!";"#;
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
}

#[test]
fn type_alias() {
    let source = r#"
type Foo = i32;
type Bar = (i32, String);"#;
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
}

#[test]
fn function() {
    let source = r#"
fn foo();
fn bar() {}
fn baz(a: i32, b: String) -> (i32, String) {
    (a, b)
}"#;
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    dbg!(&diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
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
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
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
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
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
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
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
    let diagnostic = Diagnostic::new();
    let (events, diagnostic) = parse(&source, diagnostic);

    dbg!(&diagnostic);

    assert!(diagnostic.is_empty());
    insta::assert_debug_snapshot!(events);
}

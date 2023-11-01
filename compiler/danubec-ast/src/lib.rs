#[derive(Debug)]
pub struct Ast {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    // use std::io;
    Use(UseItem),

    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    Struct(StructItem),

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    Enum(EnumItem),

    // fn main() {
    //     println!("Hello, world!");
    // }
    Function(FunctionItem),

    // mod foo {
    //     fn bar() {}
    // }
    Module(ModuleItem),

    // impl Foo for Bar {
    //     fn baz() {}
    // }
    Implement(ImplementItem),
}

#[derive(Debug)]
pub struct UseItem {
    pub path: Vec<String>,
}

#[derive(Debug)]
pub struct StructItem {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug)]
pub struct EnumItem {
    pub name: String,
    pub variants: Vec<Variant>,
}

#[derive(Debug)]
pub struct Variant {
    pub name: String,
    pub ty: Option<Type>,
}

#[derive(Debug)]
pub struct FunctionItem {
    pub name: String,
    pub params: Vec<Param>,
    pub ret: Option<Type>,
    pub body: Vec<Expression>,
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug)]
pub struct ModuleItem {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct ImplementItem {
    pub ty: Type,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Type {
    Ident(String),
    Tuple(Vec<Type>),
    Fn(Vec<Type>, Box<Type>),
    Array(Box<Type>),
    Ptr(Box<Type>),
    Ref(Box<Type>),
    Mut(Box<Type>),
    Never,
    Unit,
}

#[derive(Debug)]
pub enum Expression {
    //
}

impl Ast {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
}

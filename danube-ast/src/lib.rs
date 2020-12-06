#![warn(clippy::all)]

use std::collections::HashMap;

pub type Module = Attributed<ModuleNode>;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Attributed<T: Sized> {
    pub attribute_list: Vec<AttributeNode>,
    pub node: T,
}

impl<T: Sized> std::ops::Deref for Attributed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AttributeNode {
    pub path: PathNode,
    pub args: HashMap<String, Option<LiteralKind>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PathNode {
    pub ident_list: Vec<IdentNode>,
}

#[macro_export]
macro_rules! path {
    () => {
        PathNode {
            ident_list: vec![],
        }
    };
    ($($expr:expr),+) => {
        PathNode {
            ident_list: vec![$($expr,)+],
        }
    };
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdentNode {
    pub raw: String,
}

#[macro_export]
macro_rules! ident {
    ($expr:expr) => {
        IdentNode { raw: $expr }
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, PartialEq)]
pub struct ModuleNode {
    pub item_list: Vec<Item>,
}

pub type Item = Attributed<ItemKind>;

#[derive(Debug, PartialEq, Clone)]
pub enum ItemKind {
    Function(FunctionNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    pub ident: IdentNode,
    pub parameter_list: Vec<FunctionParametertNode>,
    pub return_type: Option<TypeKind>,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParametertNode {
    pub label: IdentNode,
    pub argument_label: IdentNode,
    pub ty: Option<TypeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternKind {
    Placeholder,
    Path(PathNode),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    Path(PathNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockNode {
    pub statement_list: Vec<StatementKind>,
}

#[macro_export]
macro_rules! block {
    () => {
        BlockNode {
            statement_list: Default::default(),
        }
    };
    ($($expr:expr),+) => {
        BlockNode {
            statement_list: vec![$($expr,)+],
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Item(Box<Item>),
    Let(Box<LetNode>),
    Expression(ExpressionKind),
    Semicolon,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetNode {
    pub pattern: PatternKind,
    pub ty: Option<TypeKind>,
    pub value: Option<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionKind {
    Literal(LiteralKind),
}

#![warn(clippy::all)]

use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Crate {
    pub items: BTreeMap<ItemId, Item>,
    pub function_bodies: BTreeMap<FunctionBodyId, FunctionBody>,
    pub modules: BTreeMap<ItemId, Module>,
}

#[derive(Debug, PartialEq)]
pub struct Module {
    pub feature_list: Vec<Feature>,
    pub items: BTreeMap<Id, Item>,
}

#[derive(Debug, PartialEq)]
pub struct Feature {
    pub path: Path,
}

#[derive(Debug, PartialEq)]
pub struct Item {
    pub ident: Ident,
    pub attribute_list: Vec<Attribute>,
    pub kind: ItemKind,
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub path: Path,
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum ItemKind {
    Function(FunctionDeclaration),
}

#[derive(Debug, PartialEq)]
pub enum ExpressionKind {
    Literal(LiteralKind),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    Path(Path),
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub argument_list: Vec<FunctionArgument>,
    pub return_type: FunctionReturnTypeKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionArgument {
    pub ident: Ident,
    pub ty: TypeKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParameter {
    pub ident: Ident,
    pub ty: TypeKind,
}

#[derive(Debug, PartialEq)]
pub enum FunctionReturnTypeKind {
    Unit,
    Return(Ty),
}

#[derive(Debug, PartialEq)]
pub struct FunctionBody {
    pub parameter_list: Vec<FunctionParameter>,
    pub value: ExpressionKind,
}

#[derive(Debug, PartialEq)]
pub struct Ty {
    pub id: Id,
    pub kind: Box<TypeKind>,
}

#[derive(Debug, PartialEq)]
pub enum StatementKind {
    Item(Item),
    Let(Let),
    Expression(ExpressionKind),
}

#[derive(Debug, PartialEq)]
pub struct Let {
    pub pattern: PatternKind,
    pub ty: Option<TypeKind>,
    pub value: Option<ExpressionKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternKind {
    Placeholder,
    Path(Path),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub resolve: ResolveKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ResolveKind {
    Local(Id),
    Primitive(PrimitiveKind),
    Error,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimitiveKind {
    Int,
    Float,
    String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident {
    pub name: String,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Default)]
pub struct Id(u32);

impl Id {
    pub fn from_usize(id: usize) -> Self {
        Id(id as u32)
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

macro_rules! id {
    ($($ident:ident),+) => {
        $(
            #[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Default)]
            pub struct $ident(Id);

            impl From<Id> for $ident {
                fn from(id: Id) -> Self {
                    $ident(id)
                }
            }
        )+
    };
}

id![ItemId, FunctionBodyId, VariableId];

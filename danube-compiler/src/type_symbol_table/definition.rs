use danube_parser::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct TypeSymbolTable {
  pub name: String,
  pub symbol_tables: HashMap<String, TypeSymbolKind>,
}

impl TypeSymbolTable {
  pub(super) fn new(name: &str) -> Self {
    TypeSymbolTable {
      name: name.to_owned(),
      symbol_tables: Default::default(),
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeSymbolKind {
  Module(Module),
  Primitive,
  NamedStruct(NamedStruct),
  UnnamedStruct(UnnamedStruct),
  Enum(Enum),
  TypeAlias(TypeAlias),
  Function(Function),
  Trait(Trait),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Module {
  pub fields: HashMap<String, TypeSymbolKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Primitive {
  pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamedStruct {
  pub fields: HashMap<String, TypeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnnamedStruct {
  pub fields: Vec<TypeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Enum {
  pub variants: HashMap<String, Option<TypeKind>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAlias {
  pub kind: TypeKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
  pub argument_list: Vec<(String, TypeKind)>,
  pub return_type: Option<TypeKind>,
  pub items: HashMap<String, TypeSymbolKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Trait {
  pub items: HashMap<String, TraitFunction>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitFunction {
  pub argument_list: Vec<(String, TypeKind)>,
  pub return_type: Option<TypeKind>,
  pub items: HashMap<String, TypeSymbolKind>,
}

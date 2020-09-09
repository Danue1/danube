use danube_parser::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct SymbolTable {
  pub name: String,
  pub types: HashMap<String, TypeSymbolKind>,
  pub variables: HashMap<String, VariableSymbolKind>,
}

impl SymbolTable {
  pub(super) fn new(name: &str) -> Self {
    SymbolTable {
      name: name.to_owned(),
      types: Default::default(),
      variables: Default::default(),
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeSymbolKind {
  Module(ModuleSymbol),
  Primitive,
  NamedStruct(NamedStructSymbol),
  UnnamedStruct(UnnamedStructSymbol),
  Enum(EnumSymbol),
  TypeAlias(TypeAliasSymbol),
  Trait(TraitSymbol),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleSymbol {
  pub fields: HashMap<String, TypeSymbolKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Primitive {
  pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamedStructSymbol {
  pub fields: HashMap<String, TypeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnnamedStructSymbol {
  pub fields: Vec<TypeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumSymbol {
  pub variants: HashMap<String, Option<TypeKind>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAliasSymbol {
  pub kind: TypeKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitSymbol {
  pub items: HashMap<String, TraitItemSymbolKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TraitItemSymbolKind {
  OutputType(TraitItemOutputTypeSymbol),
  Constant(TraitItemConstantSymbol),
  Function(TraitItemFunctionSymbol),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitItemOutputTypeSymbol {
  pub ty: Option<TypeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitItemConstantSymbol {
  pub ty: TypeKind,
  pub default_value: Option<LiteralKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraitItemFunctionSymbol {
  pub arguments: HashMap<String, TypeKind>,
  pub return_type: Option<TypeKind>,
  pub items: HashMap<String, TypeSymbolKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableSymbolKind {
  Constant(ConstantSymbol),
  Static(StaticSymbol),
  Let(LetSymbol),
  Function(FunctionSymbol),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantSymbol {
  pub name: String,
  pub ty: TypeKind,
  pub value: LiteralKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StaticSymbol {
  pub name: String,
  pub ty: TypeKind,
  pub value: LiteralKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetSymbol {
  pub name: String,
  pub is_mutable: bool,
  pub value: LiteralKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionSymbol {
  pub argument_list: Vec<(String, TypeKind)>,
  pub return_type: Option<TypeKind>,
  pub items: HashMap<String, TypeSymbolKind>,
}

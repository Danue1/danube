use crate::ErrorKind;
use danube_parser::*;
use std::collections::HashMap;

pub fn create_type_symbol_table(node: &ProgramNode) -> TypeSymbolResult<TypeSymbolTable> {
  let mut type_symbol_table = TypeSymbolTable::new("root");
  type_symbol_table.scan_program_node(node)?;

  Ok(type_symbol_table)
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeSymbolTable {
  pub name: String,
  pub symbol_tables: HashMap<String, TypeSymbolKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeSymbolKind {
  NamedStruct(NamedStruct),
  UnnamedStruct(UnnamedStruct),
  Enum(Enum),
  TypeAlias(TypeAlias),
  Function(Function),
  Trait(Trait),
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

impl TypeSymbolTable {
  fn new(name: &str) -> Self {
    TypeSymbolTable {
      name: name.to_owned(),
      symbol_tables: Default::default(),
    }
  }
}

type TypeSymbolResult<T = ()> = Result<T, ErrorKind>;

impl TypeSymbolTable {
  fn scan_program_node(&mut self, node: &ProgramNode) -> TypeSymbolResult {
    for feature in node.feature_list.iter() {
      self.scan_feature_node(feature)?;
    }
    for item in node.item_list.iter() {
      self.scan_item_node(item)?;
    }

    Ok(())
  }

  fn scan_feature_node(&mut self, _node: &FeatureNode) -> TypeSymbolResult {
    Ok(())
  }

  fn scan_module_node(&mut self, node: &ModuleNode) -> TypeSymbolResult {
    for ref item in node.item_list.iter() {
      self.scan_item_node(item)?;
    }

    Ok(())
  }

  fn scan_item_node(&mut self, node: &ItemNode) -> TypeSymbolResult {
    match &node.kind {
      ItemKind::Use(node) => self.scan_use_node(node),
      ItemKind::Module(node) => self.scan_module_node(node),
      ItemKind::Struct(node) => self.scan_struct_node(node),
      ItemKind::Enum(node) => self.scan_enum_node(node),
      ItemKind::Function(node) => self.scan_function_node(node),
      ItemKind::TypeAlias(node) => self.scan_type_alias_node(node),
      ItemKind::Trait(node) => self.scan_trait_node(node),
      _ => Ok(()),
    }
  }

  fn scan_use_node(&mut self, _node: &UseNode) -> TypeSymbolResult {
    Ok(())
  }

  fn scan_struct_node(&mut self, node: &StructNode) -> TypeSymbolResult {
    let ident = &node.ident;
    match &node.fields {
      StructFieldsKind::Named(node) => {
        let mut fields: HashMap<String, TypeKind> = Default::default();
        for (ident, type_kind) in node.node_list.iter() {
          if fields.contains_key(&ident.raw) {
            return Err(ErrorKind::Message(
              "Can not declare duplicated field name.".to_owned(),
            ));
          }
          fields.insert(ident.raw.to_owned(), type_kind.clone());
        }
        let kind = TypeSymbolKind::NamedStruct(NamedStruct { fields });
        self.add_symbol(&ident.raw, kind)
      }
      StructFieldsKind::Unnamed(node) => {
        let kind = TypeSymbolKind::UnnamedStruct(UnnamedStruct {
          fields: node.node_list.clone(),
        });
        self.add_symbol(&ident.raw, kind)
      }
    }
  }

  fn scan_enum_node(&mut self, node: &EnumNode) -> TypeSymbolResult {
    let mut variants: HashMap<String, Option<TypeKind>> = Default::default();
    for node in node.variant_list.iter() {
      if variants.contains_key(&node.ident.raw) {
        return Err(ErrorKind::Message(
          "Can not declare duplicated variant.".to_owned(),
        ));
      }
      variants.insert(node.ident.raw.to_owned(), node.ty.clone());
    }
    let kind = TypeSymbolKind::Enum(Enum { variants: variants });
    self.add_symbol(&node.ident.raw, kind)
  }

  fn scan_function_node(&mut self, node: &FunctionNode) -> TypeSymbolResult {
    let arguments: HashMap<String, TypeKind> = Default::default();
    for argument in node.argument_list.iter() {
      if arguments.contains_key(&argument.ident.raw) {
        return Err(ErrorKind::Message(
          "Can not declare duplicated argument name.".to_owned(),
        ));
      }
    }

    let mut type_symbol_table = TypeSymbolTable::new(&node.ident.raw);
    for kind in node.block.statement_list.iter() {
      type_symbol_table.scan_statement_kind(kind)?;
    }

    let kind = TypeSymbolKind::Function(Function {
      argument_list: arguments.into_iter().collect(),
      return_type: node.return_type.clone(),
      items: type_symbol_table.symbol_tables,
    });
    self.add_symbol(&node.ident.raw, kind)?;

    Ok(())
  }

  fn scan_type_alias_node(&mut self, node: &TypeAliasNode) -> TypeSymbolResult {
    let kind = TypeSymbolKind::TypeAlias(TypeAlias {
      kind: node.ty.clone(),
    });
    self.add_symbol(&node.ident.raw, kind)
  }

  fn scan_trait_node(&mut self, node: &TraitNode) -> TypeSymbolResult {
    let mut items: HashMap<String, TraitFunction> = Default::default();
    for item in node.item_list.iter() {
      match item {
        TraitItemKind::Function(node) => {
          if items.contains_key(&node.ident.raw) {
            return Err(ErrorKind::Message(
              "Can not declare duplicated function name.".to_owned(),
            ));
          }
          let trait_function = self.scan_trait_item_function_node(node)?;
          items.insert(node.ident.raw.to_owned(), trait_function);
        }
        _ => {}
      }
    }

    let kind = TypeSymbolKind::Trait(Trait { items });
    self.add_symbol(&node.ident.raw, kind)?;

    Ok(())
  }

  fn scan_statement_kind(&mut self, node: &StatementKind) -> TypeSymbolResult {
    match node {
      StatementKind::Item(node) => self.scan_item_node(node),
      _ => Ok(()),
    }
  }

  fn scan_trait_item_function_node(
    &mut self,
    node: &TraitItemFunctionNode,
  ) -> TypeSymbolResult<TraitFunction> {
    let arguments: HashMap<String, TypeKind> = Default::default();
    for argument in node.argument_list.iter() {
      if arguments.contains_key(&argument.ident.raw) {
        return Err(ErrorKind::Message(
          "Can not declare duplicated argument name.".to_owned(),
        ));
      }
    }

    let mut type_symbol_table = TypeSymbolTable::new(&node.ident.raw);
    if let Some(block) = &node.block {
      for kind in block.statement_list.iter() {
        type_symbol_table.scan_statement_kind(kind)?;
      }
    }

    let trait_function = TraitFunction {
      argument_list: arguments.into_iter().collect(),
      return_type: node.return_type.clone(),
      items: type_symbol_table.symbol_tables,
    };
    Ok(trait_function)
  }
}

impl TypeSymbolTable {
  fn add_symbol(&mut self, name: &str, kind: TypeSymbolKind) -> TypeSymbolResult {
    if self.symbol_tables.contains_key(name) {
      Err(ErrorKind::Message(
        "Can not declare duplicated type name.".to_owned(),
      ))
    } else {
      self.symbol_tables.insert(name.to_owned(), kind);
      Ok(())
    }
  }
}

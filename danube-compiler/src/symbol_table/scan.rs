use super::*;
use std::collections::HashMap;

impl SymbolTable {
  fn add_type(&mut self, name: &str, kind: TypeSymbolKind) -> SymbolTableResult {
    if self.types.contains_key(name) {
      Err(ErrorKind::Message(
        "Can not declare duplicated type name.".to_owned(),
      ))
    } else {
      self.types.insert(name.to_owned(), kind);
      Ok(())
    }
  }

  fn add_variable(&mut self, name: &str, kind: VariableSymbolKind) -> SymbolTableResult {
    if self.variables.contains_key(name) {
      Err(ErrorKind::Message(
        "Can not declare duplicated variable name.".to_owned(),
      ))
    } else {
      self.variables.insert(name.to_owned(), kind);
      Ok(())
    }
  }
}

impl SymbolTable {
  pub(super) fn scan_program_node(&mut self, node: &ProgramNode) -> SymbolTableResult {
    macro_rules! add_type {
      ($($name:expr,)+) => {
        $(let _ = self.add_type($name, TypeSymbolKind::Primitive); )+
      };
    }

    add_type!["bool", "int", "float", "str",];

    let mut type_symbol_table = SymbolTable::new("Entry");

    for feature in node.feature_list.iter() {
      type_symbol_table.scan_feature_node(feature)?;
    }

    for item in node.item_list.iter() {
      type_symbol_table.scan_item_node(item)?;
    }

    self.add_type(
      "Entry",
      TypeSymbolKind::Module(ModuleSymbol {
        fields: type_symbol_table.types,
      }),
    )?;

    Ok(())
  }

  fn scan_feature_node(&mut self, _node: &FeatureNode) -> SymbolTableResult {
    Ok(())
  }

  fn scan_module_node(&mut self, node: &ModuleNode) -> SymbolTableResult {
    for ref item in node.item_list.iter() {
      self.scan_item_node(item)?;
    }

    Ok(())
  }

  fn scan_item_node(&mut self, node: &ItemNode) -> SymbolTableResult {
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

  fn scan_use_node(&mut self, _node: &UseNode) -> SymbolTableResult {
    Ok(())
  }

  fn scan_struct_node(&mut self, node: &StructNode) -> SymbolTableResult {
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
        let kind = TypeSymbolKind::NamedStruct(NamedStructSymbol { fields });
        self.add_type(&ident.raw, kind)
      }
      StructFieldsKind::Unnamed(node) => {
        let kind = TypeSymbolKind::UnnamedStruct(UnnamedStructSymbol {
          fields: node.node_list.clone(),
        });
        self.add_type(&ident.raw, kind)
      }
    }
  }

  fn scan_enum_node(&mut self, node: &EnumNode) -> SymbolTableResult {
    let mut variants: HashMap<String, Option<TypeKind>> = Default::default();
    for node in node.variant_list.iter() {
      if variants.contains_key(&node.ident.raw) {
        return Err(ErrorKind::Message(
          "Can not declare duplicated variant.".to_owned(),
        ));
      }
      variants.insert(node.ident.raw.to_owned(), node.ty.clone());
    }
    let kind = TypeSymbolKind::Enum(EnumSymbol { variants: variants });
    self.add_type(&node.ident.raw, kind)
  }

  fn scan_function_node(&mut self, node: &FunctionNode) -> SymbolTableResult {
    let arguments: HashMap<String, TypeKind> = Default::default();
    for argument in node.argument_list.iter() {
      if arguments.contains_key(&argument.ident.raw) {
        return Err(ErrorKind::Message(
          "Can not declare duplicated argument name.".to_owned(),
        ));
      }
    }

    let mut type_symbol_table = SymbolTable::new(&node.ident.raw);
    for kind in node.block.statement_list.iter() {
      type_symbol_table.scan_statement_kind(kind)?;
    }

    let kind = VariableSymbolKind::Function(FunctionSymbol {
      argument_list: arguments.into_iter().collect(),
      return_type: node.return_type.clone(),
      items: type_symbol_table.types,
    });
    self.add_variable(&node.ident.raw, kind)?;

    Ok(())
  }

  fn scan_type_alias_node(&mut self, node: &TypeAliasNode) -> SymbolTableResult {
    let kind = TypeSymbolKind::TypeAlias(TypeAliasSymbol {
      kind: node.ty.clone(),
    });
    self.add_type(&node.ident.raw, kind)
  }

  fn scan_trait_node(&mut self, node: &TraitNode) -> SymbolTableResult {
    let mut symbol = TraitSymbol::new();
    for item in node.item_list.iter() {
      symbol.scan_kind(&item)?;
    }

    let kind = TypeSymbolKind::Trait(symbol);
    self.add_type(&node.ident.raw, kind)?;

    Ok(())
  }

  fn scan_statement_kind(&mut self, node: &StatementKind) -> SymbolTableResult {
    match node {
      StatementKind::Item(node) => self.scan_item_node(node),
      _ => Ok(()),
    }
  }
}

impl TraitSymbol {
  fn new() -> Self {
    TraitSymbol {
      items: Default::default(),
    }
  }

  fn add_variable(&mut self, name: &str, kind: TraitItemSymbolKind) -> SymbolTableResult {
    if self.items.contains_key(name) {
      Err(ErrorKind::Message(
        "Can not declare duplicated trait item name.".to_owned(),
      ))
    } else {
      self.items.insert(name.to_owned(), kind);
      Ok(())
    }
  }
}

impl TraitSymbol {
  fn scan_kind(&mut self, node: &TraitItemKind) -> SymbolTableResult {
    match node {
      TraitItemKind::OutputType(node) => self.scan_output_type_node(node),
      TraitItemKind::Constant(node) => self.scan_constant_node(node),
      TraitItemKind::Function(node) => self.scan_function_node(node),
    }
  }

  fn scan_output_type_node(&mut self, node: &TraitItemOutputTypeNode) -> SymbolTableResult {
    self.add_variable(
      &node.ident.raw,
      TraitItemSymbolKind::OutputType(TraitItemOutputTypeSymbol {
        ty: node.ty.clone(),
      }),
    )
  }

  fn scan_constant_node(&mut self, node: &TraitItemConstantNode) -> SymbolTableResult {
    self.add_variable(
      &node.ident.raw,
      TraitItemSymbolKind::Constant(TraitItemConstantSymbol {
        ty: node.ty.clone(),
        default_value: node.default_value.clone(),
      }),
    )
  }

  fn scan_function_node(&mut self, node: &TraitItemFunctionNode) -> SymbolTableResult {
    let mut arguments: HashMap<String, TypeKind> = Default::default();
    for argument in node.argument_list.iter() {
      if arguments.contains_key(&argument.ident.raw) {
        return Err(ErrorKind::Message(
          "Can not declare duplicated argument name.".to_owned(),
        ));
      }
      arguments.insert(argument.ident.raw.to_owned(), argument.ty.clone());
    }

    let mut type_symbol_table = SymbolTable::new(&node.ident.raw);
    if let Some(block) = &node.block {
      for kind in block.statement_list.iter() {
        type_symbol_table.scan_statement_kind(kind)?;
      }
    }

    let trait_function = TraitItemFunctionSymbol {
      arguments,
      return_type: node.return_type.clone(),
      items: type_symbol_table.types,
    };
    self.add_variable(
      &node.ident.raw,
      TraitItemSymbolKind::Function(trait_function),
    )
  }
}

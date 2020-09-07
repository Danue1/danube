mod block_node;
mod compound_assign_kind;
mod compound_assign_node;
mod constant_node;
mod expression_kind;
mod function_argument_node;
mod function_node;
mod ident_node;
mod item_kind;
mod item_node;
mod let_node;
mod literal_value_kind;
mod path_node;
mod pattern_kind;
mod program_node;
mod return_node;
mod statement_kind;
mod tuple_argument_node;
mod tuple_node;

use block_node::transpile_block_node;
use compound_assign_kind::transpile_compound_assign_kind;
use compound_assign_node::transpile_compound_assign_node;
use constant_node::transpile_constant_node;
use danube_parser::*;
use expression_kind::transpile_expression_kind;
use function_argument_node::transpile_function_argument_node;
use function_node::transpile_function_node;
use ident_node::transpile_ident_node;
use item_kind::transpile_item_kind;
use item_node::transpile_item_node;
use let_node::transpile_let_node;
use literal_value_kind::transpile_literal_value_kind;
use path_node::transpile_path_node;
use pattern_kind::transpile_pattern_kind;
use program_node::transpile_program_node;
use return_node::transpile_return_node;
use statement_kind::transpile_statement_kind;
use std::collections::HashSet;
use std::fmt::Write;
use tuple_argument_node::transpile_tuple_argument_node;
use tuple_node::transpile_tuple_node;

pub(super) fn transpile(node: &ProgramNode, config: &Config) -> String {
  let mut context = Context::new(config);
  transpile_program_node(node, &mut context);

  context.runtime
}

struct Context<'a> {
  runtime: String,
  batteries: HashSet<String>,
  config: &'a Config,
}

impl<'a> Context<'a> {
  fn new(config: &'a Config) -> Self {
    Context {
      runtime: Default::default(),
      batteries: Default::default(),
      config,
    }
  }

  fn new_instance(&self) -> Self {
    Context {
      runtime: Default::default(),
      batteries: Default::default(),
      config: self.config,
    }
  }
}

impl<'a> Context<'a> {
  fn add_battery(&mut self, battery: String) {
    self.batteries.insert(battery);
  }
}

impl<'a> std::fmt::Write for Context<'a> {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    self.runtime.push_str(s);
    Ok(())
  }
}

#[derive(Default)]
pub struct Config {
  pub language: Language,
}

pub enum Language {
  JavaScript,
  TypeScript,
}

impl Default for Language {
  fn default() -> Self {
    Self::JavaScript
  }
}

impl Language {
  fn is_javascript(&self) -> bool {
    match self {
      Self::JavaScript => true,
      Self::TypeScript => false,
    }
  }

  fn _is_typescript(&self) -> bool {
    !self.is_javascript()
  }
}

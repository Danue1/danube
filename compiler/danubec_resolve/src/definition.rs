use danubec_middle::{ast, hir, Environment, Namespace, Scope, ScopeKind};
use std::collections::HashMap;

#[derive(Debug)]
pub struct DefinitionCollection {
    pub definitions: HashMap<hir::DefId, hir::Definition>,
    pub bodies: HashMap<hir::BodyId, hir::Body>,

    pub environment: Environment,
}

impl DefinitionCollection {
    pub fn new() -> Self {
        DefinitionCollection {
            definitions: HashMap::new(),
            bodies: HashMap::new(),

            environment: Environment::new(),
        }
    }

    pub fn with_scope<F, T>(&mut self, kind: ScopeKind, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        self.environment.push(kind);
        let result = f(self);
        self.environment.pop();

        result
    }
}

impl ast::Visitor for DefinitionCollection {
    fn visit_module_definition(&mut self, node: ast::ModuleDefinition) {
        let def_id = hir::DefId::new();
        let definition = hir::Definition {
            name: hir::Ident::new(node.identifier().unwrap().to_string()),
            kind: hir::DefinitionKind::Module(hir::ModuleDef {
                definitions: vec![],
            }),
        };
        self.definitions.insert(def_id, definition);

        self.with_scope(ScopeKind::Module, |context| {
            ast::walk_module_definition(context, node);
        });
    }

    fn visit_struct_definition(&mut self, node: ast::StructDefinition) {
        let def_id = hir::DefId::new();
        let ident = hir::Ident::new(node.identifier().unwrap().to_string());
        let definition = hir::Definition {
            name: ident,
            kind: hir::DefinitionKind::Struct(hir::StructDef { kind: None }),
        };
        self.definitions.insert(def_id, definition);
        self.environment.define(Namespace::Type, ident, def_id);

        self.with_scope(ScopeKind::Struct, |context| {
            ast::walk_struct_definition(context, node);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::DefinitionCollection;
    use danubec_middle::ast::Visitor;
    use danubec_parse::parse;

    #[test]
    fn test_resolve() {
        let source = r#"
            mod foo {
                mod bar { }
            }

            mod baz {
                struct Baz;
            }
        "#;
        let root = parse(source);
        let mut collector = DefinitionCollection::new();
        collector.visit_root(root);
        dbg!(collector.definitions);
    }
}

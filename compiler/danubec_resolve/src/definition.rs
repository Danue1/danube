use danubec_middle::{ast, hir};
use std::collections::HashMap;

#[derive(Debug)]
pub struct DefinitionCollection {
    definitions: HashMap<hir::DefId, hir::Definition>,
}

pub fn resolve(root: &ast::Root) -> DefinitionCollection {
    let mut collection = DefinitionCollection {
        definitions: HashMap::new(),
    };
    let mut scope = vec![];
    for definition in &root.definitions {
        resolve_definition(definition, &mut scope, &mut collection);
    }

    collection
}

fn resolve_definition(
    definition: &ast::Definition,
    scope: &mut Vec<hir::DefId>,
    collection: &mut DefinitionCollection,
) {
    let def_id = hir::DefId::new();
    scope.push(def_id);

    let def = match &definition.kind {
        ast::DefinitionKind::Module {
            visibility,
            ident,
            definitions,
        } => {
            let mut scope = vec![];
            for definition in definitions {
                resolve_definition(definition, &mut scope, collection);
            }

            hir::Definition {
                kind: hir::DefinitionKind::Module(hir::ModuleDef {
                    ident: *ident,
                    definitions: scope,
                }),
            }
        }
        // ast::DefinitionKind::Struct {
        //     visibility,
        //     ident,
        //     type_parameters,
        //     predicates,
        //     kind,
        // } => hir::Definition {
        //     kind: hir::DefinitionKind::Struct(hir::StructDef {
        //         ident: Symbol::new(definition.identifier().unwrap().to_string().as_str()),
        //         kind: None,
        //     }),
        // },
        ast::DefinitionKind::Use { visibility, tree } => hir::Definition {
            kind: hir::DefinitionKind::Use(hir::UseDef {
                tree: resolve_use_tree(tree),
            }),
        },
        _ => return,
    };

    collection.definitions.insert(def_id, def);
}

fn resolve_use_tree(tree: &ast::UseTree) -> hir::UseTree {
    let path = resolve_path(&tree.path);
    let kind = match &tree.kind {
        Some(ast::UseTreeKind::Barrel) => Some(hir::UseTreeKind::Barrel),
        Some(ast::UseTreeKind::Alias(alias)) => {
            Some(hir::UseTreeKind::Ident(hir::UseTreeIdent { alias: *alias }))
        }
        Some(ast::UseTreeKind::Nested(trees)) => {
            let trees = trees.iter().map(resolve_use_tree).collect();
            Some(hir::UseTreeKind::Nested(hir::UseTreeNested { trees }))
        }
        None => None,
    };

    hir::UseTree { path, kind }
}

fn resolve_path(path: &ast::Path) -> hir::Path {
    hir::Path {
        segments: path
            .segments
            .iter()
            .map(|segment| hir::PathSegment {
                ident: segment.ident,
                type_arguments: vec![],
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use danubec_lst_lowering::lower_root;
    use danubec_parse::parse;

    #[test]
    fn test_resolve() {
        let source = r#"
            use bar::Baz;

            mod foo {
                mod bar { }
            }

            mod baz {
                struct Baz;
            }
        "#;
        let root = parse(source);
        let root = lower_root(root).unwrap();
        let collection = super::resolve(&root);

        dbg!(&collection);
    }
}

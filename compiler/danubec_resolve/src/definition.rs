use danubec_middle::{ast, hir};
use std::collections::HashMap;

#[derive(Debug)]
pub struct DefinitionCollection {
    definitions: HashMap<hir::DefId, hir::Definition>,
}

pub fn resolve(root: ast::Root) -> DefinitionCollection {
    let mut collection = DefinitionCollection {
        definitions: HashMap::new(),
    };
    let mut definitions = vec![];
    for definition in root.definitions() {
        resolve_definition(definition, &mut definitions, &mut collection);
    }

    collection
}

fn resolve_definition(
    definition: ast::Definition,
    scope: &mut Vec<hir::DefId>,
    collection: &mut DefinitionCollection,
) {
    let def_id = hir::DefId::new();
    scope.push(def_id);

    let def = match definition.kind() {
        Some(ast::DefinitionKind::Module(definition)) => {
            let mut definitions = vec![];
            for definition in definition.definitions() {
                resolve_definition(definition, &mut definitions, collection);
            }

            hir::Definition {
                kind: hir::DefinitionKind::Module(hir::ModuleDef {
                    ident: hir::Ident::new(definition.identifier().unwrap().to_string().as_str()),
                    definitions,
                }),
            }
        }
        Some(ast::DefinitionKind::Struct(definition)) => hir::Definition {
            kind: hir::DefinitionKind::Struct(hir::StructDef {
                ident: hir::Ident::new(definition.identifier().unwrap().to_string().as_str()),
                kind: None,
            }),
        },
        Some(ast::DefinitionKind::Use(definition)) => hir::Definition {
            kind: hir::DefinitionKind::Use(hir::UseDef {
                tree: resolve_use_tree(definition.tree().unwrap()),
            }),
        },
        _ => return,
    };

    collection.definitions.insert(def_id, def);
}

fn resolve_use_tree(tree: ast::UseTree) -> hir::UseTree {
    let path = resolve_path(tree.path().unwrap());
    let kind = match tree.kind() {
        Some(ast::UseTreeKind::Barrel(_)) => Some(hir::UseTreeKind::Barrel),
        Some(ast::UseTreeKind::Ident(tree)) => Some(hir::UseTreeKind::Ident(hir::UseTreeIdent {
            alias: hir::Ident::new(tree.identifier().unwrap().to_string().as_str()),
        })),
        Some(ast::UseTreeKind::Nested(tree)) => {
            Some(hir::UseTreeKind::Nested(hir::UseTreeNested {
                trees: tree.trees().map(resolve_use_tree).collect(),
            }))
        }
        None => None,
    };

    hir::UseTree { path, kind }
}

fn resolve_path(path: ast::Path) -> hir::Path {
    hir::Path {
        segments: path
            .segments()
            .map(|segment| hir::PathSegment {
                ident: hir::Ident::new(segment.identifier().unwrap().to_string().as_str()),
                type_arguments: vec![],
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use danubec_parse::parse;

    #[test]
    fn test_resolve() {
        let source = r#"
            use foo::bar::baz;

            mod foo {
                mod bar { }
            }

            mod baz {
                struct Baz;
            }
        "#;
        let root = parse(source);
        let collection = super::resolve(root);

        dbg!(&collection);
    }
}

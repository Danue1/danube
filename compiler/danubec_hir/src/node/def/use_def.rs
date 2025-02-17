use crate::Path;
use danubec_symbol::Symbol;

#[derive(Debug)]
pub struct UseDef {
    pub tree: UseTree,
}

#[derive(Debug)]
pub struct UseTree {
    pub path: Path,
    pub kind: Option<UseTreeKind>,
}

#[derive(Debug)]
pub enum UseTreeKind {
    Barrel,
    Ident(UseTreeIdent),
    Nested(UseTreeNested),
}

#[derive(Debug)]
pub struct UseTreeIdent {
    pub alias: Symbol,
}

#[derive(Debug)]
pub struct UseTreeNested {
    pub trees: Vec<UseTree>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Import {
    pub path: Path,
    pub kind: Option<ImportKind>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImportKind {
    Alias(Symbol),
    Barrel,
}

impl UseDef {
    pub fn flatten(&self) -> Vec<Import> {
        self.tree.flatten()
    }
}

impl UseTree {
    pub fn flatten(&self) -> Vec<Import> {
        let mut imports = vec![];

        match &self.kind {
            Some(UseTreeKind::Barrel) => {
                imports.push(Import {
                    path: self.path.clone(),
                    kind: Some(ImportKind::Barrel),
                });
            }
            Some(UseTreeKind::Ident(ident)) => {
                imports.push(Import {
                    path: self.path.clone(),
                    kind: Some(ImportKind::Alias(ident.alias.clone())),
                });
            }
            Some(UseTreeKind::Nested(nested)) => {
                for tree in &nested.trees {
                    for import in tree.flatten() {
                        imports.push(Import {
                            path: self.path.clone() + import.path.clone(),
                            kind: import.kind,
                        });
                    }
                }
            }
            None => {
                //
            }
        }

        imports
    }
}

#[test]
fn use_tree() {
    use crate::PathSegment;

    let tree = UseTree {
        path: Path {
            segments: vec![
                PathSegment {
                    ident: Symbol::new("foo"),
                    type_arguments: vec![],
                },
                PathSegment {
                    ident: Symbol::new("bar"),
                    type_arguments: vec![],
                },
            ],
        },
        kind: Some(UseTreeKind::Nested(UseTreeNested {
            trees: vec![
                UseTree {
                    path: Path {
                        segments: vec![PathSegment {
                            ident: Symbol::new("baz"),
                            type_arguments: vec![],
                        }],
                    },
                    kind: Some(UseTreeKind::Ident(UseTreeIdent {
                        alias: Symbol::new("qux"),
                    })),
                },
                UseTree {
                    path: Path {
                        segments: vec![PathSegment {
                            ident: Symbol::new("quux"),
                            type_arguments: vec![],
                        }],
                    },
                    kind: Some(UseTreeKind::Barrel),
                },
            ],
        })),
    };

    assert_eq!(
        tree.flatten(),
        vec![
            Import {
                path: Path {
                    segments: vec![
                        PathSegment {
                            ident: Symbol::new("foo"),
                            type_arguments: vec![],
                        },
                        PathSegment {
                            ident: Symbol::new("bar"),
                            type_arguments: vec![],
                        },
                        PathSegment {
                            ident: Symbol::new("baz"),
                            type_arguments: vec![],
                        },
                    ],
                },
                kind: Some(ImportKind::Alias(Symbol::new("qux"))),
            },
            Import {
                path: Path {
                    segments: vec![
                        PathSegment {
                            ident: Symbol::new("foo"),
                            type_arguments: vec![],
                        },
                        PathSegment {
                            ident: Symbol::new("bar"),
                            type_arguments: vec![],
                        },
                        PathSegment {
                            ident: Symbol::new("quux"),
                            type_arguments: vec![],
                        },
                    ],
                },
                kind: Some(ImportKind::Barrel),
            },
        ]
    );
}

use crate::{Ident, Path};

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
    pub alias: Ident,
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
    Alias(Ident),
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
                    ident: Ident::new("foo"),
                    type_arguments: vec![],
                },
                PathSegment {
                    ident: Ident::new("bar"),
                    type_arguments: vec![],
                },
            ],
        },
        kind: Some(UseTreeKind::Nested(UseTreeNested {
            trees: vec![
                UseTree {
                    path: Path {
                        segments: vec![PathSegment {
                            ident: Ident::new("baz"),
                            type_arguments: vec![],
                        }],
                    },
                    kind: Some(UseTreeKind::Ident(UseTreeIdent {
                        alias: Ident::new("qux"),
                    })),
                },
                UseTree {
                    path: Path {
                        segments: vec![PathSegment {
                            ident: Ident::new("quux"),
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
                            ident: Ident::new("foo"),
                            type_arguments: vec![],
                        },
                        PathSegment {
                            ident: Ident::new("bar"),
                            type_arguments: vec![],
                        },
                        PathSegment {
                            ident: Ident::new("baz"),
                            type_arguments: vec![],
                        },
                    ],
                },
                kind: Some(ImportKind::Alias(Ident::new("qux"))),
            },
            Import {
                path: Path {
                    segments: vec![
                        PathSegment {
                            ident: Ident::new("foo"),
                            type_arguments: vec![],
                        },
                        PathSegment {
                            ident: Ident::new("bar"),
                            type_arguments: vec![],
                        },
                        PathSegment {
                            ident: Ident::new("quux"),
                            type_arguments: vec![],
                        },
                    ],
                },
                kind: Some(ImportKind::Barrel),
            },
        ]
    );
}

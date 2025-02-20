use danubec_symbol::Symbol;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Environment<Definition> {
    scopes: Vec<Scope>,
    nodes: Vec<Node<Definition>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeIndex(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeIndex(usize);

#[derive(Debug)]
pub struct Node<Definition> {
    definition: Definition,
    scope: Option<ScopeIndex>,
}

#[derive(Debug)]
pub struct Scope {
    parent: Option<ScopeIndex>,
    symbols: HashMap<Symbol, NodeIndex>,
    imports: Vec<Import>,
}

#[derive(Debug)]
pub struct Import {
    path: Vec<Symbol>,
    kind: ImportKind,
}

#[derive(Debug)]
pub enum ImportKind {
    /// ```danube
    /// use foo::bar; // if None
    /// use foo::bar as baz; // if Some
    /// ```
    Named(Option<Symbol>),

    /// ```danube
    /// use foo::*;
    /// ```
    Glob,
}

#[derive(Debug)]
pub enum Error {
    DuplicatedSymbol,
}

impl<Definition> Environment<Definition> {
    pub const fn new() -> Self {
        Environment {
            scopes: vec![],
            nodes: vec![],
        }
    }

    pub fn add_scope(&mut self, parent: Option<ScopeIndex>) -> ScopeIndex {
        let scope_index = ScopeIndex(self.scopes.len());
        self.scopes.push(Scope {
            parent,
            symbols: HashMap::new(),
            imports: vec![],
        });

        scope_index
    }

    pub fn add_definition(
        &mut self,
        scope: ScopeIndex,
        symbol: Symbol,
        node: Node<Definition>,
    ) -> Result<NodeIndex, Error> {
        let node_index = NodeIndex(self.nodes.len());

        let scope = &mut self[scope];
        if scope.symbols.contains_key(&symbol) {
            return Err(Error::DuplicatedSymbol);
        }
        scope.symbols.insert(symbol, node_index);

        self.nodes.push(node);

        Ok(node_index)
    }

    #[inline]
    pub fn add_import_direct(&mut self, scope: ScopeIndex, path: Vec<Symbol>) {
        self[scope].imports.push(Import {
            path,
            kind: ImportKind::Named(None),
        });
    }

    #[inline]
    pub fn add_import_alias(&mut self, scope: ScopeIndex, path: Vec<Symbol>, alias: Symbol) {
        self[scope].imports.push(Import {
            path,
            kind: ImportKind::Named(Some(alias)),
        });
    }

    #[inline]
    pub fn add_import_glob(&mut self, scope: ScopeIndex, path: Vec<Symbol>) {
        self[scope].imports.push(Import {
            path,
            kind: ImportKind::Glob,
        });
    }

    pub fn resolve(&self, scope: ScopeIndex, path: &[Symbol]) -> Vec<Definition>
    where
        Definition: Copy,
    {
        if path.is_empty() {
            return vec![];
        }

        let mut definitions = HashSet::new();
        let mut visited = HashSet::new();

        self.resolve_path(scope, path, &mut definitions, &mut visited);

        definitions
            .into_iter()
            .map(|node| self[node].definition)
            .collect()
    }

    /// `local` -> `import` -> `parent`
    fn resolve_path(
        &self,
        current_scope: ScopeIndex,
        path: &[Symbol],
        nodes: &mut HashSet<NodeIndex>,
        visited: &mut HashSet<ScopeIndex>,
    ) {
        if visited.contains(&current_scope) {
            return;
        }

        let Some((name, rest)) = path.split_first() else {
            return;
        };

        // 1. Find nodes in local symbols.
        if let Some(&node) = self[current_scope].symbols.get(name) {
            if rest.is_empty() {
                nodes.insert(node);
                return;
            }
            if let Some(scope) = self[node].scope {
                return self.resolve_path(scope, rest, nodes, visited);
            }
        }

        // 2. Find nodes in imports.
        for import in &self[current_scope].imports {
            match import.kind {
                ImportKind::Named(alias) if import.last_name() == Some(name) => {
                    let Some((name_, rest_)) = import.path.split_first() else {
                        continue;
                    };
                    let Some(&node) = self[current_scope].symbols.get(name_) else {
                        continue;
                    };
                    if rest_.is_empty() {
                        nodes.insert(node);
                        continue;
                    }
                    if let Some(scope) = self[node].scope {
                        let rest: Vec<_> = rest_.iter().chain(rest.iter()).cloned().collect();
                        self.resolve_path(scope, &rest, nodes, visited);
                        continue;
                    }
                }
                ImportKind::Glob => {
                    let Some((name_, rest_)) = import.path.split_first() else {
                        continue;
                    };
                    let Some(&node) = self[current_scope].symbols.get(name_) else {
                        continue;
                    };
                    visited.insert(current_scope);
                    if let Some(scope) = self[node].scope {
                        let rest: Vec<_> = rest_.iter().chain(path.iter()).cloned().collect();
                        self.resolve_path(scope, &rest, nodes, visited);
                        visited.remove(&current_scope);
                        continue;
                    }
                }
                _ => {
                    //
                }
            }
        }

        // 3. Find nodes in parent scope.
        if let Some(parent) = self[current_scope].parent {
            self.resolve_path(parent, path, nodes, visited);
        }
    }
}

impl Import {
    pub fn last_name(&self) -> Option<&Symbol> {
        match self.kind {
            ImportKind::Named(Some(ref alias)) => Some(alias),
            ImportKind::Named(None) => self.path.last(),
            ImportKind::Glob => None,
        }
    }
}

impl<Definition> std::ops::Index<ScopeIndex> for Environment<Definition> {
    type Output = Scope;

    #[inline]
    fn index(&self, index: ScopeIndex) -> &Self::Output {
        &self.scopes[index.0]
    }
}

impl<Definition> std::ops::IndexMut<ScopeIndex> for Environment<Definition> {
    #[inline]
    fn index_mut(&mut self, index: ScopeIndex) -> &mut Self::Output {
        &mut self.scopes[index.0]
    }
}

impl<Definition> std::ops::Index<NodeIndex> for Environment<Definition> {
    type Output = Node<Definition>;

    #[inline]
    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.nodes[index.0]
    }
}

#[macro_export]
macro_rules! scope {
    ($env:ident) => {{ $env.add_scope(None) }};
    ($env:ident, $parent:ident) => {{ $env.add_scope(Some($parent)) }};
}

#[macro_export]
macro_rules! def {
    ($env:ident, $scope:ident, $symbol:literal, $node:expr) => {{
        $env.add_definition(
            $scope,
            symbol!($symbol),
            Node {
                definition: $node,
                scope: None,
            },
        )
        .unwrap()
    }};
    ($env:ident, $scope:ident, $symbol:literal, $node:expr, $node_scope:expr) => {{
        $env.add_definition(
            $scope,
            symbol!($symbol),
            Node {
                definition: $node,
                scope: Some($node_scope),
            },
        )
        .unwrap()
    }};
}

#[macro_export]
macro_rules! import {
    ($env:ident, $scope:ident, [$($path:literal),+ $(,)?]) => {{
        $env.add_import_direct($scope, vec![ $(symbol!($path)),+ ])
    }};
    ($env:ident, $scope:ident, [$($path:literal),+ $(,)?] as $alias:literal) => {{
        $env.add_import_alias($scope, vec![ $(symbol!($path)),+ ], symbol!($alias))
    }};
    ($env:ident, $scope:ident, [$($path:literal),+ $(,)?] *) => {{
        $env.add_import_glob($scope, vec![ $(symbol!($path)),+ ])
    }};
}

#[cfg(test)]
mod tests {
    use super::Node;

    type Environment = super::Environment<Definition>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Definition {
        Struct(usize),
        Module(usize),
    }

    macro_rules! assert_path {
        ($env:ident .resolve($scope:expr, [$($path:literal),+ $(,)?]) => [$($definition:ident($index:expr)),* $(,)?]) => {{
            let nodes = $env.resolve($scope, &[ $(symbol!($path)),+ ]);
            assert_eq!(nodes, vec![ $(Definition::$definition($index)),* ]);
        }};
    }

    #[test]
    fn environment() {
        // use foo::Bar;
        // use foo::bar::Baz as A;
        // use foo::baz::*;
        //
        // mod foo {
        //     struct Bar;
        //
        //     mod bar {
        //         struct Baz;
        //
        //         mod qux {
        //             struct Quux;
        //         }
        //     }
        //
        //     mod baz {
        //         struct Qux;
        //     }
        // }
        let mut env = Environment::new();

        let root = scope!(env);
        import!(env, root, ["foo", "Bar"]);
        import!(env, root, ["foo", "bar", "Baz"] as "A");
        import!(env, root, ["foo", "baz"] *);

        let foo = scope!(env, root);
        def!(env, root, "foo", Definition::Module(0), foo);
        def!(env, foo, "Bar", Definition::Struct(1));

        let bar = scope!(env, foo);
        def!(env, foo, "bar", Definition::Module(2), bar);
        def!(env, bar, "Baz", Definition::Struct(3));

        let qux = scope!(env, bar);
        def!(env, bar, "qux", Definition::Module(4), qux);
        def!(env, qux, "Quux", Definition::Struct(5));

        let baz = scope!(env, foo);
        def!(env, foo, "baz", Definition::Module(6), baz);
        def!(env, baz, "Qux", Definition::Struct(7));

        assert_path!(env.resolve(root, ["foo"]) => [Module(0)]);
        assert_path!(env.resolve(root, ["foo", "Bar"]) => [Struct(1)]);
        assert_path!(env.resolve(root, ["foo", "bar"]) => [Module(2)]);
        assert_path!(env.resolve(root, ["foo", "bar", "Baz"]) => [Struct(3)]);
        assert_path!(env.resolve(root, ["Bar"]) => [Struct(1)]);
        assert_path!(env.resolve(root, ["A"]) => [Struct(3)]);
        assert_path!(env.resolve(root, ["Qux"]) => [Struct(7)]);
        assert_path!(env.resolve(root, ["bar", "qux", "Quux"]) => [Struct(5)]);
    }
}

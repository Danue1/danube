#[macro_export]
macro_rules! ast_node {
    (
        $(#[$meta:meta])*
        struct $node:ident;

        $($rest:tt)*
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $node(crate::SyntaxNode);

        impl rowan::ast::AstNode for crate::$node {
            type Language = crate::Danube;

            #[inline]
            fn can_cast(kind: <Self::Language as rowan::Language>::Kind) -> bool
            where
                Self: Sized,
            {
                matches!(kind, crate::SyntaxKind::$node)
            }

            fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self>
            where
                Self: Sized,
            {
                Self::can_cast(node.kind()).then(|| Self(node))
            }

            #[inline]
            fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
                &self.0
            }
        }

        ast_node!(impl $node { $($rest)* });
    };
    (
        $(#[$meta:meta])*
        enum $node:ident { $($variant:ident($definition:ident),)+ }

        $($rest:tt)*
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $node {
            $($variant(crate::$definition),)+
        }

        impl rowan::ast::AstNode for crate::$node {
            type Language = crate::Danube;

            fn can_cast(kind: <Self::Language as rowan::Language>::Kind) -> bool
            where
                Self: Sized,
            {
                $(crate::$definition::can_cast(kind))||+
            }

            fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self>
            where
                Self: Sized,
            {
                $(if let Some(node) = crate::$definition::cast(node.clone()) {
                    return Some(Self::$variant(node));
                })+
                None
            }

            fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
                match self {
                    $(Self::$variant(node) => node.syntax(),)+
                }
            }
        }

        ast_node!(impl $node { $($rest)* });
    };
    (impl $node:ident {
        $($rest:tt)*
    }) => {
        impl std::fmt::Display for crate::$node {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use rowan::ast::AstNode;

                self.syntax().fmt(f)
            }
        }

        #[allow(non_snake_case)]
        impl crate::$node {
            ast_node!($($rest)*);
        }
    };
    (node $method:ident -> $ty:ident; $($rest:tt)*) => {
        pub fn $method(&self) -> Option<crate::$ty> {
            use rowan::ast::AstNode;

            self.syntax()
                .children()
                .find_map(crate::$ty::cast)
        }

        ast_node!($($rest)*);
    };
    (node [$index:expr] $method:ident -> $ty:ident; $($rest:tt)*) => {
        pub fn $method(&self) -> Option<crate::$ty> {
            use rowan::ast::AstNode;

            self.syntax()
                .children()
                .nth($index)
                .and_then(crate::$ty::cast)
        }

        ast_node!($($rest)*);
    };
    (node $method:ident -> $ty:ident before $before:ident; $($rest:tt)*) => {
        pub fn $method(&self) -> Option<crate::$ty> {
            use rowan::ast::AstNode;

            self.syntax()
                .children()
                .take_while(|node| !crate::$before::can_cast(node.kind()))
                .find_map(crate::$ty::cast)
        }

        ast_node!($($rest)*);
    };
    (node $method:ident -> $ty:ident after $after:ident; $($rest:tt)*) => {
        pub fn $method(&self) -> Option<crate::$ty> {
            use rowan::ast::AstNode;

            self.syntax()
                .children()
                .skip_while(|node| !crate::$after::can_cast(node.kind()))
                .skip(1)
                .find_map(crate::$ty::cast)
        }

        ast_node!($($rest)*);
    };
    (nodes $method:ident -> $ty:ident; $($rest:tt)*) => {
        pub fn $method(&self) -> impl Iterator<Item = crate::$ty> + '_ {
            use rowan::ast::AstNode;

            self.syntax()
                .children()
                .filter_map(crate::$ty::cast)
        }

        ast_node!($($rest)*);
    };
    (token $method:ident -> $ty:ident; $($rest:tt)*) => {
        pub fn $method(&self) -> Option<crate::SyntaxToken> {
            use rowan::ast::AstNode;

            self.syntax()
                .children_with_tokens()
                .find_map(|node| {
                    match node {
                        rowan::NodeOrToken::Token(token) if token.kind() == crate::SyntaxKind::$ty => {
                            Some(token)
                        }
                        _ => None,
                    }
                })
        }

        ast_node!($($rest)*);
    };
    (tokens $method:ident -> $ty:ident; $($rest:tt)*) => {
        pub fn $method(&self) -> impl Iterator<Item = crate::SyntaxToken> + '_ {
            use rowan::ast::AstNode;

            self.syntax()
                .children_with_tokens()
                .filter_map(|node| {
                    match node {
                        rowan::NodeOrToken::Token(token) if token.kind() == crate::SyntaxKind::$ty => {
                            Some(token)
                        }
                        _ => None,
                    }
                })
        }

        ast_node!($($rest)*);
    };
    () => {
        //
    };
}

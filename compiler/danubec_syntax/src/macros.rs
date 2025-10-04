#[macro_export]
macro_rules! ast_node {
    (
        $(#[$meta:meta])*
        enum $node:ident;

        $(
            variant $variant:ident -> $ty:ident;
        )*
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone)]
        pub enum $node {
            $(
                $variant($ty),
            )*
        }

        impl rowan::ast::AstNode for $node {
            type Language = crate::Danube;

            #[inline]
            fn can_cast(kind: <Self::Language as rowan::Language>::Kind) -> bool {
                $(crate::$ty::can_cast(kind))||+
            }

            #[inline]
            fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self> {
                $(
                    if <$ty>::can_cast(node.kind()) {
                        return Some($node::$variant(<$ty>::cast(node).unwrap()));
                    }
                )*

                None
            }

            #[inline]
            fn syntax(&self) -> &crate::SyntaxNode {
                match self {
                    $(
                        $node::$variant(it) => it.syntax(),
                    )*
                }
            }
        }
    };
    (
        $(#[$meta:meta])*
        struct $node:ident where $kind:ident;

        $($rest:tt)*
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $node(crate::SyntaxNode);

        impl rowan::ast::AstNode for $node {
            type Language = crate::Danube;

            #[inline]
            fn can_cast(kind: <Self::Language as rowan::Language>::Kind) -> bool {
                matches!(kind, crate::SyntaxKind::$kind)
            }

            #[inline]
            fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self> {
                Self::can_cast(node.kind()).then(|| $node(node))
            }

            #[inline]
            fn syntax(&self) -> &crate::SyntaxNode {
                &self.0
            }
        }

        ast_node!(impl $node { $($rest)* });
    };
    (impl $node:ident { $($body:tt)* }) => {
        #[allow(non_snake_case)]
        impl crate::$node {
            ast_node!($($body)*);
        }
    };
    (token $node:ident where $kind:ident; $($rest:tt)*) => {
        pub fn $node(&self) -> Option<crate::SyntaxToken> {
            use rowan::{NodeOrToken, ast::AstNode};

            self.syntax().children_with_tokens().find_map(|node| {
                match node {
                    NodeOrToken::Token(token) if token.kind() == crate::SyntaxKind::$kind => Some(token),
                    _ => None
                }
            })
        }

        ast_node!($($rest)*);
    };
    (tokens $node:ident where $kind:ident; $($rest:tt)*) => {
        pub fn $node(&self) -> impl Iterator<Item = crate::SyntaxToken> {
            use rowan::{NodeOrToken, ast::AstNode};

            self.syntax().children_with_tokens().filter_map(|node| {
                match node {
                    NodeOrToken::Token(token) if token.kind() == crate::SyntaxKind::$kind => Some(token),
                    _ => None
                }
            })
        }

        ast_node!($($rest)*);
    };
    (node $node:ident -> $ty:ty; $($rest:tt)*) => {
        pub fn $node(&self) -> Option<$ty> {
            use rowan::ast::AstNode;

            self.syntax().children().find_map(<$ty>::cast)
        }

        ast_node!($($rest)*);
    };
    (nodes $node:ident -> $ty:ty; $($rest:tt)*) => {
        pub fn $node(&self) -> impl Iterator<Item = $ty> {
            use rowan::ast::AstNode;

            self.syntax().children().filter_map(<$ty>::cast)
        }

        ast_node!($($rest)*);
    };
    () => {
        //
    };
}

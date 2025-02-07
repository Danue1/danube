pub mod definition;
pub mod expression;
pub mod identifier;
pub mod literal;
pub mod name;
pub mod raw;
pub mod root;
pub mod statement;
pub mod r#type;

pub use definition::*;
pub use expression::*;
pub use identifier::*;
pub use literal::*;
pub use name::*;
pub use r#type::*;
pub use raw::*;
pub use root::*;
pub use statement::*;

use danubec_syntax::*;

#[macro_export]
macro_rules! ast_node {
    (struct $node:ident; $($rest:tt)*) => {
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

        crate::ast_node!(impl $node { $($rest)* });
    };
    (enum $node:ident { $($variant:ident,)+ } $($rest:tt)*) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $node {
            $($variant(crate::$variant),)+
        }

        impl rowan::ast::AstNode for crate::$node {
            type Language = crate::Danube;

            fn can_cast(kind: <Self::Language as rowan::Language>::Kind) -> bool
            where
                Self: Sized,
            {
                false || $(crate::$variant::can_cast(kind))||+
            }

            fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self>
            where
                Self: Sized,
            {
                $(if let Some(node) = crate::$variant::cast(node.clone()) {
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

        crate::ast_node!(impl $node { $($rest)* });
    };
    (enum $node:ident { $($variant:ident($definition:ident),)+ } $($rest:tt)*) => {
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
                false || $(crate::$definition::can_cast(kind))||+
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

        crate::ast_node!(impl $node { $($rest)* });
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
            crate::ast_node!($($rest)*);
        }
    };
    (node $method:ident -> $ty:ident; $($rest:tt)*) => {
        pub fn $method(&self) -> Option<crate::$ty> {
            use rowan::ast::AstNode;

            self.syntax()
                .children()
                .find_map(crate::$ty::cast)
        }

        crate::ast_node!($($rest)*);
    };
    (nodes $method:ident -> $ty:ident; $($rest:tt)*) => {
        pub fn $method(&self) -> impl Iterator<Item = crate::$ty> + '_ {
            use rowan::ast::AstNode;

            self.syntax()
                .children()
                .filter_map(crate::$ty::cast)
        }

        crate::ast_node!($($rest)*);
    };
    (token $method:ident -> $ty:ident; $($rest:tt)*) => {
        pub fn $method(&self) -> Option<crate::SyntaxNode> {
            use rowan::ast::AstNode;

            self.syntax()
                .children()
                .find(|node| matches!(node.kind(), crate::SyntaxKind::$ty))
        }

        crate::ast_node!($($rest)*);
    };
    (tokens $method:ident -> $ty:ident; $($rest:tt)*) => {
        pub fn $method(&self) -> impl Iterator<Item = crate::SyntaxNode> + '_ {
            use rowan::ast::AstNode;

            self.syntax()
                .children()
                .filter(|node| matches!(node.kind(), crate::SyntaxKind::$ty))
        }

        crate::ast_node!($($rest)*);
    };
    () => {
        //
    };
}

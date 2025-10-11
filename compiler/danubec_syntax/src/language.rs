pub type SyntaxNode = rowan::SyntaxNode<Danube>;

pub type SyntaxToken = rowan::SyntaxToken<Danube>;

pub type SyntaxElement = rowan::SyntaxElement<Danube>;

pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<Danube>;

pub type Span = rowan::ast::SyntaxNodePtr<Danube>;

pub type GreenNodeBuilder = rowan::GreenNodeBuilder<'static>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Danube;

impl rowan::Language for Danube {
    type Kind = crate::SyntaxKind;

    #[inline]
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        unsafe { std::mem::transmute(raw.0) }
    }

    #[inline]
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

impl From<crate::SyntaxKind> for rowan::SyntaxKind {
    #[inline]
    fn from(kind: crate::SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

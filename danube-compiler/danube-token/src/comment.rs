use crate::Span;

#[derive(Debug, PartialEq, Clone)]
pub struct Comment {
    pub is_document: bool,
    pub kind: CommentKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CommentKind {
    Singleline,
    Multiline(Vec<Span>),
}

impl Comment {
    pub const fn new_singleline(is_document: bool) -> Self {
        Comment {
            is_document,
            kind: CommentKind::Singleline,
        }
    }

    pub const fn new_multiline(is_document: bool, spans: Vec<Span>) -> Self {
        Comment {
            is_document,
            kind: CommentKind::Multiline(spans),
        }
    }
}

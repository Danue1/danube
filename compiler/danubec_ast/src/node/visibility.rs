use super::Path;

pub struct Visibility {
    pub kind: VisibilityKind,
}

pub enum VisibilityKind {
    Private,
    Public,
    Crate,
    Super,
    In(Path),
}

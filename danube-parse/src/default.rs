use crate::*;

impl Default for VisibilityKind {
    fn default() -> Self {
        VisibilityKind::TypeSelf
    }
}

impl Default for ImmutablityKind {
    fn default() -> Self {
        ImmutablityKind::Yes
    }
}

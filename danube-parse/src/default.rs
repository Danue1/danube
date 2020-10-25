use crate::*;

impl Default for VisibilityKind {
    fn default() -> Self {
        VisibilityKind::Current
    }
}

impl Default for ImmutablityKind {
    fn default() -> Self {
        ImmutablityKind::Yes
    }
}

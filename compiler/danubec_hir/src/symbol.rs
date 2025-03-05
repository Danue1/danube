new_monotonic! {
    struct KrateId;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirId(DefId);

new_monotonic! {
    struct DefId;
}

new_arena! {
    struct BodyId;
}

impl DefId {
    #[inline]
    pub const fn as_hir_id(self) -> HirId {
        HirId(self)
    }
}

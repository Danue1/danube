pub mod arena;
pub mod monotonic;

pub use arena::*;

new_arena!(DefId);
new_monotonic!(LocalDefId);

new_arena!(ModDefId);
new_monotonic!(LocalModDefId);

new_arena!(CrateId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash64(u64);

impl Hash64 {
    pub fn new(raw: &str) -> Self {
        use std::hash::{DefaultHasher, Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        raw.hash(&mut hasher);

        Hash64(hasher.finish())
    }

    #[inline]
    pub const fn new_unchecked(value: u64) -> Self {
        Hash64(value)
    }
}

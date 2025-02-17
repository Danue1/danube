#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash(u64);

impl Hash {
    pub fn new(raw: &str) -> Self {
        use std::hash::{DefaultHasher, Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        raw.hash(&mut hasher);

        Hash(hasher.finish())
    }
}

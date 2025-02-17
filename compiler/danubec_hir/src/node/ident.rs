use danubec_data_structure::Hash;
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

static MAP: OnceLock<IdentMap> = OnceLock::new();

struct IdentMap {
    map: Mutex<HashMap<Hash, String>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident(Hash);

impl Ident {
    pub fn new(raw: &str) -> Self {
        let hash = Hash::new(raw);
        let map = MAP.get_or_init(IdentMap::new);
        map.insert(hash, raw.to_owned());

        Ident(hash)
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let map = MAP.get_or_init(IdentMap::new);
        let raw = map.get(self.0).expect("failed to get ident");

        write!(f, "{}", raw)
    }
}

impl std::fmt::Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let map = MAP.get_or_init(IdentMap::new);
        let raw = map.get(self.0).expect("failed to get ident");

        f.debug_struct("Ident")
            .field("hash", &self.0)
            .field("raw", &raw)
            .finish()
    }
}

impl IdentMap {
    pub fn new() -> Self {
        IdentMap {
            map: Mutex::new(HashMap::new()),
        }
    }

    pub fn insert(&self, hash: Hash, raw: String) {
        let mut map = self.map.lock().expect("failed to lock ident map");
        map.insert(hash, raw);
    }

    pub fn get(&self, hash: Hash) -> Option<String> {
        let map = self.map.lock().expect("failed to lock ident map");

        map.get(&hash).cloned()
    }
}

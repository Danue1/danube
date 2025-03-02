use danubec_data_structure::Hash64;
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

static SYMBOLS: OnceLock<Symbols> = OnceLock::new();

struct Symbols {
    map: Mutex<HashMap<Hash64, String>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(Hash64);

impl Symbol {
    pub const CRATE: Self = Symbol(Hash64::new_unchecked(3160908839602319882));
    pub const SUPER: Self = Symbol(Hash64::new_unchecked(10332305186512876660));

    pub fn new(raw: &str) -> Self {
        let hash = Hash64::new(raw);
        let map = SYMBOLS.get_or_init(Symbols::new);
        map.insert(hash, raw.to_owned());

        Symbol(hash)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let map = SYMBOLS.get_or_init(Symbols::new);
        let raw = map.get(self.0).expect("failed to get ident");

        write!(f, "{}", raw)
    }
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let map = SYMBOLS.get_or_init(Symbols::new);
        let raw = map.get(self.0).expect("failed to get ident");

        f.debug_struct("Ident")
            .field("hash", &self.0)
            .field("raw", &raw)
            .finish()
    }
}

impl Symbols {
    pub fn new() -> Self {
        Symbols {
            map: Mutex::new(HashMap::new()),
        }
    }

    pub fn insert(&self, hash: Hash64, raw: String) {
        let mut map = self.map.lock().expect("failed to lock ident map");
        map.insert(hash, raw);
    }

    pub fn get(&self, hash: Hash64) -> Option<String> {
        let map = self.map.lock().expect("failed to lock ident map");

        map.get(&hash).cloned()
    }
}

#[macro_export]
macro_rules! symbol {
    ($raw:literal) => {
        $crate::Symbol::new($raw)
    };
}

#[test]
fn constant() {
    assert_eq!(Symbol::new("crate"), Symbol::CRATE);
    assert_eq!(Symbol::new("super"), Symbol::SUPER);
}

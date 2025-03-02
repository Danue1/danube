#![warn(clippy::all)]

pub use danubec_parse::EntryKind;

use danubec_parse::parse_crate;
use std::{collections::HashMap, path::PathBuf};

pub struct CompileConfig {
    pub working_directory: PathBuf,
    pub crates: HashMap<String, EntryKind>,
}

impl CompileConfig {
    pub fn compile(self) {
        let mut crates = HashMap::new();
        for (krate, kind) in self.crates {
            let entry = self.working_directory.join(&krate);
            crates.insert(krate.to_owned(), parse_crate(entry, kind));
        }

        dbg!(crates);
    }
}

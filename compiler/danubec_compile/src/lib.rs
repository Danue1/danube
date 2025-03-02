#![warn(clippy::all)]

use danubec_diagnostic::Diagnostic;
use danubec_lst_lowering::lower_krate;
use danubec_middle::EntryKind;
use danubec_parse::parse_crate;
use std::{collections::HashMap, path::PathBuf};

pub struct CompileConfig {
    pub working_directory: PathBuf,
    pub crates: HashMap<String, EntryKind>,
}

impl CompileConfig {
    pub fn compile(self) -> Result<(), Diagnostic> {
        let mut crates = HashMap::new();
        for (name, kind) in self.crates {
            let entry = self.working_directory.join(&name);
            let krate = parse_crate(entry, kind);
            let krate = lower_krate(krate)?;
            crates.insert(name.to_owned(), krate);
        }

        Ok(())
    }
}

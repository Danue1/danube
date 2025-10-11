use crate::{build, file_system::FileSystem, symbol::SymbolInterner, table::Table};
use danubec_diagnostic::Diagnostic;
use std::path::PathBuf;

pub struct Context {
    pub path: PathBuf,
}

pub fn lower(Context { path }: Context) -> (FileSystem, Table, SymbolInterner, Diagnostic) {
    let mut fs = FileSystem::new();
    let mut table = Table::new();
    let mut symbols = SymbolInterner::new();
    let mut diagnostic = Diagnostic::new();

    let root = fs.krate(path);

    build(&mut fs, &mut table, &mut symbols, &mut diagnostic, root);

    (fs, table, symbols, diagnostic)
}

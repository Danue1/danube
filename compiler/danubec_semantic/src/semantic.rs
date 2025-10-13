use crate::{
    check::check, collect::collect, env::Env, fs::Fs, inference::inference, resolve::resolve,
    ticker::Ticker,
};
use danubec_diagnostic::Diagnostic;
use danubec_symbol::SymbolInterner;
use std::path::PathBuf;

pub struct Context {
    pub root: PathBuf,
}

pub fn semantic(Context { root }: Context) -> (Fs, Env, SymbolInterner, Diagnostic) {
    let mut fs = Fs::new();
    let mut env = Env::new();
    let mut symbols = SymbolInterner::new();
    let mut diagnostic = Diagnostic::new();

    let root = fs.krate(root);

    collect(&mut fs, &mut env, &mut symbols, &mut diagnostic, root);

    loop {
        let mut ticker = Ticker::new();
        resolve(&mut env, &mut symbols, &mut diagnostic, &mut ticker);
        inference(&mut env, &mut symbols, &mut diagnostic, &mut ticker);
        check(&mut env, &mut symbols, &mut diagnostic, &mut ticker);
        if !ticker.changed() {
            break;
        }
    }

    (fs, env, symbols, diagnostic)
}

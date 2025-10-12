use crate::{
    check::check, collect::collect, env::Env, fs::Fs, inference::inference, resolve::resolve,
};
use danubec_diagnostic::Diagnostic;
use danubec_symbol::SymbolInterner;
use std::path::PathBuf;

pub struct Context {
    pub path: PathBuf,
}

pub fn semantic(Context { path }: Context) -> (Fs, Env, SymbolInterner, Diagnostic) {
    let mut fs = Fs::new();
    let mut env = Env::new();
    let mut symbols = SymbolInterner::new();
    let mut diagnostic = Diagnostic::new();

    let root = fs.krate(path);

    collect(&mut fs, &mut env, &mut symbols, &mut diagnostic, root);

    fixed_point(|changed| {
        resolve();
        inference();
        check();

        *changed = false;
    });

    (fs, env, symbols, diagnostic)
}

fn fixed_point<F>(mut f: F)
where
    F: FnMut(&mut bool),
{
    loop {
        let mut changed = true;
        f(&mut changed);
        if !changed {
            break;
        }
    }
}

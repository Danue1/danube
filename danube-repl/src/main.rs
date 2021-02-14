mod repl;

use repl::*;

fn main() {
    let mut repl = REPL::new();
    repl.run();
}

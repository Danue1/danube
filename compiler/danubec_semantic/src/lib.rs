#[macro_use]
extern crate miette;

#[cfg(test)]
mod tests;

mod check;
mod collect;
mod env;
mod fs;
mod inference;
mod resolve;
mod semantic;

pub use semantic::*;

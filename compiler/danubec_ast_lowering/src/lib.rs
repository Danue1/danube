#![allow(unused)]

#[macro_use]
extern crate miette;

#[cfg(test)]
mod tests;

mod build;
mod collect;
mod file_system;
mod lower;
mod symbol;
mod table;

pub use build::*;
pub use lower::*;

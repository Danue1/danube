#![allow(clippy::all)]

#[macro_use]
extern crate miette;

#[cfg(test)]
mod tests;

pub mod parse;

mod event;
mod grammar;
mod lower;
mod token_stream;

pub use parse::*;

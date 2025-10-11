#![allow(clippy::all)]

#[macro_use]
extern crate miette;

#[macro_use]
mod tokens;

#[cfg(test)]
mod tests;

pub mod parse;

mod event;
mod grammar;
mod token_stream;

pub use parse::*;

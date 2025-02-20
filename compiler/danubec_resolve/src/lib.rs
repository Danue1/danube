#![warn(clippy::all)]
#![allow(unused)]

#[macro_use]
extern crate danubec_symbol;

#[macro_use]
pub mod scope;

pub mod definition;

pub use definition::*;

pub use scope::*;

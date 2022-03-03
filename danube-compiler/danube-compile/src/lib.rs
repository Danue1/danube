#![warn(clippy::all)]

#[cfg(test)]
mod tests;

pub mod compile;
pub mod config;
mod context;

pub use compile::*;
pub use config::*;
use context::*;

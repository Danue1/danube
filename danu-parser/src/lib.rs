#![warn(clippy::all)]

mod ast;
mod error;
mod parse;
mod position;
mod utils;

pub use ast::*;
pub use error::*;
pub use parse::*;
pub use position::*;
pub(crate) use utils::*;

type Result<'a, T> = nom::IResult<Span<'a>, T, Error<'a>>;
type Span<'a> = nom_locate::LocatedSpan<&'a str>;

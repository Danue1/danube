mod function;
mod item;
mod krate;
mod module;
mod ty;

use crate::*;
use danube_hir::Crate;

type HirResult<T> = Result<T, Error>;

pub fn from_str(s: &str) -> HirResult<Crate> {
    HirContext::default().lower_crate(&danube_parse::from_str(s)?)
}

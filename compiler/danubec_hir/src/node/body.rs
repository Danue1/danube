use super::Statement;
use crate::HirId;

#[derive(Debug)]
pub struct Body {
    pub inputs: Vec<HirId>,
    pub output: Option<HirId>,
    pub statements: Vec<Statement>,
}

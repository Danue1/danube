use super::Statement;
use crate::DefId;

pub struct Body {
    pub inputs: Vec<DefId>,
    pub output: Option<DefId>,
    pub statements: Vec<Statement>,
}

use crate::{Error, Parse};
use danube_ast::PatternNode;

impl<'parse> Parse<'parse> {
    pub fn parse_pattern_node(&mut self) -> Result<PatternNode, Error> {
        std::todo!();
    }
}

use crate::*;
use nom::combinator::map;

pub(super) fn ident_node(s: Span) -> Result<IdentNode> {
  map(name, |raw| IdentNode { raw })(s)
}

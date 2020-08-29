use super::*;

pub(super) fn parse_program_node(s: Tokens) -> ParseResult<ProgramNode> {
  map(all_consuming(parse_module_node), |module| ProgramNode {
    module,
  })(s)
}

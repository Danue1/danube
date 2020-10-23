use super::*;

pub(super) fn parse_generic_node(t: Tokens) -> ParseResult<GenericNode> {
    map(
        tuple((
            parse_ident_node,
            opt(preceded(
                parse_symbol(Symbol::Colon),
                tuple((
                    separated_nonempty_list(parse_symbol(Symbol::Add), parse_path_node),
                    opt(preceded(
                        parse_symbol(Symbol::Assign),
                        separated_nonempty_list(parse_symbol(Symbol::Add), parse_path_node),
                    )),
                )),
            )),
        )),
        |(ident, trait_list)| {
            let (trait_list, default_trait_list) = trait_list.unwrap_or_else(Default::default);
            let default_trait_list = default_trait_list.unwrap_or_else(Default::default);

            GenericNode {
                ident,
                trait_list,
                default_trait_list,
            }
        },
    )(t)
}

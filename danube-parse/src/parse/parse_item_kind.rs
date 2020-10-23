use super::*;

pub(super) fn parse_item_kind(t: Tokens) -> ParseResult<ItemKind> {
    alt((
        map(parse_use_node, ItemKind::Use),
        map(parse_module, ItemKind::Module),
        map(parse_struct, ItemKind::Struct),
        map(parse_enum, ItemKind::Enum),
        map(parse_function, ItemKind::Function),
        map(parse_type_alias_node, ItemKind::TypeAlias),
        map(parse_trait_node, ItemKind::Trait),
        map(parse_constant_node, ItemKind::Constant),
        map(parse_static_node, ItemKind::Static),
        map(parse_implement_node, ItemKind::Implement),
        map(parse_implement_trait_node, ItemKind::ImplementTrait),
    ))(t)
}

use super::*;

pub fn parse_attributed<'a, T, F>(f: F) -> impl Fn(Tokens<'a>) -> ParseResult<'a, Attributed<T>>
where
    T: Sized,
    F: Copy + Fn(Tokens<'a>) -> ParseResult<'a, T>,
{
    move |t: Tokens<'a>| -> ParseResult<'a, Attributed<T>> {
        map(
            tuple((many0(parse_attribute_node), f)),
            |(attribute_list, node)| Attributed {
                attribute_list,
                node,
            },
        )(t)
    }
}

use super::*;

pub(super) fn parse_attributed<'a, T, F>(
    f: F,
) -> impl Fn(Tokens<'a>) -> ParseResult<'a, Attributed<T>>
where
    T: Sized,
    F: Fn(Tokens<'a>) -> ParseResult<'a, T>,
{
    move |t: Tokens<'a>| -> ParseResult<'a, Attributed<T>> {
        let (t, attribute_list) = many0(parse_attribute_node)(t)?;
        let (t, node) = f(t)?;
        let node = Attributed {
            attribute_list,
            node,
        };

        Ok((t, node))
    }
}

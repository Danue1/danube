use danube_span::Span;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownSymbol,
    Invalid(usize),
    Need(usize, char),
    MalformedInteger(Span),
    MalformedFloating(Span),
}

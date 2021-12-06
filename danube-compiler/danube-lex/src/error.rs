use danube_token::Span;

#[derive(Debug, PartialEq)]
pub enum Error {
    Invalid(usize),
    Need(usize, char),
    MalformedInteger(Span),
    MalformedFloating(Span),
}

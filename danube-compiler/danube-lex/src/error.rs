use danube_span::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownSymbol,
    Invalid(Location),
    Need(usize, char),
    MalformedInteger(Location),
    MalformedFloating(Location),
}

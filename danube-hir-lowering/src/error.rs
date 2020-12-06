#[derive(Debug, PartialEq)]
pub enum Error {
    Lex(danube_lex::Error),
    Parser(danube_parse::Error),
    Diagnogistic(String),
}

impl From<danube_parse::Error> for Error {
    fn from(error: danube_parse::Error) -> Self {
        Error::Parser(error)
    }
}

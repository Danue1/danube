use danube_token::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    Invalid,
    Lex(danube_lex::Error),
    Illegal(Token),
}

impl From<danube_lex::Error> for Error {
    fn from(error: danube_lex::Error) -> Self {
        Error::Lex(error)
    }
}

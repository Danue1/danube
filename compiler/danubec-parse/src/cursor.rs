use danubec_token::Token;

pub struct Cursor<'parse> {
    tokens: std::iter::Peekable<std::slice::Iter<'parse, Token<'parse>>>,
}

impl<'parse> Cursor<'parse> {
    pub fn new(tokens: &'parse [Token]) -> Cursor<'parse> {
        Cursor {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn advanced(&mut self) {
        self.tokens.next();
    }
}

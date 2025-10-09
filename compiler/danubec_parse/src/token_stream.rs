use danubec_syntax::SyntaxKind;

pub struct TokenStream<'source> {
    tokens: &'source [SyntaxKind],
}

impl<'source> TokenStream<'source> {
    pub fn new(tokens: &'source [SyntaxKind]) -> Self {
        let mut tokens = Self { tokens };
        tokens.trivia();
        tokens
    }

    // Get the nth token, ignoring trivia
    pub fn nth(&self, n: usize) -> SyntaxKind {
        self.tokens
            .iter()
            .filter(|kind| !kind.at_trivia())
            .nth(n)
            .copied()
            .unwrap_or(SyntaxKind::END_OF_FILE)
    }

    // Get the nth token, including trivia
    pub fn nth_(&self, n: usize) -> SyntaxKind {
        self.tokens
            .get(n)
            .copied()
            .unwrap_or(SyntaxKind::END_OF_FILE)
    }

    pub fn bump(&mut self) {
        self.advance();
        self.trivia();
    }

    fn advance(&mut self) {
        self.tokens = &self.tokens[1..];
    }

    fn trivia(&mut self) {
        loop {
            match self.tokens.get(0) {
                Some(kind) if kind.at_trivia() => {
                    self.advance();
                }
                _ => break,
            }
        }
    }
}

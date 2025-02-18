use danubec_syntax::SyntaxKind;

#[derive(Clone)]
pub struct Lex<'lex> {
    source: &'lex str,
    index: usize,
}

impl<'lex> Lex<'lex> {
    pub fn new(source: &'lex str) -> Self {
        Self { source, index: 0 }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.source.len() == self.index
    }

    #[inline]
    pub fn peek(&self) -> Option<(SyntaxKind, &'lex str)> {
        self.clone().next()
    }

    pub fn matches<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(SyntaxKind, &'lex str) -> bool,
    {
        match self.peek() {
            Some((kind, source)) => f(kind, source),
            None => false,
        }
    }
}

impl<'lex> Iterator for Lex<'lex> {
    type Item = (SyntaxKind, &'lex str);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.index;
        let mut end = start + 1;
        let mut chars = self.source[start..].chars();

        macro_rules! bump_while {
            ($pat:pat) => {
                while let Some($pat) = chars.next() {
                    end += 1;
                }
            };
        }

        macro_rules! token {
            ($token:ident) => {{
                self.index += end - start;

                Some((SyntaxKind::$token, &self.source[start..end]))
            }};
        }

        match chars.next()? {
            ' ' => {
                bump_while!(' ');
                token!(WHITESPACE)
            }
            '\n' => {
                bump_while!('\n');
                token!(NEW_LINE)
            }
            '\t' => {
                bump_while!('\t');
                token!(TAB)
            }
            '~' => token!(TILDE),
            '`' => token!(BACKTICK),
            '!' => token!(EXCLAMATION),
            '@' => token!(AT),
            '#' => token!(HASH),
            '$' => token!(DOLLAR),
            '%' => token!(PERCENT),
            '^' => token!(CARET),
            '&' => token!(AMPERSAND),
            '*' => token!(ASTERISK),
            '-' => token!(HYPHEN),
            '_' => token!(UNDERSCORE),
            '+' => token!(PLUS),
            '=' => token!(EQUAL),
            '|' => token!(PIPE),
            ':' => token!(COLON),
            ';' => token!(SEMICOLON),
            ',' => token!(COMMA),
            '.' => token!(DOT),
            '/' => token!(SLASH),
            '?' => token!(QUESTION),
            '\\' => token!(BACKSLASH),
            '{' => token!(LEFT_BRACE),
            '}' => token!(RIGHT_BRACE),
            '[' => token!(LEFT_BRACKET),
            ']' => token!(RIGHT_BRACKET),
            '(' => token!(LEFT_PAREN),
            ')' => token!(RIGHT_PAREN),
            '<' => token!(LEFT_CHEVRON),
            '>' => token!(RIGHT_CHEVRON),
            '"' => token!(DOUBLE_QUOTE),
            '\'' => token!(SINGLE_QUOTE),
            '0'..='9' => {
                bump_while!('0'..='9');
                token!(NUMERIC)
            }
            'a'..='z' | 'A'..='Z' => {
                bump_while!('a'..='z' | 'A'..='Z');
                match &self.source[start..end] {
                    "fn" => token!(FN),
                    "let" => token!(LET),
                    "true" => token!(TRUE),
                    "false" => token!(FALSE),
                    "pub" => token!(PUB),
                    "crate" => token!(CRATE),
                    "super" => token!(SUPER),
                    "in" => token!(IN),
                    "type" => token!(TYPE),
                    "where" => token!(WHERE),
                    "struct" => token!(STRUCT),
                    "enum" => token!(ENUM),
                    "trait" => token!(TRAIT),
                    "impl" => token!(IMPL),
                    "const" => token!(CONST),
                    "static" => token!(STATIC),
                    "use" => token!(USE),
                    "mod" => token!(MOD),
                    "self" => token!(SELF),
                    "Self" => token!(SELF_UPPERCASE),
                    "as" => token!(AS),
                    "for" => token!(FOR),
                    "if" => token!(IF),
                    "else" => token!(ELSE),
                    "match" => token!(MATCH),
                    "loop" => token!(LOOP),
                    "while" => token!(WHILE),
                    "return" => token!(RETURN),
                    "break" => token!(BREAK),
                    "continue" => token!(CONTINUE),
                    "await" => token!(AWAIT),
                    "yield" => token!(YIELD),
                    _ => token!(ALPHABETIC),
                }
            }
            _ => token!(UNEXPECTED),
        }
    }
}

#[test]
fn keywords() {
    let source = "fn let true false pub crate super in type where struct enum trait impl const static use mod self Self as for";
    let lex = Lex::new(source);
    let tokens: Vec<_> = lex.collect();

    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn punctuations() {
    let source = "\n\t~`!@#$%^&*-_+=|:;,./?\\{}[]()<>\"'";
    let lex = Lex::new(source);
    let tokens: Vec<_> = lex.collect();

    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn numeric() {
    let source = "123.456e+789 0b1010 0o123 0xABC";
    let lex = Lex::new(source);
    let tokens: Vec<_> = lex.collect();

    insta::assert_debug_snapshot!(tokens);
}

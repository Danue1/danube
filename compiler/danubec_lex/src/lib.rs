use danubec_syntax::SyntaxKind;

#[derive(Clone)]
pub struct Lex<'lex> {
    source: &'lex str,
    index: usize,
}

impl<'lex> Lex<'lex> {
    pub const fn new(source: &'lex str) -> Self {
        Self { source, index: 0 }
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.source.len() == self.index
    }

    #[inline]
    pub fn peek(&self) -> Option<(SyntaxKind, &'lex str)> {
        self.clone().next()
    }

    #[inline]
    pub fn matches<F>(&self, f: F) -> bool
    where
        F: FnOnce((SyntaxKind, &'lex str)) -> bool,
    {
        self.peek().is_some_and(f)
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
            '\'' => token!(SINGLE_QUOTE),
            '\\' => token!(BACKSLASH),
            '_' => token!(UNDERSCORE),
            '-' => token!(HYPHEN),
            ',' => token!(COMMA),
            ';' => token!(SEMICOLON),
            ':' => token!(COLON),
            '!' => token!(EXCLAMATION),
            '?' => token!(QUESTION),
            '.' => token!(DOT),
            '"' => token!(DOUBLE_QUOTE),
            '(' => token!(LEFT_PAREN),
            ')' => token!(RIGHT_PAREN),
            '[' => token!(LEFT_BRACKET),
            ']' => token!(RIGHT_BRACKET),
            '{' => token!(LEFT_BRACE),
            '}' => token!(RIGHT_BRACE),
            '@' => token!(AT),
            '*' => token!(ASTERISK),
            '/' => token!(SLASH),
            '&' => token!(AMPERSAND),
            '#' => token!(HASH),
            '%' => token!(PERCENT),
            '`' => token!(BACKTICK),
            '^' => token!(CARET),
            '+' => token!(PLUS),
            '<' => token!(LEFT_CHEVRON),
            '=' => token!(EQUAL),
            '>' => token!(RIGHT_CHEVRON),
            '|' => token!(PIPE),
            '~' => token!(TILDE),
            '$' => token!(DOLLAR),
            '0'..='9' => {
                bump_while!('0'..='9');
                token!(NUMERIC)
            }
            'a'..='z' | 'A'..='Z' => {
                bump_while!('a'..='z' | 'A'..='Z');
                match &self.source[start..end] {
                    "as" => token!(AS),
                    "await" => token!(AWAIT),
                    "break" => token!(BREAK),
                    "const" => token!(CONST),
                    "continue" => token!(CONTINUE),
                    "crate" => token!(CRATE),
                    "else" => token!(ELSE),
                    "enum" => token!(ENUM),
                    "false" => token!(FALSE),
                    "fn" => token!(FN),
                    "for" => token!(FOR),
                    "if" => token!(IF),
                    "impl" => token!(IMPL),
                    "in" => token!(IN),
                    "let" => token!(LET),
                    "loop" => token!(LOOP),
                    "match" => token!(MATCH),
                    "mut" => token!(MUT),
                    "mod" => token!(MOD),
                    "pub" => token!(PUB),
                    "return" => token!(RETURN),
                    "Self" => token!(SELF_UPPERCASE),
                    "self" => token!(SELF),
                    "static" => token!(STATIC),
                    "struct" => token!(STRUCT),
                    "super" => token!(SUPER),
                    "trait" => token!(TRAIT),
                    "true" => token!(TRUE),
                    "type" => token!(TYPE),
                    "use" => token!(USE),
                    "where" => token!(WHERE),
                    "while" => token!(WHILE),
                    "yield" => token!(YIELD),
                    _ => token!(ALPHABETIC),
                }
            }
            _ => {
                let count: usize = self.source[start..]
                    .chars()
                    .take_while(|c| {
                        !matches!(c,
                            'a'..='z' | 'A'..='Z' | '0'..='9'
                            | ' ' | '\n' | '\t' | '\'' | '\\' | '_' | '-' | ',' | ';' | ':' | '!'
                            | '?' | '.' | '"' | '(' | ')' | '[' | ']' | '{' | '}' | '@' | '*' | '/'
                            | '&' | '#' | '%' | '`' | '^' | '+' | '<' | '=' | '>' | '|' | '~' | '$'
                        )
                    })
                    .map(|c| c.len_utf8())
                    .sum();
                end = start + count;
                token!(RAW)
            }
        }
    }
}

#[test]
fn keywords() {
    let source = "as await break const continue crate else enum false fn for if impl in let loop match mut mod pub return Self self static struct super trait true type use where while yield";
    let lex = Lex::new(source);
    let tokens: Vec<_> = lex
        .filter(|(kind, _)| !matches!(kind, SyntaxKind::WHITESPACE))
        .collect();

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

#[test]
fn raw() {
    let source = "💖✨🚀🦀 한글";
    dbg!(source.len());
    let lex = Lex::new(source);
    let tokens: Vec<_> = lex.collect();

    insta::assert_debug_snapshot!(tokens);
}

#![warn(clippy::all)]

#[cfg(test)]
mod tests;

use danubec_syntax::SyntaxKind;
use unicode_ident::{is_xid_continue, is_xid_start};

#[derive(Debug, Clone, Copy)]
enum Mode {
    Base {
        depth: usize,
    },
    InString {
        depth: usize,
        multiline: bool,
        allow_interpolation: bool,
    },
}

pub fn lex<'lex>(source: &'lex str) -> Vec<(SyntaxKind, &'lex str)> {
    let mut tokens = vec![];
    let mut modes = vec![];
    let mut chars = source.chars();
    let mut index = 0;

    macro_rules! source {
        ($count:expr, $predicate:expr) => {{
            let mut count = $count;
            let mut peekable = chars.clone();
            loop {
                match peekable.next() {
                    Some(c) if $predicate(c) => {
                        count += c.len_utf8();
                        chars.next();
                    }
                    _ => break,
                }
            }
            slice!(count)
        }};
    }

    macro_rules! peek {
        () => {{ chars.clone().next() }};
    }

    macro_rules! nth {
        ($n:expr) => {{
            let mut peekable = chars.clone();
            for _ in 0..$n {
                peekable.next();
            }
            peekable.next()
        }};
    }

    macro_rules! many {
        ($kind:ident, $count:expr, $predicate:expr) => {{
            token!(SyntaxKind::$kind, source!($count, $predicate));
        }};
    }

    macro_rules! one {
        ($kind:ident) => {{
            token!(SyntaxKind::$kind, slice!(1));
        }};
        ($kind:ident, $count:expr) => {{
            token!(SyntaxKind::$kind, slice!($count));
        }};
    }

    macro_rules! token {
        ($kind:expr, $source:expr) => {{
            tokens.push(($kind, $source));
        }};
    }

    macro_rules! slice {
        ($count:expr) => {{
            let start = index;
            index += $count;
            let end = index;
            &source[start..end]
        }};
    }

    macro_rules! eat_one {
        ($kind:ident if $predicate:expr) => {
            match peek!() {
                Some(c) if $predicate(c) => {
                    chars.next();

                    token!(SyntaxKind::$kind, slice!(c.len_utf8()));
                }
                _ => {}
            }
        };
    }

    macro_rules! starts_with {
        ($prefix:expr) => {{ source[index..].starts_with($prefix) }};
    }

    'lex: loop {
        let current_mode = modes.last().copied().unwrap_or(Mode::Base { depth: 0 });
        match current_mode {
            Mode::Base { depth } => {
                let Some(c) = chars.next() else {
                    break 'lex;
                };
                match c {
                    '\'' => {
                        one!(CHARACTER_START);

                        match (peek!(), nth!(1), nth!(2)) {
                            // Normal character
                            (Some(c), _, _) if !matches!(c, '\'' | '\\' | '\n' | '\t') => {
                                chars.next();

                                one!(CHARACTER_SEGMENT, c.len_utf8());
                            }
                            // Escape sequences
                            (Some('\\'), Some('\\'), _)
                            | (Some('\\'), Some('\''), _)
                            | (Some('\\'), Some('"'), _)
                            | (Some('\\'), Some('n'), _)
                            | (Some('\\'), Some('t'), _) => {
                                chars.next(); // skip '\'
                                chars.next(); // skip escaped char

                                token!(SyntaxKind::CHARACTER_SEGMENT, slice!(2));
                            }
                            // Unicode escape sequence
                            (Some('\\'), Some('u'), Some('{')) => {
                                chars.next(); // skip '\'
                                chars.next(); // skip 'u'
                                chars.next(); // skip '{'

                                let mut peekable = chars.clone();
                                let mut count = 3;
                                match peekable.next().filter(|c| c.is_ascii_hexdigit()) {
                                    Some(_) => {
                                        count += 1;
                                        chars.next(); // skip first hex digit
                                    }
                                    _ => {
                                        token!(SyntaxKind::ERROR, slice!(count));
                                        continue 'lex;
                                    }
                                }
                                for _ in 0..5 {
                                    match peekable.next().filter(|c| c.is_ascii_hexdigit()) {
                                        Some(_) => {
                                            count += 1;
                                            chars.next(); // skip hex digit
                                        }
                                        _ => break,
                                    }
                                }

                                let mut peekable = chars.clone();
                                match peekable.next() {
                                    Some('}') => {
                                        count += 1;
                                        chars.next(); // skip '}'
                                        token!(SyntaxKind::CHARACTER_SEGMENT, slice!(count));
                                    }
                                    _ => {
                                        token!(SyntaxKind::ERROR, slice!(count));
                                        continue 'lex;
                                    }
                                }
                            }
                            // Invalid character
                            _ => continue 'lex,
                        }

                        eat_one!(CHARACTER_END if |c| matches!(c, '\''));
                    }
                    '"' => {
                        match (peek!(), nth!(1)) {
                            // Triple-quoted string start
                            (Some('"'), Some('"')) => {
                                chars.next(); // skip second '"'
                                chars.next(); // skip third '"'

                                one!(STRING_START, "\"\"\"".len());
                                modes.push(Mode::InString {
                                    depth,
                                    multiline: true,
                                    allow_interpolation: true,
                                });
                                continue 'lex;
                            }
                            _ => {
                                one!(STRING_START);
                                modes.push(Mode::InString {
                                    depth,
                                    multiline: false,
                                    allow_interpolation: true,
                                });
                                continue 'lex;
                            }
                        }
                    }
                    prefix @ '0'..='9' => {
                        match (prefix, peek!()) {
                            ('0', Some('b')) => {
                                chars.next(); // skip 'b' or 'B'

                                one!(BINARY_START, 2);

                                loop {
                                    let mut peekable = chars.clone();
                                    match peekable.next() {
                                        Some(c) if matches!(c, '0' | '1') => {
                                            chars.next(); // skip first binary digit

                                            let mut count = 1;
                                            while let Some(_) =
                                                peekable.next().filter(|c| matches!(c, '0' | '1'))
                                            {
                                                count += 1;
                                                chars.next(); // skip binary digit
                                            }

                                            one!(BINARY_SEGMENT, count);
                                        }
                                        Some('_') => {
                                            chars.next(); // skip '_'

                                            let mut count = 1;
                                            while let Some('_') = peekable.next() {
                                                count += 1;
                                                chars.next(); // skip '_'
                                            }

                                            one!(NUMERIC_SEPARATOR, count);
                                        }
                                        _ => continue 'lex,
                                    }
                                }
                            }
                            ('0', Some('o')) => {
                                chars.next(); // skip 'o'

                                one!(OCTAL_START, 2);

                                loop {
                                    let mut peekable = chars.clone();
                                    match peekable.next() {
                                        Some(c) if matches!(c, '0'..='7') => {
                                            chars.next(); // skip first octal digit

                                            let mut count = 1;
                                            while let Some(_) =
                                                peekable.next().filter(|c| matches!(c, '0'..='7'))
                                            {
                                                count += 1;
                                                chars.next(); // skip octal digit
                                            }

                                            one!(OCTAL_SEGMENT, count);
                                        }
                                        Some('_') => {
                                            chars.next(); // skip '_'

                                            let mut count = 1;
                                            while let Some('_') = peekable.next() {
                                                count += 1;
                                                chars.next(); // skip '_'
                                            }

                                            one!(NUMERIC_SEPARATOR, count);
                                        }
                                        _ => continue 'lex,
                                    }
                                }
                            }
                            ('0', Some('x')) => {
                                chars.next(); // skip 'x'

                                one!(HEX_START, 2);

                                loop {
                                    let mut peekable = chars.clone();
                                    let mut count = 1;
                                    match peekable.next() {
                                        Some(c) if c.is_ascii_hexdigit() => {
                                            chars.next(); // skip first hex digit

                                            while let Some(_) =
                                                peekable.next().filter(|c| c.is_ascii_hexdigit())
                                            {
                                                count += 1;
                                                chars.next(); // skip hex digit
                                            }

                                            one!(HEX_SEGMENT, count);
                                        }
                                        Some('_') => {
                                            chars.next(); // skip '_'

                                            while let Some('_') = peekable.next() {
                                                count += 1;
                                                chars.next(); // skip '_'
                                            }

                                            one!(NUMERIC_SEPARATOR, count);
                                        }
                                        _ => continue 'lex,
                                    }
                                }
                            }
                            // Decimal or floating-point number
                            (_, _) => {
                                // Integer part
                                let mut peekable = chars.clone();
                                let mut count = 1;
                                while let Some(_) = peekable.next().filter(|c| is_numeric(*c)) {
                                    count += 1;
                                    chars.next(); // skip digit
                                }

                                one!(INTEGER_SEGMENT, count);

                                loop {
                                    let mut peekable = chars.clone();
                                    let mut count = 1;
                                    match peekable.next() {
                                        Some(c) if is_numeric(c) => {
                                            chars.next();

                                            while let Some(_) =
                                                peekable.next().filter(|c| is_numeric(*c))
                                            {
                                                count += 1;
                                                chars.next(); // skip digit
                                            }

                                            one!(INTEGER_SEGMENT, count);
                                        }
                                        Some('_') => {
                                            chars.next();

                                            while let Some('_') = peekable.next() {
                                                count += 1;
                                                chars.next(); // skip '_'
                                            }

                                            one!(NUMERIC_SEPARATOR, count);
                                        }
                                        _ => break,
                                    }
                                }

                                // Fractional part
                                if matches!(peek!(), Some('.'))
                                    && matches!(nth!(1), Some(c) if is_numeric(c) || c == '_')
                                {
                                    chars.next(); // skip '.'

                                    one!(FRACTIONAL_PREFIX);

                                    loop {
                                        let mut peekable = chars.clone();
                                        let mut count = 1;
                                        match peekable.next() {
                                            Some(c) if is_numeric(c) => {
                                                chars.next();

                                                while let Some(_) =
                                                    peekable.next().filter(|c| is_numeric(*c))
                                                {
                                                    count += 1;
                                                    chars.next(); // skip digit
                                                }

                                                one!(FRACTIONAL_SEGMENT, count);
                                            }
                                            Some('_') => {
                                                chars.next();

                                                while let Some('_') = peekable.next() {
                                                    count += 1;
                                                    chars.next(); // skip '_'
                                                }

                                                one!(NUMERIC_SEPARATOR, count);
                                            }
                                            _ => break,
                                        }
                                    }
                                }

                                // Exponential part
                                if matches!(peek!(), Some('e') | Some('E')) {
                                    chars.next(); // skip 'e' or 'E'

                                    one!(EXPONENTIAL_PREFIX);

                                    if matches!(peek!(), Some('+') | Some('-')) {
                                        chars.next(); // skip '+' or '-'

                                        one!(EXPONENT_SIGN);
                                    }

                                    loop {
                                        let mut peekable = chars.clone();
                                        let mut count = 1;
                                        match peekable.next() {
                                            Some(c) if is_numeric(c) => {
                                                chars.next();

                                                while let Some(_) =
                                                    peekable.next().filter(|c| is_numeric(*c))
                                                {
                                                    count += 1;
                                                    chars.next(); // skip digit
                                                }

                                                one!(EXPONENTIAL_SEGMENT, count);
                                            }
                                            Some('_') => {
                                                chars.next();

                                                while let Some('_') = peekable.next() {
                                                    count += 1;
                                                    chars.next(); // skip '_'
                                                }

                                                one!(NUMERIC_SEPARATOR, count);
                                            }
                                            _ => break,
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c if is_identifier_start(c) => {
                        let source = source!(c.len_utf8(), is_identifier_continue);
                        token!(keyword(source), source);
                    }
                    c if is_whitespace(c) => many!(WHITESPACE, c.len_utf8(), is_whitespace),
                    c if is_tab(c) => many!(TAB, c.len_utf8(), is_tab),
                    '\n' => one!(NEW_LINE),
                    '\\' => one!(BACKSLASH),
                    '-' => one!(HYPHEN),
                    ',' => one!(COMMA),
                    ';' => one!(SEMICOLON),
                    ':' => one!(COLON),
                    '!' => one!(EXCLAMATION),
                    '?' => one!(QUESTION),
                    '.' => one!(DOT),
                    '(' => one!(LEFT_PAREN),
                    ')' => one!(RIGHT_PAREN),
                    '[' => one!(LEFT_BRACKET),
                    ']' => one!(RIGHT_BRACKET),
                    '{' => one!(LEFT_BRACE),
                    '}' if depth != 0 => {
                        one!(INTERPOLATION_END);
                        modes.pop();
                    }
                    '}' => one!(RIGHT_BRACE),
                    '@' => one!(AT),
                    '*' => one!(ASTERISK),
                    '/' => one!(SLASH),
                    '&' => one!(AMPERSAND),
                    '#' => one!(HASH),
                    '%' => one!(PERCENT),
                    '`' => one!(BACKTICK),
                    '^' => one!(CARET),
                    '+' => one!(PLUS),
                    '<' => one!(LEFT_CHEVRON),
                    '=' => one!(EQUAL),
                    '>' => one!(RIGHT_CHEVRON),
                    '|' => one!(PIPE),
                    '~' => one!(TILDE),
                    '$' => one!(DOLLAR),
                    other => many!(ERROR, other.len_utf8(), unexpected),
                }
            }
            Mode::InString {
                depth,
                multiline: false,
                allow_interpolation,
            } => {
                if starts_with!("\"") {
                    chars.next(); // skip '"'

                    one!(STRING_END);
                    modes.pop();
                    continue 'lex;
                }

                // Error: Newline in string
                if starts_with!("\n") {
                    modes.pop();
                    continue 'lex;
                }

                if allow_interpolation && starts_with!("${") {
                    chars.next(); // skip '$'
                    chars.next(); // skip '{'

                    one!(INTERPOLATION_START, "${".len());
                    modes.push(Mode::Base { depth: depth + 1 });
                    continue 'lex;
                }

                let mut count = 0;
                loop {
                    let source = &source[index + count..];
                    if source.starts_with('"') || source.starts_with('\n') {
                        break;
                    }
                    if allow_interpolation && source.starts_with("${") {
                        break;
                    }

                    chars.next();
                    count += 1;
                }

                one!(STRING_SEGMENT, count);
            }
            Mode::InString {
                depth,
                multiline: true,
                allow_interpolation,
            } => {
                if starts_with!("\"\"\"") {
                    chars.next(); // skip first '"'
                    chars.next(); // skip second '"'
                    chars.next(); // skip third '"'

                    one!(STRING_END, "\"\"\"".len());
                    modes.pop();
                    continue 'lex;
                }

                if allow_interpolation && starts_with!("${") {
                    chars.next(); // skip '$'
                    chars.next(); // skip '{'

                    one!(INTERPOLATION_START, "${".len());
                    modes.push(Mode::Base { depth: depth + 1 });
                    continue 'lex;
                }

                let mut count = 0;
                loop {
                    let source = &source[index + count..];
                    if source.starts_with("\"\"\"") {
                        break;
                    }
                    if allow_interpolation && source.starts_with("${") {
                        break;
                    }

                    chars.next();
                    count += 1;
                }
                one!(STRING_SEGMENT, count);
            }
        }
    }

    tokens
}

const fn is_whitespace(c: char) -> bool {
    matches!(c, ' ')
}

const fn is_tab(c: char) -> bool {
    matches!(c, '\t')
}

fn is_identifier_start(c: char) -> bool {
    matches!(c, '_') || is_xid_start(c)
}

fn is_identifier_continue(c: char) -> bool {
    matches!(c, '_') || is_xid_continue(c) || is_numeric(c)
}

fn unexpected(c: char) -> bool {
    is_numeric(c) || is_identifier_start(c) || is_punctuation(c)
}

const fn is_numeric(c: char) -> bool {
    matches!(c, '0'..='9')
}

const fn is_punctuation(c: char) -> bool {
    matches!(
        c,
        ' ' | '\t'
            | '\n'
            | '\\'
            | '-'
            | ','
            | ';'
            | ':'
            | '!'
            | '?'
            | '.'
            | '('
            | ')'
            | '['
            | ']'
            | '{'
            | '}'
            | '@'
            | '*'
            | '/'
            | '&'
            | '#'
            | '%'
            | '`'
            | '^'
            | '+'
            | '<'
            | '='
            | '>'
            | '|'
            | '~'
            | '$'
    )
}

fn keyword(identifier: &str) -> SyntaxKind {
    match identifier {
        "fn" => SyntaxKind::FN,
        "let" => SyntaxKind::LET,
        "true" => SyntaxKind::TRUE,
        "false" => SyntaxKind::FALSE,
        "pub" => SyntaxKind::PUB,
        "crate" => SyntaxKind::CRATE,
        "super" => SyntaxKind::SUPER,
        "in" => SyntaxKind::IN,
        "type" => SyntaxKind::TYPE,
        "where" => SyntaxKind::WHERE,
        "struct" => SyntaxKind::STRUCT,
        "enum" => SyntaxKind::ENUM,
        "trait" => SyntaxKind::TRAIT,
        "impl" => SyntaxKind::IMPL,
        "const" => SyntaxKind::CONST,
        "static" => SyntaxKind::STATIC,
        "use" => SyntaxKind::USE,
        "mod" => SyntaxKind::MOD,
        "self" => SyntaxKind::SELF,
        "Self" => SyntaxKind::SELF_UPPERCASE,
        "as" => SyntaxKind::AS,
        "for" => SyntaxKind::FOR,
        "if" => SyntaxKind::IF,
        "else" => SyntaxKind::ELSE,
        "match" => SyntaxKind::MATCH,
        "mut" => SyntaxKind::MUT,
        "loop" => SyntaxKind::LOOP,
        "while" => SyntaxKind::WHILE,
        "return" => SyntaxKind::RETURN,
        "break" => SyntaxKind::BREAK,
        "continue" => SyntaxKind::CONTINUE,
        "await" => SyntaxKind::AWAIT,
        "yield" => SyntaxKind::YIELD,
        _ => SyntaxKind::IDENTIFIER,
    }
}

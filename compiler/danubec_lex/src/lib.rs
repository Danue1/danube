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
            let mut peekable = chars.clone();
            let mut count = $count;
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
                    // Single-quoted character
                    '\'' if !matches!(peek!(), Some('\'' | '\\' | '\n' | '\t')) => {
                        one!(CHARACTER_START, 1);

                        if let Some(c) = chars.next() {
                            one!(CHARACTER_SEGMENT, c.len_utf8());
                        }

                        eat_one!(CHARACTER_END if |c| matches!(c, '\''));
                    }
                    // Escaped character
                    '\'' if matches!(peek!(), Some('\\'))
                        && matches!(nth!(1), Some('\\' | '\'' | '"' | 'n' | 't')) =>
                    {
                        one!(CHARACTER_START, 1);

                        chars.next(); // skip '\'
                        chars.next(); // skip escaped char

                        one!(CHARACTER_SEGMENT, 2);

                        eat_one!(CHARACTER_END if |c| matches!(c, '\''));
                    }
                    '\'' if matches!(peek!(), Some('\\'))
                        && matches!(nth!(1), Some('u'))
                        && matches!(nth!(2), Some('{')) =>
                    {
                        one!(CHARACTER_START, 1);

                        chars.next(); // skip '\'
                        chars.next(); // skip 'u'
                        chars.next(); // skip '{'
                        one!(UNICODE_START, 3);

                        let source = source!(0, |c: char| c.is_ascii_hexdigit());
                        if source.is_empty() {
                            continue 'lex;
                        }
                        token!(SyntaxKind::UNICODE_SEGMENT, source);

                        loop {
                            let source = source!(0, |c: char| c == '_');
                            if source.is_empty() {
                                break;
                            }
                            token!(SyntaxKind::NUMERIC_SEPARATOR, source);

                            let source = source!(0, |c: char| c.is_ascii_hexdigit());
                            if source.is_empty() {
                                break;
                            }
                            token!(SyntaxKind::UNICODE_SEGMENT, source);
                        }

                        if let Some('}') = peek!() {
                            chars.next(); // skip '}'

                            one!(UNICODE_END, 1);
                            eat_one!(CHARACTER_END if |c| matches!(c, '\''));
                        }
                    }
                    // Error: Empty character
                    '\'' => one!(CHARACTER_START),
                    // Triple-quoted string
                    '"' if matches!(peek!(), Some('"')) && matches!(nth!(1), Some('"')) => {
                        chars.next(); // skip second '"'
                        chars.next(); // skip third '"'

                        one!(STRING_START, "\"\"\"".len());
                        modes.push(Mode::InString {
                            depth,
                            multiline: true,
                            allow_interpolation: true,
                        });
                    }
                    // One-double-quoted string
                    '"' => {
                        one!(STRING_START);
                        modes.push(Mode::InString {
                            depth,
                            multiline: false,
                            allow_interpolation: true,
                        });
                    }
                    // Binary number
                    '0' if matches!(peek!(), Some('b')) => {
                        chars.next(); // skip 'b'

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
                    // Octal number
                    '0' if matches!(peek!(), Some('o')) => {
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
                    // Hexadecimal number
                    '0' if matches!(peek!(), Some('x')) => {
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
                    '0'..='9' => {
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

                                    while let Some(_) = peekable.next().filter(|c| is_numeric(*c)) {
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
                    // Interpolation end
                    '}' if depth != 0 => {
                        one!(INTERPOLATION_END);
                        modes.pop();
                    }
                    '}' => one!(RIGHT_BRACE),
                    '@' => one!(AT),
                    '*' => one!(ASTERISK),
                    // Line comment
                    '/' if matches!(peek!(), Some('/')) => {
                        chars.next(); // skip second '/'

                        one!(LINE_COMMENT_START, 2);

                        let source = source!(0, |c| matches!(c, ' ' | '\t'));
                        if !source.is_empty() {
                            token!(SyntaxKind::WHITESPACE, source);
                        }

                        let source = source!(0, |c| !matches!(c, '\n'));
                        if !source.is_empty() {
                            token!(SyntaxKind::LINE_COMMENT_SEGMENT, source);
                        }
                    }
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

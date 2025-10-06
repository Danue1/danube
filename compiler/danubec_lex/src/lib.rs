#![warn(clippy::all)]

#[cfg(test)]
mod tests;

use danubec_syntax::SyntaxKind;
use unicode_ident::{is_xid_continue, is_xid_start};

enum Mode {
    Base {
        depth: usize,
        in_interpolation: bool,
    },
    InString {
        depth: usize,
        multiline: bool,
    },
    InRawString {
        pattern: String,
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

    'lex: while peek!().is_some() {
        let current_mode = modes.last().unwrap_or(&Mode::Base {
            depth: 0,
            in_interpolation: false,
        });
        match current_mode {
            &Mode::Base {
                depth,
                in_interpolation,
            } => {
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
                        chars.next(); // skip '\'
                        chars.next(); // skip escaped char

                        one!(CHARACTER_START, 1); // skip '\''
                        one!(ESCAPE_START, 1); // skip '\'
                        one!(ESCAPE_SEGMENT, 1); // skip escaped char
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
                        });
                    }
                    // One-double-quoted string
                    '"' => {
                        one!(STRING_START);
                        modes.push(Mode::InString {
                            depth,
                            multiline: false,
                        });
                    }
                    // Binary number
                    '0' if matches!(peek!(), Some('b')) => {
                        chars.next(); // skip 'b'

                        one!(BINARY_START, 2);

                        loop {
                            let segment = source!(0, |c| matches!(c, '0' | '1'));
                            if !segment.is_empty() {
                                token!(SyntaxKind::BINARY_SEGMENT, segment);
                            }

                            let separator = source!(0, |c| c == '_');
                            if !separator.is_empty() {
                                token!(SyntaxKind::NUMERIC_SEPARATOR, separator);
                            }

                            if segment.is_empty() && separator.is_empty() {
                                break;
                            }
                        }
                    }
                    // Octal number
                    '0' if matches!(peek!(), Some('o')) => {
                        chars.next(); // skip 'o'

                        one!(OCTAL_START, 2);

                        loop {
                            let segment = source!(0, |c| matches!(c, '0'..='7'));
                            if !segment.is_empty() {
                                token!(SyntaxKind::OCTAL_SEGMENT, segment);
                            }

                            let separator = source!(0, |c| c == '_');
                            if !separator.is_empty() {
                                token!(SyntaxKind::NUMERIC_SEPARATOR, separator);
                            }

                            if segment.is_empty() && separator.is_empty() {
                                break;
                            }
                        }
                    }
                    // Hexadecimal number
                    '0' if matches!(peek!(), Some('x')) => {
                        chars.next(); // skip 'x'

                        one!(HEX_START, 2);

                        loop {
                            let segment = source!(0, |c: char| c.is_ascii_hexdigit());
                            if !segment.is_empty() {
                                token!(SyntaxKind::HEX_SEGMENT, segment);
                            }

                            let separator = source!(0, |c| c == '_');
                            if !separator.is_empty() {
                                token!(SyntaxKind::NUMERIC_SEPARATOR, separator);
                            }

                            if segment.is_empty() && separator.is_empty() {
                                break;
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
                            let segment = source!(0, |c| is_numeric(c));
                            if !segment.is_empty() {
                                token!(SyntaxKind::INTEGER_SEGMENT, segment);
                            }

                            let separator = source!(0, |c| c == '_');
                            if !separator.is_empty() {
                                token!(SyntaxKind::NUMERIC_SEPARATOR, separator);
                            }

                            if segment.is_empty() && separator.is_empty() {
                                break;
                            }
                        }

                        // Fractional part
                        if matches!(peek!(), Some('.'))
                            && matches!(nth!(1), Some(c) if is_numeric(c) || c == '_')
                        {
                            chars.next(); // skip '.'

                            one!(FRACTION_START);

                            loop {
                                let segment = source!(0, |c| is_numeric(c));
                                if !segment.is_empty() {
                                    token!(SyntaxKind::FRACTION_SEGMENT, segment);
                                }

                                let separator = source!(0, |c| c == '_');
                                if !separator.is_empty() {
                                    token!(SyntaxKind::NUMERIC_SEPARATOR, separator);
                                }

                                if segment.is_empty() && separator.is_empty() {
                                    break;
                                }
                            }
                        }

                        // Exponential part
                        if matches!(peek!(), Some('e') | Some('E')) {
                            chars.next(); // skip 'e' or 'E'

                            one!(EXPONENT_START);

                            if matches!(peek!(), Some('+') | Some('-')) {
                                chars.next(); // skip '+' or '-'

                                one!(EXPONENT_SIGN);
                            }

                            loop {
                                let segment = source!(0, |c| is_numeric(c));
                                if !segment.is_empty() {
                                    token!(SyntaxKind::EXPONENT_SEGMENT, segment);
                                }

                                let separator = source!(0, |c| c == '_');
                                if !separator.is_empty() {
                                    token!(SyntaxKind::NUMERIC_SEPARATOR, separator);
                                }

                                if segment.is_empty() && separator.is_empty() {
                                    break;
                                }
                            }
                        }
                    }
                    // Raw string
                    'r' if matches!(peek!(), Some('"'))
                        || (matches!(peek!(), Some('#')) && matches!(nth!(1), Some('"' | '#'))) =>
                    {
                        let mut peekable = chars.clone();
                        let mut count = 0;
                        while let Some('#') = peekable.next() {
                            count += 1;
                            chars.next(); // skip '#'
                        }

                        if let Some('"') = peek!() {
                            chars.next(); // skip '"'

                            let source = &source[index..index + 1 + count + 1];
                            token!(SyntaxKind::RAW_STRING_START, source);

                            index += 1 + count + 1;

                            modes.push(Mode::InRawString {
                                pattern: format!("\"{}", "#".repeat(count)),
                            });
                        } else {
                            let source = &source[index..index + 1 + count];
                            index += 1 + count;
                            token!(SyntaxKind::RAW_STRING_START, source);
                        }
                    }
                    // Raw Identifier
                    'r' if matches!(peek!(), Some('#')) => {
                        chars.next(); // skip '#'
                        one!(RAW_IDENTIFIER_START, 2);

                        let source = source!(0, is_identifier_continue);
                        if !source.is_empty() {
                            token!(SyntaxKind::IDENTIFIER, source);
                        }
                    }
                    c if is_identifier_start(c) => {
                        let source = source!(c.len_utf8(), is_identifier_continue);
                        token!(keyword(source), source);
                    }
                    c if is_whitespace(c) => many!(WHITESPACE, c.len_utf8(), is_whitespace),
                    c if is_tab(c) => many!(TAB, c.len_utf8(), is_tab),
                    '\n' => one!(NEW_LINE),
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
                    // Block start
                    '{' => {
                        one!(LEFT_BRACE);
                        modes.push(Mode::Base {
                            depth: depth + 1,
                            in_interpolation: false,
                        });
                    }
                    // Interpolation end
                    '}' if in_interpolation => {
                        one!(INTERPOLATION_END);
                        modes.pop();
                    }
                    // Block end
                    '}' => {
                        one!(RIGHT_BRACE);
                        modes.pop();
                    }
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
                multiline: true, ..
            } if starts_with!("\"\"\"") => {
                chars.next(); // skip first '"'
                chars.next(); // skip second '"'
                chars.next(); // skip third '"'

                one!(STRING_END, "\"\"\"".len());
                modes.pop();
            }
            Mode::InString {
                multiline: false, ..
            } if starts_with!("\"") => {
                chars.next(); // skip '"'

                one!(STRING_END);
                modes.pop();
            }
            Mode::InString {
                multiline: false, ..
            } if starts_with!("\n") => {
                modes.pop();
            }
            Mode::InString { depth, .. } if starts_with!("${") => {
                chars.next(); // skip '$'
                chars.next(); // skip '{'

                one!(INTERPOLATION_START, "${".len());
                modes.push(Mode::Base {
                    depth: depth + 1,
                    in_interpolation: true,
                });
            }
            Mode::InString { .. }
                if starts_with!("\\\\")
                    || starts_with!("\\\'")
                    || starts_with!("\\\"")
                    || starts_with!("\\n")
                    || starts_with!("\\t") =>
            {
                chars.next(); // skip '\'
                chars.next(); // skip escaped char

                one!(ESCAPE_START, 1); // skip '\'
                one!(ESCAPE_SEGMENT, 1); // skip escaped char
            }
            Mode::InString { .. } if starts_with!("\\u{") => {
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
                }
            }
            Mode::InString {
                multiline: false, ..
            } => {
                let mut count = 0;
                let mut source = &source[index + count..];
                loop {
                    if source.starts_with('"')
                        || source.starts_with('\n')
                        || source.starts_with("${")
                        || source.starts_with("\\\\")
                        || source.starts_with("\\\'")
                        || source.starts_with("\\\"")
                        || source.starts_with("\\n")
                        || source.starts_with("\\t")
                        || source.starts_with("\\u{")
                    {
                        break;
                    }

                    if let Some(c) = chars.next() {
                        count += c.len_utf8();
                        source = &source[c.len_utf8()..];
                    } else {
                        break;
                    }
                }

                one!(STRING_SEGMENT, count);
            }
            Mode::InString {
                multiline: true, ..
            } => {
                let mut count = 0;
                let mut source = &source[index + count..];
                loop {
                    if source.starts_with("\"\"\"")
                        || source.starts_with("${")
                        || source.starts_with("\\\\")
                        || source.starts_with("\\\'")
                        || source.starts_with("\\\"")
                        || source.starts_with("\\n")
                        || source.starts_with("\\t")
                        || source.starts_with("\\u{")
                    {
                        break;
                    }

                    if let Some(c) = chars.next() {
                        count += c.len_utf8();
                        source = &source[c.len_utf8()..];
                    } else {
                        break;
                    }
                }

                one!(STRING_SEGMENT, count);
            }
            Mode::InRawString { pattern } if starts_with!(pattern) => {
                for _ in 0..pattern.len() {
                    chars.next();
                }

                one!(RAW_STRING_END, pattern.len());
                modes.pop();
            }
            Mode::InRawString { pattern } => {
                let mut count = 0;
                let mut source = &source[index + count..];
                loop {
                    if source.starts_with(pattern) {
                        break;
                    }

                    if let Some(c) = chars.next() {
                        count += c.len_utf8();
                        source = &source[c.len_utf8()..];
                    } else {
                        break;
                    }
                }

                if count != 0 {
                    one!(STRING_SEGMENT, count);
                }
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
        "_" => SyntaxKind::PLACEHOLDER,
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

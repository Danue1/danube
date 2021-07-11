use crate::{Cursor, Error};
use danube_token::{Keyword, Span, Symbol, Token, TokenKind};

pub type LexResult<T> = Result<T, Error>;

pub struct LexIter<'lex> {
    cursor: Cursor<'lex>,
}

impl<'lex> LexIter<'lex> {
    pub fn new(source: &'lex str) -> Self {
        LexIter {
            cursor: Cursor::<'lex>::new(source),
        }
    }
}

impl<'lex> std::iter::Iterator for LexIter<'lex> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.peek().is_some() {
            match lex_token(&mut self.cursor) {
                Ok(None) => None,
                Ok(Some(token)) => Some(Ok(token)),
                Err(error) => Some(Err(error)),
            }
        } else {
            None
        }
    }
}

macro_rules! token {
    ($span:expr, $kind:expr) => {
        Token {
            span: $span,
            kind: $kind,
        }
    };
}

#[inline]
fn lex_token(cursor: &mut Cursor) -> LexResult<Option<Token>> {
    let token = match cursor.peek().unwrap() {
        '(' => consume_symbol(cursor, Symbol::LeftParens)?,
        ')' => consume_symbol(cursor, Symbol::RightParens)?,
        '[' => consume_symbol(cursor, Symbol::LeftBracket)?,
        ']' => consume_symbol(cursor, Symbol::RightBracket)?,
        '{' => consume_symbol(cursor, Symbol::LeftBrace)?,
        '}' => consume_symbol(cursor, Symbol::RightBrace)?,
        '<' => lex_left_chevron(cursor)?,
        '>' => lex_right_chevron(cursor)?,
        '#' => consume_symbol(cursor, Symbol::Hash)?,
        '.' => lex_dot(cursor)?,
        ',' => consume_symbol(cursor, Symbol::Comma)?,
        ':' => lex_colon(cursor)?,
        ';' => consume_symbol(cursor, Symbol::Semicolon)?,
        '=' => lex_eq(cursor)?,
        '+' => lex_plus(cursor)?,
        '-' => lex_hyphen(cursor)?,
        '*' => lex_asterisk(cursor)?,
        '/' => match lex_slash(cursor)? {
            Some(token) => token,
            None => return Ok(None),
        },
        '%' => lex_percent(cursor)?,
        '!' => lex_exclamation(cursor)?,
        '?' => consume_symbol(cursor, Symbol::Question)?,
        '&' => lex_ampersand(cursor)?,
        '|' => lex_pipeline(cursor)?,
        '~' => lex_tilde(cursor)?,
        '^' => lex_caret(cursor)?,
        ' ' | '\r' | '\n' | '\t' => {
            skip_whitespace(cursor);
            return Ok(None);
        }
        '"' => lex_string(cursor)?,
        '\'' => lex_char(cursor)?,
        '0'..='9' => lex_numeric(cursor)?,
        'a'..='z' | 'A'..='Z' | '_' => lex_identifier(cursor)?,
        c => return Err(Error::Illegal(cursor.position(), c)),
    };

    Ok(Some(token))
}

#[inline]
fn lex_left_chevron(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('<') => {
                cursor.next();
                match cursor.peek() {
                    Some('=') => {
                        cursor.next();
                        Ok(Symbol::LeftChevronLeftChevronEq)
                    }
                    _ => Ok(Symbol::LeftChevronLeftChevron),
                }
            }
            Some('=') => {
                cursor.next();
                Ok(Symbol::LeftChevronEq)
            }
            _ => Ok(Symbol::LeftChevron),
        }
    })?;

    Ok(token!(span, TokenKind::Symbol(symbol)))
}

#[inline]
fn lex_right_chevron(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('>') => {
                cursor.next();
                match cursor.peek() {
                    Some('=') => {
                        cursor.next();
                        Ok(Symbol::RightChevronRightChevronEq)
                    }
                    _ => Ok(Symbol::RightChevronRightChevron),
                }
            }
            Some('=') => {
                cursor.next();
                Ok(Symbol::RightChevronEq)
            }
            _ => Ok(Symbol::RightChevron),
        }
    })?;

    Ok(token!(span, TokenKind::Symbol(symbol)))
}

#[inline]
fn lex_dot(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('.') => {
                cursor.next();
                match cursor.peek() {
                    Some('=') => {
                        cursor.next();
                        Ok(Some(Symbol::DotDotEq))
                    }
                    _ => Ok(Some(Symbol::DotDot)),
                }
            }
            Some('0'..='9') => Ok(None),
            _ => Ok(Some(Symbol::Dot)),
        }
    })?;
    match symbol {
        Some(symbol) => Ok(token!(span, TokenKind::Symbol(symbol))),
        None => lex_float(cursor, span),
    }
}

#[inline]
fn lex_colon(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some(':') => {
                cursor.next();
                Ok(Symbol::ColonColon)
            }
            _ => Ok(Symbol::Colon),
        }
    })?;

    Ok(token!(span, TokenKind::Symbol(symbol)))
}

#[inline]
fn lex_eq(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('>') => {
                cursor.next();
                Ok(Symbol::EqRightChevron)
            }
            Some('=') => {
                cursor.next();
                Ok(Symbol::EqEq)
            }
            _ => Ok(Symbol::Eq),
        }
    })?;

    Ok(token!(span, TokenKind::Symbol(symbol)))
}

#[inline]
fn lex_plus(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('=') => {
                cursor.next();
                Ok(Symbol::PlusEq)
            }
            _ => Ok(Symbol::Plus),
        }
    })?;

    Ok(token!(span, TokenKind::Symbol(symbol)))
}

#[inline]
fn lex_hyphen(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('=') => {
                cursor.next();
                Ok(Symbol::HyphenEq)
            }
            Some('>') => {
                cursor.next();
                Ok(Symbol::HyphenRightChevron)
            }
            _ => Ok(Symbol::Hyphen),
        }
    })?;

    Ok(token!(span, TokenKind::Symbol(symbol)))
}

#[inline]
fn lex_asterisk(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('*') => {
                cursor.next();
                match cursor.peek() {
                    Some('=') => {
                        cursor.next();
                        Ok(Symbol::AsteriskAsteriskEq)
                    }
                    _ => Ok(Symbol::AsteriskAsterisk),
                }
            }
            Some('=') => {
                cursor.next();
                Ok(Symbol::AsteriskEq)
            }
            _ => Ok(Symbol::Asterisk),
        }
    })?;

    Ok(token!(span, TokenKind::Symbol(symbol)))
}

#[inline]
fn lex_slash(cursor: &mut Cursor) -> LexResult<Option<Token>> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('/') => {
                cursor.next();
                Ok(None)
            }
            Some('=') => {
                cursor.next();
                Ok(Some(Symbol::SlashEq))
            }
            _ => Ok(Some(Symbol::Slash)),
        }
    })?;
    match symbol {
        Some(symbol) => Ok(Some(token!(span, TokenKind::Symbol(symbol)))),
        None => {
            skip_comment(cursor);
            Ok(None)
        }
    }
}

#[inline]
fn lex_percent(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('=') => {
                cursor.next();
                Ok(Symbol::PercentEq)
            }
            _ => Ok(Symbol::Percent),
        }
    })?;

    Ok(token!(span, TokenKind::Symbol(symbol)))
}

#[inline]
fn lex_exclamation(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('=') => {
                cursor.next();
                Ok(Symbol::ExclamationEq)
            }
            _ => Ok(Symbol::Exclamation),
        }
    })?;

    Ok(token!(span, TokenKind::Symbol(symbol)))
}

#[inline]
fn lex_ampersand(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('&') => {
                cursor.next();
                match cursor.peek() {
                    Some('=') => {
                        cursor.next();
                        Ok(Symbol::AmpersandAmpersandEq)
                    }
                    _ => Ok(Symbol::AmpersandAmpersand),
                }
            }
            Some('=') => {
                cursor.next();
                Ok(Symbol::AmpersandEq)
            }
            _ => Ok(Symbol::Ampersand),
        }
    })?;

    Ok(token!(span, TokenKind::Symbol(symbol)))
}

#[inline]
fn lex_pipeline(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('|') => {
                cursor.next();
                match cursor.peek() {
                    Some('=') => {
                        cursor.next();
                        Ok(Symbol::PipelinePipelineEq)
                    }
                    _ => Ok(Symbol::PipelinePipeline),
                }
            }
            Some('>') => {
                cursor.next();
                Ok(Symbol::PipelineRightChevron)
            }
            Some('=') => {
                cursor.next();
                Ok(Symbol::PipelineEq)
            }
            _ => Ok(Symbol::Pipeline),
        }
    })?;

    Ok(token!(span, TokenKind::Symbol(symbol)))
}

#[inline]
fn lex_tilde(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('=') => {
                cursor.next();
                Ok(Symbol::TildeEq)
            }
            _ => Ok(Symbol::Tilde),
        }
    })?;

    Ok(token!(span, TokenKind::Symbol(symbol)))
}

#[inline]
fn lex_caret(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, symbol) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('=') => {
                cursor.next();
                Ok(Symbol::CaretEq)
            }
            _ => Ok(Symbol::Caret),
        }
    })?;

    Ok(token!(span, TokenKind::Symbol(symbol)))
}

#[inline]
fn lex_string(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, literal) = cursor.span_by(|cursor| {
        cursor.next();
        let (span, _) = cursor.span_by(|cursor| {
            cursor.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'));
            cursor.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'| '_'));

            Ok(())
        })?;
        match cursor.peek() {
            Some('"') => {
                cursor.next();
                Ok(cursor.slice(span.start..span.end).to_string())
            }
            Some(c) => Err(Error::Illegal(cursor.position(), c)),
            None => Err(Error::Need(cursor.position(), '"')),
        }
    })?;

    Ok(token!(span, TokenKind::StringLiteral(literal)))
}

#[inline]
fn lex_char(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, literal) = cursor.span_by(|cursor| {
        cursor.next();
        match cursor.peek() {
            Some('\\') => {
                cursor.next();
                let c = match cursor.peek() {
                    Some('r') => '\r',
                    Some('n') => '\n',
                    Some('t') => '\t',
                    _ => return Err(Error::Invalid),
                };
                cursor.next();
                match cursor.peek() {
                    Some('\'') => {
                        cursor.next();
                        Ok(c)
                    }
                    Some(c) => Err(Error::Illegal(cursor.position(), c)),
                    None => Err(Error::Need(cursor.position(), '\'')),
                }
            }
            Some(c) => {
                cursor.next();
                match cursor.peek() {
                    Some('\'') => {
                        cursor.next();
                        Ok(c)
                    }
                    Some(c) => Err(Error::Illegal(cursor.position(), c)),
                    None => Err(Error::Need(cursor.position(), '\'')),
                }
            }
            _ => Err(Error::Invalid),
        }
    })?;

    Ok(token!(span, TokenKind::CharLiteral(literal)))
}

#[inline]
fn lex_numeric(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, _) = cursor.span_by(|cursor| {
        cursor.consume_while(|c| matches!(c, '0'..='9'));

        Ok(())
    })?;
    match cursor.peek() {
        Some('.') => {
            cursor.next();
            lex_float(cursor, span)
        }
        Some(c) => Err(Error::Illegal(cursor.position(), c)),
        None if span.is_empty() => Err(Error::Invalid),
        None => {
            let literal = cursor.slice(span.start..span.end).parse().unwrap();

            Ok(token!(span, TokenKind::IntLiteral(literal)))
        }
    }
}

#[inline]
fn lex_float(cursor: &mut Cursor, integer_position: Span) -> LexResult<Token> {
    let (fractional_position, _) = cursor.span_by(|cursor| {
        cursor.consume_while(|c| matches!(c, '0'..='9'));

        Ok(())
    })?;
    if integer_position.is_empty() && fractional_position.is_empty() {
        return Err(Error::Invalid);
    }

    let literal = cursor
        .slice(integer_position.start..fractional_position.end)
        .parse()
        .unwrap();

    Ok(token!(
        Span::concat(integer_position, fractional_position),
        TokenKind::FloatLiteral(literal)
    ))
}

#[inline]
fn lex_identifier(cursor: &mut Cursor) -> LexResult<Token> {
    let (span, _) = cursor.span_by(|cursor| {
        cursor.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'));
        cursor.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));

        Ok(())
    })?;
    let literal = cursor.slice(span.start..span.end).to_owned();
    match literal_to_keyword(literal.as_str()) {
        Some(keyword) => Ok(token!(span, TokenKind::Keyword(keyword))),
        None => Ok(token!(span, TokenKind::Identifier(literal))),
    }
}

#[inline(always)]
fn skip_comment(cursor: &mut Cursor) {
    cursor.consume_while(|c| !matches!(c, '\r' | '\n'));
    cursor.consume_while(|c| matches!(c, '\r' | '\t'));
}

#[inline(always)]
fn skip_whitespace(cursor: &mut Cursor) {
    cursor.consume_while(|c| matches!(c, ' ' | '\r' | '\n' | '\t'));
}

#[inline(always)]
fn consume_symbol(cursor: &mut Cursor, symbol: Symbol) -> LexResult<Token> {
    let (span, kind) = cursor.span_by(|cursor| {
        for _ in 0..symbol.count() {
            cursor.next();
        }

        Ok(TokenKind::Symbol(symbol))
    })?;

    Ok(token!(span, kind))
}

#[inline(always)]
fn literal_to_keyword(literal: &str) -> Option<Keyword> {
    match literal {
        "if" => Some(Keyword::If),
        "else" => Some(Keyword::Else),
        "for" => Some(Keyword::For),
        "while" => Some(Keyword::While),
        "loop" => Some(Keyword::Loop),
        "in" => Some(Keyword::In),
        "break" => Some(Keyword::Break),
        "continue" => Some(Keyword::Continue),
        "match" => Some(Keyword::Match),
        "return" => Some(Keyword::Return),
        "yield" => Some(Keyword::Yield),
        "where" => Some(Keyword::Where),
        "const" => Some(Keyword::Const),
        "let" => Some(Keyword::Let),
        "mut" => Some(Keyword::Mutable),
        "fn" => Some(Keyword::Function),
        "trait" => Some(Keyword::Trait),
        "type" => Some(Keyword::Type),
        "enum" => Some(Keyword::Enum),
        "impl" => Some(Keyword::Impl),
        "mod" => Some(Keyword::Module),
        "Self" => Some(Keyword::TypeSelf),
        "self" => Some(Keyword::VariableSelf),
        "pub" => Some(Keyword::Public),
        "await" => Some(Keyword::Await),
        "use" => Some(Keyword::Use),
        "super" => Some(Keyword::Super),
        "as" => Some(Keyword::As),
        "package" => Some(Keyword::Package),
        "_" => Some(Keyword::Placeholder),
        _ => None,
    }
}

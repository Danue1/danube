use crate::Error;
use danube_token::{Keyword, Position, Symbol, Token, TokenKind};
use std::iter::Peekable;
use std::str::Chars;

pub fn lex(source: &str) -> Result<Vec<Token>, Error> {
    Context::new(source).lex_all()
}

struct Context<'lex> {
    source: &'lex str,
    peekable: Peekable<Chars<'lex>>,
    token_list: Vec<Token>,
    position: usize,
}

impl<'lex> Context<'lex> {
    fn new(source: &'lex str) -> Self {
        Context {
            source,
            peekable: source.chars().peekable(),
            token_list: vec![],
            position: 0,
        }
    }

    fn lex_all(mut self) -> Result<Vec<Token>, Error> {
        while self.peek().is_some() {
            self.lex()?;
        }

        Ok(self.token_list)
    }

    fn lex(&mut self) -> Result<(), Error> {
        match self.peek().unwrap() {
            '(' => self.consume_symbol(Symbol::LeftParens)?,
            ')' => self.consume_symbol(Symbol::RightParens)?,
            '[' => self.consume_symbol(Symbol::LeftBracket)?,
            ']' => self.consume_symbol(Symbol::RightBracket)?,
            '{' => self.consume_symbol(Symbol::LeftBrace)?,
            '}' => self.consume_symbol(Symbol::RightBrace)?,
            '<' => self.lex_left_chevron()?,
            '>' => self.lex_right_chevron()?,
            '#' => self.consume_symbol(Symbol::Hash)?,
            '.' => self.lex_dot()?,
            ',' => self.consume_symbol(Symbol::Comma)?,
            ':' => self.lex_colon()?,
            ';' => self.consume_symbol(Symbol::Semicolon)?,
            '=' => self.lex_eq()?,
            '+' => self.lex_plus()?,
            '-' => self.lex_hyphen()?,
            '*' => self.lex_asterisk()?,
            '/' => self.lex_slash()?,
            '%' => self.lex_percent()?,
            '!' => self.lex_exclamation()?,
            '?' => self.consume_symbol(Symbol::Question)?,
            '&' => self.lex_ampersand()?,
            '|' => self.lex_pipeline()?,
            '~' => self.lex_tilde()?,
            '^' => self.lex_caret()?,
            ' ' | '\r' | '\n' | '\t' => self.skip_whitespace(),
            '"' => self.lex_string()?,
            '\'' => self.lex_char()?,
            '0'..='9' => self.lex_numeric()?,
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier()?,
            c => return Err(Error::Illegal(self.position, c)),
        };

        Ok(())
    }

    fn lex_left_chevron(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('<') => {
                    this.next();
                    match this.peek() {
                        Some('=') => {
                            this.next();
                            Ok(Symbol::LeftChevronLeftChevronEq)
                        }
                        _ => Ok(Symbol::LeftChevronLeftChevron),
                    }
                }
                Some('=') => {
                    this.next();
                    Ok(Symbol::LeftChevronEq)
                }
                _ => Ok(Symbol::LeftChevron),
            }
        })?;
        self.emit_token(position, TokenKind::Symbol(symbol));

        Ok(())
    }

    fn lex_right_chevron(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('>') => {
                    this.next();
                    match this.peek() {
                        Some('=') => {
                            this.next();
                            Ok(Symbol::RightChevronRightChevronEq)
                        }
                        _ => Ok(Symbol::RightChevronRightChevron),
                    }
                }
                Some('=') => {
                    this.next();
                    Ok(Symbol::RightChevronEq)
                }
                _ => Ok(Symbol::RightChevron),
            }
        })?;
        self.emit_token(position, TokenKind::Symbol(symbol));

        Ok(())
    }

    fn lex_dot(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('.') => {
                    this.next();
                    match this.peek() {
                        Some('=') => {
                            this.next();
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
            Some(symbol) => self.emit_token(position, TokenKind::Symbol(symbol)),
            None => self.lex_float(position)?,
        }

        Ok(())
    }

    fn lex_colon(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some(':') => {
                    this.next();
                    Ok(Symbol::ColonColon)
                }
                _ => Ok(Symbol::Colon),
            }
        })?;
        self.emit_token(position, TokenKind::Symbol(symbol));

        Ok(())
    }

    fn lex_eq(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('>') => {
                    this.next();
                    Ok(Symbol::EqRightChevron)
                }
                Some('=') => {
                    this.next();
                    Ok(Symbol::EqEq)
                }
                _ => Ok(Symbol::Eq),
            }
        })?;
        self.emit_token(position, TokenKind::Symbol(symbol));

        Ok(())
    }

    fn lex_plus(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('=') => {
                    this.next();
                    Ok(Symbol::PlusEq)
                }
                _ => Ok(Symbol::Plus),
            }
        })?;
        self.emit_token(position, TokenKind::Symbol(symbol));

        Ok(())
    }

    fn lex_hyphen(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('=') => {
                    this.next();
                    Ok(Symbol::HyphenEq)
                }
                Some('>') => {
                    this.next();
                    Ok(Symbol::HyphenRightChevron)
                }
                _ => Ok(Symbol::Hyphen),
            }
        })?;
        self.emit_token(position, TokenKind::Symbol(symbol));

        Ok(())
    }

    fn lex_asterisk(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('*') => {
                    this.next();
                    match this.peek() {
                        Some('=') => {
                            this.next();
                            Ok(Symbol::AsteriskAsteriskEq)
                        }
                        _ => Ok(Symbol::AsteriskAsterisk),
                    }
                }
                Some('=') => {
                    this.next();
                    Ok(Symbol::AsteriskEq)
                }
                _ => Ok(Symbol::Asterisk),
            }
        })?;
        self.emit_token(position, TokenKind::Symbol(symbol));

        Ok(())
    }

    fn lex_slash(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('/') => {
                    this.next();
                    Ok(None)
                }
                Some('=') => {
                    this.next();
                    Ok(Some(Symbol::SlashEq))
                }
                _ => Ok(Some(Symbol::Slash)),
            }
        })?;
        match symbol {
            Some(symbol) => self.emit_token(position, TokenKind::Symbol(symbol)),
            None => {
                self.skip_comment();
            }
        }

        Ok(())
    }

    fn lex_percent(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('=') => {
                    this.next();
                    Ok(Symbol::PercentEq)
                }
                _ => Ok(Symbol::Percent),
            }
        })?;
        self.emit_token(position, TokenKind::Symbol(symbol));

        Ok(())
    }

    fn lex_exclamation(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('=') => {
                    this.next();
                    Ok(Symbol::ExclamationEq)
                }
                _ => Ok(Symbol::Exclamation),
            }
        })?;
        self.emit_token(position, TokenKind::Symbol(symbol));

        Ok(())
    }

    fn lex_ampersand(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('&') => {
                    this.next();
                    match this.peek() {
                        Some('=') => {
                            this.next();
                            Ok(Symbol::AmpersandAmpersandEq)
                        }
                        _ => Ok(Symbol::AmpersandAmpersand),
                    }
                }
                Some('=') => {
                    this.next();
                    Ok(Symbol::AmpersandEq)
                }
                _ => Ok(Symbol::Ampersand),
            }
        })?;
        self.emit_token(position, TokenKind::Symbol(symbol));

        Ok(())
    }

    fn lex_pipeline(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('|') => {
                    this.next();
                    match this.peek() {
                        Some('=') => {
                            this.next();
                            Ok(Symbol::PipelinePipelineEq)
                        }
                        _ => Ok(Symbol::PipelinePipeline),
                    }
                }
                Some('>') => {
                    this.next();
                    Ok(Symbol::PipelineRightChevron)
                }
                Some('=') => {
                    this.next();
                    Ok(Symbol::PipelineEq)
                }
                _ => Ok(Symbol::Pipeline),
            }
        })?;
        self.emit_token(position, TokenKind::Symbol(symbol));

        Ok(())
    }

    fn lex_tilde(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('=') => {
                    this.next();
                    Ok(Symbol::TildeEq)
                }
                _ => Ok(Symbol::Tilde),
            }
        })?;
        self.emit_token(position, TokenKind::Symbol(symbol));

        Ok(())
    }

    fn lex_caret(&mut self) -> Result<(), Error> {
        let (position, symbol) = self.position_by(|this| {
            this.next();
            match this.peek() {
                Some('=') => {
                    this.next();
                    Ok(Symbol::CaretEq)
                }
                _ => Ok(Symbol::Caret),
            }
        })?;
        self.emit_token(position, TokenKind::Symbol(symbol));

        Ok(())
    }

    fn lex_string(&mut self) -> Result<(), Error> {
        let (position, literal) = self.position_by(|this| {
            this.next();
            let (position, _) = this.position_by(|that| {
                that.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'));
                that.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'| '_'));

                Ok(())
            })?;
            match this.peek() {
                Some('"') => {
                    this.next();
                    Ok(this.source[position.start..position.end].to_string())
                }
                Some(c) => Err(Error::Illegal(this.position, c)),
                None => Err(Error::Need(this.position, '"')),
            }
        })?;
        self.emit_token(position, TokenKind::StringLiteral(literal));

        Ok(())
    }

    fn lex_char(&mut self) -> Result<(), Error> {
        let (position, literal) = self.position_by(|this| {
            this.next();
            let c = match this.peek() {
                Some('\\') => {
                    this.next();
                    let c = match this.peek() {
                        Some('r') => '\r',
                        Some('n') => '\n',
                        Some('t') => '\t',
                        _ => return Err(Error::Invalid),
                    };
                    this.next();
                    match this.peek() {
                        Some('\'') => {
                            this.next();
                            c
                        }
                        Some(c) => return Err(Error::Illegal(this.position, c)),
                        None => return Err(Error::Need(this.position, '\'')),
                    }
                }
                Some(c) => {
                    this.next();
                    match this.peek() {
                        Some('\'') => {
                            this.next();
                            c
                        }
                        Some(c) => return Err(Error::Illegal(this.position, c)),
                        None => return Err(Error::Need(this.position, '\'')),
                    }
                }
                _ => return Err(Error::Invalid),
            };
            Ok(c)
        })?;
        self.emit_token(position, TokenKind::CharLiteral(literal));

        Ok(())
    }

    fn lex_numeric(&mut self) -> Result<(), Error> {
        let (position, _) = self.position_by(|this| {
            this.consume_while(|c| matches!(c, '0'..='9'));

            Ok(())
        })?;
        match self.peek() {
            Some('.') => {
                self.next();
                self.lex_float(position)?;
            }
            Some(c) => return Err(Error::Illegal(self.position, c)),
            None => {
                if position.is_empty() {
                    return Err(Error::Invalid);
                }

                let literal = &self.source[position.start..position.end];
                self.emit_token(position, TokenKind::IntLiteral(literal.parse().unwrap()));
            }
        };

        Ok(())
    }

    fn lex_float(&mut self, integer_position: Position) -> Result<(), Error> {
        let (fractional_position, _) = self.position_by(|this| {
            this.consume_while(|c| matches!(c, '0'..='9'));

            Ok(())
        })?;
        if integer_position.is_empty() && fractional_position.is_empty() {
            return Err(Error::Invalid);
        }

        let literal = &self.source[integer_position.start..fractional_position.end];
        self.emit_token(
            Position::concat(integer_position, fractional_position),
            TokenKind::FloatLiteral(literal.parse().unwrap()),
        );

        Ok(())
    }

    fn lex_identifier(&mut self) -> Result<(), Error> {
        let (position, _) = self.position_by(|this| {
            this.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'));
            this.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));

            Ok(())
        })?;
        let literal = &self.source[position.start..position.end];
        match literal_to_keyword(literal) {
            Some(keyword) => self.emit_token(position, TokenKind::Keyword(keyword)),
            None => self.emit_token(position, TokenKind::Identifier(literal.to_owned())),
        };

        Ok(())
    }

    #[inline(always)]
    fn skip_comment(&mut self) {
        self.consume_while(|c| !matches!(c, '\r' | '\n'));
        self.consume_while(|c| matches!(c, '\r' | '\t'));
    }

    #[inline(always)]
    fn skip_whitespace(&mut self) {
        self.consume_while(|c| matches!(c, ' ' | '\r' | '\n' | '\t'));
    }

    #[inline(always)]
    fn consume_symbol(&mut self, symbol: Symbol) -> Result<(), Error> {
        let (position, kind) = self.position_by(|this| {
            for _ in 0..symbol.count() {
                this.next();
            }

            Ok(TokenKind::Symbol(symbol))
        })?;

        self.emit_token(position, kind);

        Ok(())
    }

    #[inline(always)]
    fn consume_while<F>(&mut self, predicate: F)
    where
        F: Fn(char) -> bool,
    {
        while let Some(peek) = self.peek() {
            if predicate(peek) {
                self.next();
            } else {
                break;
            }
        }
    }

    #[inline(always)]
    fn position_by<T, F>(&mut self, f: F) -> Result<(Position, T), Error>
    where
        F: FnOnce(&mut Self) -> Result<T, Error>,
    {
        let start = self.position;
        let ret = f(self)?;
        let end = self.position;

        Ok((Position::new(start, end), ret))
    }

    #[inline(always)]
    fn emit_token(&mut self, position: Position, kind: TokenKind) {
        self.token_list.push(Token { position, kind });
    }

    #[inline(always)]
    fn next(&mut self) {
        self.position += 1;
        self.peekable.next();
    }

    #[inline(always)]
    fn peek(&mut self) -> Option<char> {
        self.peekable.peek().cloned()
    }
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
        "static" => Some(Keyword::Static),
        "let" => Some(Keyword::Let),
        "mut" => Some(Keyword::Mut),
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
        "_" => Some(Keyword::Placeholder),
        _ => None,
    }
}

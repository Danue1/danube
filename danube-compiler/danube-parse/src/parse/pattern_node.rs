use crate::{Error, Parse};
use danube_ast::{PatternKind, PatternNode};
use danube_token::{keywords, TokenKind};

impl<'parse> Parse<'parse> {
    pub fn parse_pattern_node(&mut self) -> Result<PatternNode, Error> {
        match self.cursor.peek().kind {
            TokenKind::DotDot => {
                self.cursor.next();

                Ok(PatternNode {
                    kind: PatternKind::Rest,
                })
            }
            TokenKind::Identifier(keywords::Placeholder) => {
                self.cursor.next();

                Ok(PatternNode {
                    kind: PatternKind::Wildcard,
                })
            }
            TokenKind::Literal(symbol, ref kind) => {
                let kind = PatternKind::Literal(symbol, kind.clone());

                self.cursor.next();

                Ok(PatternNode { kind })
            }
            TokenKind::LeftBracket => {
                self.cursor.next();

                let mut patterns = Vec::new();

                while !symbol!(self.cursor => RightBracket) {
                    patterns.push(self.parse_pattern_node()?);
                    if !symbol!(self.cursor => Comma) {
                        break;
                    }
                }

                if symbol!(self.cursor => RightBracket) {
                    Ok(PatternNode {
                        kind: PatternKind::Slice(patterns),
                    })
                } else {
                    Err(Error::Invalid)
                }
            }
            TokenKind::LeftParens => {
                self.cursor.next();

                let mut fields = Vec::new();

                while !symbol!(self.cursor => RightParens) {
                    fields.push(self.parse_pattern_node()?);
                    if !symbol!(self.cursor => Comma) {
                        break;
                    }
                }

                if symbol!(self.cursor => RightParens) {
                    Ok(PatternNode {
                        kind: PatternKind::UnnamedStruct(None, fields),
                    })
                } else {
                    Err(Error::Invalid)
                }
            }
            _ => {
                let path = if let Some(path) = self.parse_path_node()? {
                    path
                } else {
                    return Err(Error::Invalid);
                };

                match self.cursor.peek().kind {
                    TokenKind::LeftBrace => {
                        self.cursor.next();

                        let mut fields = Vec::new();

                        while !symbol!(self.cursor => RightBrace) {
                            let path = if let Some(path) = self.parse_path_node()? {
                                path
                            } else {
                                return Err(Error::Invalid);
                            };
                            let pattern = if symbol!(self.cursor => Colon) {
                                Some(self.parse_pattern_node()?)
                            } else {
                                None
                            };
                            fields.push((path, pattern));

                            if !symbol!(self.cursor => Comma) {
                                break;
                            }
                        }
                        if symbol!(self.cursor => RightBrace) {
                            Ok(PatternNode {
                                kind: PatternKind::NamedStruct(path, fields),
                            })
                        } else {
                            Err(Error::Invalid)
                        }
                    }
                    TokenKind::LeftParens => {
                        self.cursor.next();

                        let mut fields = Vec::new();

                        while !symbol!(self.cursor => RightParens) {
                            fields.push(self.parse_pattern_node()?);
                            if !symbol!(self.cursor => Comma) {
                                break;
                            }
                        }

                        if symbol!(self.cursor => RightParens) {
                            Ok(PatternNode {
                                kind: PatternKind::UnnamedStruct(Some(path), fields),
                            })
                        } else {
                            Err(Error::Invalid)
                        }
                    }
                    _ => Ok(PatternNode {
                        kind: PatternKind::Path(path),
                    }),
                }
            }
        }
    }
}

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
                        if symbol!(self.cursor => RightBracket) {
                            break;
                        }

                        return Err(Error::Invalid);
                    }
                }

                Ok(PatternNode {
                    kind: PatternKind::Slice(patterns),
                })
            }
            TokenKind::LeftParens => {
                self.cursor.next();

                let mut fields = Vec::new();

                while !symbol!(self.cursor => RightParens) {
                    fields.push(self.parse_pattern_node()?);

                    if !symbol!(self.cursor => Comma) {
                        if symbol!(self.cursor => RightParens) {
                            break;
                        }

                        return Err(Error::Invalid);
                    }
                }

                Ok(PatternNode {
                    kind: PatternKind::UnnamedStruct(None, fields),
                })
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
                                if symbol!(self.cursor => RightBrace) {
                                    break;
                                }

                                return Err(Error::Invalid);
                            }
                        }

                        Ok(PatternNode {
                            kind: PatternKind::NamedStruct(path, fields),
                        })
                    }
                    TokenKind::LeftParens => {
                        self.cursor.next();

                        let mut fields = Vec::new();

                        while !symbol!(self.cursor => RightParens) {
                            fields.push(self.parse_pattern_node()?);

                            if !symbol!(self.cursor => Comma) {
                                if symbol!(self.cursor => RightParens) {
                                    break;
                                }

                                return Err(Error::Invalid);
                            }
                        }

                        Ok(PatternNode {
                            kind: PatternKind::UnnamedStruct(Some(path), fields),
                        })
                    }
                    _ => Ok(PatternNode {
                        kind: PatternKind::Path(path),
                    }),
                }
            }
        }
    }
}

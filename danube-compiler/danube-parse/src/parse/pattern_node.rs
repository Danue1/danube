use crate::{Context, Error, Parse};
use danube_ast::{PathNode, PatternKind, PatternNode, DUMMY_NODE_ID};
use danube_token::{keywords, TokenKind};

impl Parse for PatternNode {
    type Output = PatternNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        match context.cursor.peek().kind {
            TokenKind::DotDot => {
                context.cursor.next();

                Ok(PatternNode {
                    id: DUMMY_NODE_ID,
                    kind: PatternKind::Rest,
                })
            }
            TokenKind::Identifier(keywords::Placeholder) => {
                context.cursor.next();

                Ok(PatternNode {
                    id: DUMMY_NODE_ID,
                    kind: PatternKind::Wildcard,
                })
            }
            TokenKind::Literal(symbol, ref kind) => {
                let kind = PatternKind::Literal(symbol, kind.clone());

                context.cursor.next();

                Ok(PatternNode {
                    id: DUMMY_NODE_ID,
                    kind,
                })
            }
            TokenKind::LeftBracket => {
                context.cursor.next();

                let mut patterns = Vec::new();

                while !symbol!(context.cursor => RightBracket) {
                    patterns.push(PatternNode::parse(context)?);

                    if !symbol!(context.cursor => Comma) && symbol!(context.cursor => RightBracket)
                    {
                        break;
                    }
                }

                Ok(PatternNode {
                    id: DUMMY_NODE_ID,
                    kind: PatternKind::Slice(patterns),
                })
            }
            TokenKind::LeftParens => {
                context.cursor.next();

                let mut fields = Vec::new();

                while !symbol!(context.cursor => RightParens) {
                    fields.push(PatternNode::parse(context)?);

                    if !symbol!(context.cursor => Comma) && symbol!(context.cursor => RightParens) {
                        break;
                    }
                }

                Ok(PatternNode {
                    id: DUMMY_NODE_ID,
                    kind: PatternKind::UnnamedStruct(None, fields),
                })
            }
            _ => {
                let path = if let Some(path) = PathNode::parse(context)? {
                    path
                } else {
                    return Err(Error::Invalid);
                };

                match context.cursor.peek().kind {
                    TokenKind::LeftBrace => {
                        context.cursor.next();

                        let mut fields = Vec::new();

                        while !symbol!(context.cursor => RightBrace) {
                            let path = if let Some(path) = PathNode::parse(context)? {
                                path
                            } else {
                                return Err(Error::Invalid);
                            };
                            let pattern = if symbol!(context.cursor => Colon) {
                                Some(PatternNode::parse(context)?)
                            } else {
                                None
                            };
                            fields.push((path, pattern));

                            if !symbol!(context.cursor => Comma)
                                && symbol!(context.cursor => RightBrace)
                            {
                                break;
                            }
                        }

                        Ok(PatternNode {
                            id: DUMMY_NODE_ID,
                            kind: PatternKind::NamedStruct(path, fields),
                        })
                    }
                    TokenKind::LeftParens => {
                        context.cursor.next();

                        let mut fields = Vec::new();

                        while !symbol!(context.cursor => RightParens) {
                            fields.push(PatternNode::parse(context)?);

                            if !symbol!(context.cursor => Comma)
                                && symbol!(context.cursor => RightParens)
                            {
                                break;
                            }
                        }

                        Ok(PatternNode {
                            id: DUMMY_NODE_ID,
                            kind: PatternKind::UnnamedStruct(Some(path), fields),
                        })
                    }
                    _ => Ok(PatternNode {
                        id: DUMMY_NODE_ID,
                        kind: PatternKind::Path(path),
                    }),
                }
            }
        }
    }
}

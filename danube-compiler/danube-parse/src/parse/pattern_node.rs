use crate::{Context, Parse};
use danube_ast::{
    LiteralNode, PathNode, PatternKind, PatternNamedStructFieldNode, PatternNamedStructNode,
    PatternNode, PatternUnnamedStructNode, DUMMY_NODE_ID,
};
use danube_diagnostics::MessageBuilder;
use danube_token::{keywords, TokenKind};

impl Parse for PatternNode {
    type Output = PatternNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
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
                let kind = PatternKind::Literal(LiteralNode {
                    symbol,
                    kind: kind.clone(),
                });

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
                    kind: PatternKind::UnnamedStruct(PatternUnnamedStructNode {
                        path: None,
                        fields,
                    }),
                })
            }
            _ => {
                let path = if let Some(path) = PathNode::parse(context)? {
                    path
                } else {
                    return context.report(MessageBuilder::error("Expected pattern path").build());
                };

                match context.cursor.peek().kind {
                    TokenKind::LeftBrace => {
                        context.cursor.next();

                        let mut fields = Vec::new();

                        while !symbol!(context.cursor => RightBrace) {
                            let path = if let Some(path) = PathNode::parse(context)? {
                                path
                            } else {
                                return context
                                    .report(MessageBuilder::error("Expected field path").build());
                            };
                            let pattern = if symbol!(context.cursor => Colon) {
                                Some(PatternNode::parse(context)?)
                            } else {
                                None
                            };
                            fields.push(PatternNamedStructFieldNode { path, pattern });

                            if !symbol!(context.cursor => Comma)
                                && symbol!(context.cursor => RightBrace)
                            {
                                break;
                            }
                        }

                        Ok(PatternNode {
                            id: DUMMY_NODE_ID,
                            kind: PatternKind::NamedStruct(PatternNamedStructNode { path, fields }),
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
                            kind: PatternKind::UnnamedStruct(PatternUnnamedStructNode {
                                path: Some(path),
                                fields,
                            }),
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

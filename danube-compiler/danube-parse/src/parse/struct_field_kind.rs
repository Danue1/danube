use crate::{Context, Parse};
use danube_ast::{NamedStructField, StructFieldKind, UnnamedStructField};
use danube_diagnostics::MessageBuilder;
use danube_token::TokenKind;

impl Parse for StructFieldKind {
    type Output = Option<StructFieldKind>;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        match context.cursor.peek().kind {
            TokenKind::LeftParens => {
                context.cursor.next();

                let mut fields = vec![];

                while !symbol!(context.cursor => RightParens) {
                    fields.push(UnnamedStructField::parse(context)?);

                    if !symbol!(context.cursor => Comma) && symbol!(context.cursor => RightParens) {
                        break;
                    }
                }

                Ok(Some(StructFieldKind::Unnamed(fields)))
            }
            TokenKind::LeftBrace => {
                context.cursor.next();

                let mut fields = vec![];

                while !symbol!(context.cursor => RightBrace) {
                    fields.push(NamedStructField::parse(context)?);

                    if !symbol!(context.cursor => Comma) && symbol!(context.cursor => RightBrace) {
                        break;
                    }
                }

                Ok(Some(StructFieldKind::Named(fields)))
            }
            TokenKind::Semicolon => {
                context.cursor.next();

                Ok(None)
            }
            _ => context.report(MessageBuilder::error("Expected `(` or `{{`").build()),
        }
    }
}

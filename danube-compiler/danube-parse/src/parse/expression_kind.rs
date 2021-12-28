use crate::{Error, Parse};
use danube_ast::ExpressionKind;

impl<'parse> Parse<'parse> {
    pub fn parse_expression_kind(&mut self) -> Result<ExpressionKind, Error> {
        std::todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;
    use danube_ast::{Id, StatementId, StatementKind, StatementNode};
    use danube_lex::Lex;
    use danube_token::Token;

    fn sub() {
        //
    }
}

use super::{lower_definition, lower_expression, lower_type};
use danubec_diagnostic::Diagnostic;
use danubec_middle::{ast, lst};

pub fn lower_statement(statement: lst::Statement) -> Result<ast::Statement, Diagnostic> {
    let kind = opt!(statement.kind(), "ICE: StatementKind not found");
    let kind = lower_statement_kind(kind)?;

    Ok(ast::Statement { kind })
}

pub fn lower_statement_kind(
    statement_kind: lst::StatementKind,
) -> Result<ast::StatementKind, Diagnostic> {
    match statement_kind {
        lst::StatementKind::Definition(definition) => {
            let definition = opt!(definition.definition(), "ICE: Definition not found");
            let definition = lower_definition(definition)?;

            Ok(ast::StatementKind::Definition(definition))
        }
        lst::StatementKind::Expression(expression) => {
            let expression = opt!(expression.expression(), "ICE: Expression not found");
            let expression = lower_expression(expression)?;

            Ok(ast::StatementKind::Expression(expression))
        }
        lst::StatementKind::Let(let_statement) => {
            let pattern = opt!(let_statement.pattern(), "ICE: Pattern not found");
            let pattern = super::lower_pattern(pattern)?;

            let ty = if let Some(ty) = let_statement.ty() {
                Some(lower_type(ty)?)
            } else {
                None
            };

            let expression = if let Some(rhs) = let_statement.expression() {
                Some(lower_expression(rhs)?)
            } else {
                None
            };

            Ok(ast::StatementKind::Let {
                pattern,
                ty,
                expression,
            })
        }
        lst::StatementKind::Semicolon(_) => Ok(ast::StatementKind::Semicolon),
    }
}

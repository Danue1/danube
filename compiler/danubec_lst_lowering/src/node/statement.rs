use super::{lower_definition, lower_expression, lower_type};
use danubec_diagnostic::Diagnostic;
use danubec_middle::{ast, lst};
use std::collections::HashMap;

pub fn lower_statement(
    statement: lst::Statement,
    modules: &HashMap<String, lst::Module>,
) -> Result<ast::Statement, Diagnostic> {
    let kind = opt!(statement.kind(), "ICE: StatementKind not found");
    let kind = lower_statement_kind(kind, modules)?;

    Ok(ast::Statement { kind })
}

pub fn lower_statement_kind(
    statement_kind: lst::StatementKind,
    modules: &HashMap<String, lst::Module>,
) -> Result<ast::StatementKind, Diagnostic> {
    match statement_kind {
        lst::StatementKind::Definition(definition) => {
            let definition = opt!(definition.definition(), "ICE: Definition not found");
            let definition = lower_definition(definition, modules)?;

            Ok(ast::StatementKind::Definition(definition))
        }
        lst::StatementKind::Expression(statement) => {
            let expression = opt!(statement.expression(), "ICE: Expression not found");
            let expression = lower_expression(expression, modules)?;

            let semicolon = statement.semicolon().is_some();

            Ok(ast::StatementKind::Expression {
                expression,
                semicolon,
            })
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
                Some(lower_expression(rhs, modules)?)
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

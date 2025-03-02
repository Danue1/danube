use super::{
    lower_identifier, lower_literal, lower_path, lower_path_segment, lower_pattern, lower_statement,
};
use danubec_diagnostic::Diagnostic;
use danubec_middle::{ast, lst};
use danubec_syntax::AstNode;
use std::collections::HashMap;

pub fn lower_expression(
    expression: lst::Expression,
    modules: &HashMap<String, lst::Module>,
) -> Result<ast::Expression, Diagnostic> {
    let kind = opt!(expression.kind(), "ICE: ExpressionKind is missing");
    let kind = lower_expression_kind(kind, modules)?;

    Ok(ast::Expression { kind })
}

pub fn lower_expression_kind(
    expression_kind: lst::ExpressionKind,
    modules: &HashMap<String, lst::Module>,
) -> Result<ast::ExpressionKind, Diagnostic> {
    match expression_kind {
        lst::ExpressionKind::Break(_) => Ok(ast::ExpressionKind::Break),
        lst::ExpressionKind::Continue(_) => Ok(ast::ExpressionKind::Continue),
        lst::ExpressionKind::For(for_expression) => {
            let binding = opt!(for_expression.pattern(), "ICE: pattern is missing");
            let binding = lower_pattern(binding)?;

            let expression = opt!(for_expression.iterator(), "ICE: iterator is missing");
            let expression = opt!(expression.expression(), "ICE: expression is missing");
            let expression = lower_expression(expression, modules)?;

            let block = opt!(for_expression.block(), "ICE: block is missing");
            let block = lower_block_expression(block, modules)?;

            Ok(ast::ExpressionKind::For {
                binding,
                iterator: Box::new(expression),
                block,
            })
        }
        lst::ExpressionKind::If(if_expression) => {
            let condition = opt!(if_expression.condition(), "ICE: condition is missing");
            let condition = lower_expression(condition, modules)?;

            let then_block = opt!(if_expression.then_branch(), "ICE: then branch is missing");
            let then_block = lower_block_expression(then_block, modules)?;

            let else_block = if let Some(else_branch) = if_expression.else_branch() {
                if let Some(block) = else_branch.block() {
                    let block = lower_block_expression(block, modules)?;
                    let block = ast::Expression {
                        kind: ast::ExpressionKind::Block(block),
                    };
                    Some(Box::new(block))
                } else {
                    let expression = opt!(else_branch.expression(), "ICE: expression is missing");
                    Some(Box::new(lower_expression(expression, modules)?))
                }
            } else {
                None
            };

            Ok(ast::ExpressionKind::If {
                condition: Box::new(condition),
                then_block,
                else_block,
            })
        }
        lst::ExpressionKind::Let(let_expression) => {
            let (binding, expression) = lower_let_expression(let_expression, modules)?;

            Ok(ast::ExpressionKind::Let {
                binding,
                expression,
            })
        }
        lst::ExpressionKind::Loop(loop_expression) => {
            let block = opt!(loop_expression.block(), "ICE: block is missing");
            let block = lower_block_expression(block, modules)?;

            Ok(ast::ExpressionKind::Loop(block))
        }
        lst::ExpressionKind::Match(match_expression) => {
            let condition = opt!(match_expression.expression(), "ICE: expression is missing");
            let condition = lower_expression(condition, modules)?;

            let mut arms = vec![];
            for arm in match_expression.arms() {
                let binding = opt!(arm.pattern(), "ICE: pattern is missing");
                let binding = lower_pattern(binding)?;

                let expression = opt!(arm.expression(), "ICE: block is missing");
                let expression = lower_expression(expression, modules)?;

                arms.push((binding, expression));
            }

            Ok(ast::ExpressionKind::Match {
                condition: Box::new(condition),
                arms,
            })
        }
        lst::ExpressionKind::Return(return_expression) => {
            let expression = if let Some(expression) = return_expression.expression() {
                Some(Box::new(lower_expression(expression, modules)?))
            } else {
                None
            };

            Ok(ast::ExpressionKind::Return(expression))
        }
        lst::ExpressionKind::While(while_expression) => {
            let condition = opt!(while_expression.expression(), "ICE: expression is missing");
            let condition = lower_expression(condition, modules)?;

            let block = opt!(while_expression.block(), "ICE: block is missing");
            let block = lower_block_expression(block, modules)?;

            Ok(ast::ExpressionKind::While {
                condition: Box::new(condition),
                block,
            })
        }

        lst::ExpressionKind::Array(array) => {
            let mut elements = vec![];
            for element in array.elements() {
                let expression = opt!(element.expression(), "ICE: element is missing");
                elements.push(lower_expression(expression, modules)?);
            }

            Ok(ast::ExpressionKind::Array(elements))
        }
        lst::ExpressionKind::Block(block) => {
            let block = lower_block_expression(block, modules)?;

            Ok(ast::ExpressionKind::Block(block))
        }
        lst::ExpressionKind::Literal(literal) => {
            let literal = lower_literal_expression(literal)?;

            Ok(ast::ExpressionKind::Literal(literal))
        }
        lst::ExpressionKind::Path(path) => {
            let path = opt!(path.path(), "ICE: path is missing");
            let path = lower_path(path)?;

            Ok(ast::ExpressionKind::Path(path))
        }
        lst::ExpressionKind::Unary(unary) => {
            let (operator, operand) = lower_unary_expression(unary, modules)?;

            Ok(ast::ExpressionKind::Unary { operator, operand })
        }

        lst::ExpressionKind::Assignment(assignment) => {
            let (lhs, operator, rhs) = lower_assignment_expression(assignment, modules)?;

            Ok(ast::ExpressionKind::Assignment { lhs, operator, rhs })
        }
        lst::ExpressionKind::Binary(binary) => {
            let (lhs, operator, rhs) = lower_binary_expression(binary, modules)?;

            Ok(ast::ExpressionKind::Binary { lhs, operator, rhs })
        }
        lst::ExpressionKind::Await(await_expression) => {
            let expression = opt!(await_expression.expression(), "ICE: expression is missing");
            let expression = lower_expression(expression, modules)?;

            Ok(ast::ExpressionKind::Await(Box::new(expression)))
        }
        lst::ExpressionKind::FunctionCall(function_call) => {
            let callee = opt!(function_call.expression(), "ICE: expression is missing");
            let callee = lower_expression(callee, modules)?;

            let mut arguments = vec![];
            for argument in function_call.arguments() {
                let expression = opt!(argument.expression(), "ICE: expression is missing");
                arguments.push(lower_expression(expression, modules)?);
            }

            Ok(ast::ExpressionKind::FunctionCall {
                callee: Box::new(callee),
                arguments,
            })
        }
        lst::ExpressionKind::MethodCall(method_call) => {
            let receiver = opt!(method_call.expression(), "ICE: expression is missing");
            let receiver = lower_expression(receiver, modules)?;

            let path = opt!(method_call.path_segment(), "ICE: path segment is missing");
            let path = lower_path_segment(path)?;

            let mut arguments = vec![];
            for argument in method_call.arguments() {
                let expression = opt!(argument.expression(), "ICE: expression is missing");
                arguments.push(lower_expression(expression, modules)?);
            }

            Ok(ast::ExpressionKind::MethodCall {
                receiver: Box::new(receiver),
                path,
                arguments,
            })
        }
        lst::ExpressionKind::Field(field_expression) => {
            let receiver = opt!(field_expression.expression(), "ICE: expression is missing");
            let receiver = lower_expression(receiver, modules)?;

            let field = opt!(field_expression.identifier(), "ICE: identifier is missing");
            let field = lower_identifier(field)?;

            Ok(ast::ExpressionKind::Field {
                receiver: Box::new(receiver),
                field,
            })
        }
        lst::ExpressionKind::Index(index_expression) => {
            let receiver = opt!(index_expression.expression(), "ICE: expression is missing");
            let receiver = lower_expression(receiver, modules)?;

            let index = opt!(index_expression.expression(), "ICE: expression is missing");
            let index = lower_expression(index, modules)?;

            Ok(ast::ExpressionKind::Index {
                receiver: Box::new(receiver),
                index: Box::new(index),
            })
        }
        lst::ExpressionKind::Range(range_expression) => {
            let start = if let Some(start) = range_expression.start() {
                Some(Box::new(lower_expression(start, modules)?))
            } else {
                None
            };

            let end = opt!(range_expression.end(), "ICE: end is missing");
            let end = opt!(end.expression(), "ICE: expression is missing");
            let end = lower_expression(end, modules)?;

            let range_operator = opt!(
                range_expression.range_operator(),
                "ICE: range operator is missing"
            );
            let inclusive = range_operator.syntax().to_string().as_str() == "..";

            Ok(ast::ExpressionKind::Range {
                start,
                end: Box::new(end),
                inclusive,
            })
        }
        lst::ExpressionKind::Struct(struct_expression) => {
            let path = opt!(struct_expression.path(), "ICE: path is missing");
            let path = lower_path(path)?;

            let mut fields = vec![];
            for field in struct_expression.fields() {
                let identifier = opt!(field.identifier(), "ICE: identifier is missing");
                let identifier = lower_identifier(identifier)?;

                let expression = opt!(field.expression(), "ICE: expression is missing");
                let expression = lower_expression(expression, modules)?;

                fields.push((identifier, expression));
            }

            Ok(ast::ExpressionKind::Struct { path, fields })
        }
        lst::ExpressionKind::Try(try_expression) => {
            let expression = opt!(try_expression.expression(), "ICE: expression is missing");
            let expression = lower_expression(expression, modules)?;

            Ok(ast::ExpressionKind::Try(Box::new(expression)))
        }
        lst::ExpressionKind::Yield(yield_expression) => {
            let expression = opt!(yield_expression.expression(), "ICE: expression is missing");
            let expression = lower_expression(expression, modules)?;

            Ok(ast::ExpressionKind::Yield(Box::new(expression)))
        }
    }
}

pub fn lower_assignment_expression(
    assignment: lst::AssignmentExpression,
    modules: &HashMap<String, lst::Module>,
) -> Result<
    (
        Box<ast::Expression>,
        ast::AssignmentOperator,
        Box<ast::Expression>,
    ),
    Diagnostic,
> {
    let lhs = opt!(assignment.lhs(), "ICE: lhs is missing");
    let lhs = lower_expression(lhs, modules)?;

    let operator = opt!(assignment.operator(), "ICE: operator is missing");
    let operator = lower_assignment_operator(operator)?;

    let rhs = opt!(assignment.rhs(), "ICE: rhs is missing");
    let rhs = lower_expression(rhs, modules)?;

    Ok((Box::new(lhs), operator, Box::new(rhs)))
}

pub fn lower_binary_expression(
    binary: lst::BinaryExpression,
    modules: &HashMap<String, lst::Module>,
) -> Result<
    (
        Box<ast::Expression>,
        ast::BinaryOperator,
        Box<ast::Expression>,
    ),
    Diagnostic,
> {
    let lhs = opt!(binary.lhs(), "ICE: lhs is missing");
    let lhs = lower_expression(lhs, modules)?;

    let operator = opt!(binary.operator(), "ICE: operator is missing");
    let operator = lower_binary_operator(operator)?;

    let rhs = opt!(binary.rhs(), "ICE: rhs is missing");
    let rhs = opt!(rhs.expression(), "ICE: expression is missing");
    let rhs = lower_expression(rhs, modules)?;

    Ok((Box::new(lhs), operator, Box::new(rhs)))
}

pub fn lower_block_expression(
    block: lst::BlockExpression,
    modules: &HashMap<String, lst::Module>,
) -> Result<Vec<ast::Statement>, Diagnostic> {
    let mut statements = Vec::new();
    for statement in block.statements() {
        statements.push(lower_statement(statement, modules)?);
    }

    Ok(statements)
}

pub fn lower_let_expression(
    let_expression: lst::LetExpression,
    modules: &HashMap<String, lst::Module>,
) -> Result<(ast::Pattern, Option<Box<ast::Expression>>), Diagnostic> {
    let binding = opt!(let_expression.pattern(), "ICE: pattern is missing");
    let binding = lower_pattern(binding)?;

    let expression = if let Some(expression) = let_expression.expression() {
        Some(Box::new(lower_expression(expression, modules)?))
    } else {
        None
    };

    Ok((binding, expression))
}

pub fn lower_literal_expression(
    literal: lst::LiteralExpression,
) -> Result<ast::Literal, Diagnostic> {
    let literal = opt!(literal.literal(), "ICE: literal is missing");
    let literal = lower_literal(literal)?;

    Ok(literal)
}

pub fn lower_unary_expression(
    unary: lst::UnaryExpression,
    modules: &HashMap<String, lst::Module>,
) -> Result<(ast::UnaryOperator, Box<ast::Expression>), Diagnostic> {
    let operator = opt!(unary.operator(), "ICE: operator is missing");
    let operator = lower_unary_operator(operator)?;

    let operand = opt!(unary.expression(), "ICE: operand is missing");
    let operand = lower_expression(operand, modules)?;

    Ok((operator, Box::new(operand)))
}

pub fn lower_assignment_operator(
    assignment_operator: lst::AssignmentOperator,
) -> Result<ast::AssignmentOperator, Diagnostic> {
    match assignment_operator.syntax().to_string().as_str() {
        "=" => Ok(ast::AssignmentOperator::Assign),

        "+=" => Ok(ast::AssignmentOperator::AddAssign),
        "+|=" => Ok(ast::AssignmentOperator::AddSaturatingAssign),
        "+%=" => Ok(ast::AssignmentOperator::AddWrappingAssign),

        "-=" => Ok(ast::AssignmentOperator::SubAssign),
        "-|=" => Ok(ast::AssignmentOperator::SubSaturatingAssign),
        "-%=" => Ok(ast::AssignmentOperator::SubWrappingAssign),

        "*=" => Ok(ast::AssignmentOperator::MulAssign),
        "*|=" => Ok(ast::AssignmentOperator::MulSaturatingAssign),
        "*%=" => Ok(ast::AssignmentOperator::MulWrappingAssign),

        "**=" => Ok(ast::AssignmentOperator::ExpAssign),
        "**|=" => Ok(ast::AssignmentOperator::ExpSaturatingAssign),
        "**%=" => Ok(ast::AssignmentOperator::ExpWrappingAssign),

        "/=" => Ok(ast::AssignmentOperator::DivAssign),
        "%=" => Ok(ast::AssignmentOperator::RemAssign),

        "^=" => Ok(ast::AssignmentOperator::BitXorAssign),
        "&=" => Ok(ast::AssignmentOperator::BitAndAssign),
        "|=" => Ok(ast::AssignmentOperator::BitOrAssign),
        "<<=" => Ok(ast::AssignmentOperator::ShlAssign),
        "<<|=" => Ok(ast::AssignmentOperator::ShlSaturatingAssign),
        ">>=" => Ok(ast::AssignmentOperator::ShrAssign),
        ">>>=" => Ok(ast::AssignmentOperator::ShrWrappingAssign),

        "&&=" => Ok(ast::AssignmentOperator::AndAssign),
        "||=" => Ok(ast::AssignmentOperator::OrAssign),
        _ => error!("ICE: unknown assignment operator"),
    }
}

pub fn lower_binary_operator(
    binary_operator: lst::BinaryOperator,
) -> Result<ast::BinaryOperator, Diagnostic> {
    match binary_operator.syntax().to_string().as_str() {
        "||" => Ok(ast::BinaryOperator::Or),

        "&&" => Ok(ast::BinaryOperator::And),

        "==" => Ok(ast::BinaryOperator::Equal),
        "!=" => Ok(ast::BinaryOperator::NotEqual),
        "<=" => Ok(ast::BinaryOperator::LessOrEqual),
        "<" => Ok(ast::BinaryOperator::Less),
        ">=" => Ok(ast::BinaryOperator::GreaterOrEqual),
        ">" => Ok(ast::BinaryOperator::Greater),

        "|" => Ok(ast::BinaryOperator::BitwiseOr),

        "^" => Ok(ast::BinaryOperator::BitwiseXor),

        "&" => Ok(ast::BinaryOperator::BitwiseAnd),

        "<<" => Ok(ast::BinaryOperator::ShiftLeft),
        "<<|" => Ok(ast::BinaryOperator::UnsignedShiftLeft),
        ">>" => Ok(ast::BinaryOperator::ShiftRight),
        ">>>" => Ok(ast::BinaryOperator::UnsignedShiftRight),

        "+" => Ok(ast::BinaryOperator::Add),
        "+|" => Ok(ast::BinaryOperator::AddSaturating),
        "+%" => Ok(ast::BinaryOperator::AddWrapping),
        "-" => Ok(ast::BinaryOperator::Sub),
        "-|" => Ok(ast::BinaryOperator::SubSaturating),
        "-%" => Ok(ast::BinaryOperator::SubWrapping),

        "*" => Ok(ast::BinaryOperator::Mul),
        "*|" => Ok(ast::BinaryOperator::MulSaturating),
        "*%" => Ok(ast::BinaryOperator::MulWrapping),
        "**" => Ok(ast::BinaryOperator::Exp),
        "**|" => Ok(ast::BinaryOperator::ExpSaturating),
        "**%" => Ok(ast::BinaryOperator::ExpWrapping),
        "/" => Ok(ast::BinaryOperator::Div),
        "%" => Ok(ast::BinaryOperator::Rem),

        _ => error!("ICE: unknown binary operator"),
    }
}

pub fn lower_unary_operator(
    unary_operator: lst::UnaryOperator,
) -> Result<ast::UnaryOperator, Diagnostic> {
    match unary_operator.syntax().to_string().as_str() {
        "-" => Ok(ast::UnaryOperator::Negate),
        "!" => Ok(ast::UnaryOperator::Not),
        "~" => Ok(ast::UnaryOperator::BitwiseNot),
        _ => error!("ICE: unknown unary operator"),
    }
}

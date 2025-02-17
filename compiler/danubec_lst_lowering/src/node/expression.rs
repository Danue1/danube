use super::{lower_literal, lower_pattern, lower_statement};
use danubec_diagnostic::Diagnostic;
use danubec_middle::{ast, lst};
use danubec_syntax::AstNode;

pub fn lower_expression(expression: lst::Expression) -> Result<ast::Expression, Diagnostic> {
    let kind = opt!(expression.kind(), "ICE: ExpressionKind is missing");
    let kind = lower_expression_kind(kind)?;

    Ok(ast::Expression { kind })
}

pub fn lower_expression_kind(
    expression_kind: lst::ExpressionKind,
) -> Result<ast::ExpressionKind, Diagnostic> {
    match expression_kind {
        lst::ExpressionKind::Array(array) => {
            let mut elements = vec![];
            for element in array.elements() {
                let expression = opt!(element.expression(), "ICE: element is missing");
                elements.push(lower_expression(expression)?);
            }

            Ok(ast::ExpressionKind::Array(elements))
        }
        lst::ExpressionKind::Assignment(assignment) => {
            let (lhs, operator, rhs) = lower_assignment_expression(assignment)?;

            Ok(ast::ExpressionKind::Assignment { lhs, operator, rhs })
        }
        lst::ExpressionKind::Binary(binary) => {
            let (lhs, operator, rhs) = lower_binary_expression(binary)?;

            Ok(ast::ExpressionKind::Binary { lhs, operator, rhs })
        }
        lst::ExpressionKind::Block(block) => {
            let block = lower_block_expression(block)?;

            Ok(ast::ExpressionKind::Block(block))
        }
        lst::ExpressionKind::Let(let_expression) => {
            let (pattern, expression) = lower_let_expression(let_expression)?;

            Ok(ast::ExpressionKind::Let {
                pattern,
                expression,
            })
        }
        lst::ExpressionKind::Literal(literal) => {
            let literal = lower_literal_expression(literal)?;

            Ok(ast::ExpressionKind::Literal(literal))
        }
        lst::ExpressionKind::Unary(unary) => {
            let (operator, operand) = lower_unary_expression(unary)?;

            Ok(ast::ExpressionKind::Unary { operator, operand })
        }
    }
}

pub fn lower_assignment_expression(
    assignment: lst::AssignmentExpression,
) -> Result<
    (
        Box<ast::Expression>,
        ast::AssignmentOperator,
        Box<ast::Expression>,
    ),
    Diagnostic,
> {
    let lhs = opt!(assignment.lhs(), "ICE: lhs is missing");
    let lhs = lower_expression(lhs)?;

    let operator = opt!(assignment.operator(), "ICE: operator is missing");
    let operator = lower_assignment_operator(operator)?;

    let rhs = opt!(assignment.rhs(), "ICE: rhs is missing");
    let rhs = lower_expression(rhs)?;

    Ok((Box::new(lhs), operator, Box::new(rhs)))
}

pub fn lower_binary_expression(
    binary: lst::BinaryExpression,
) -> Result<
    (
        Box<ast::Expression>,
        ast::BinaryOperator,
        Box<ast::Expression>,
    ),
    Diagnostic,
> {
    let lhs = opt!(binary.lhs(), "ICE: lhs is missing");
    let lhs = lower_expression(lhs)?;

    let operator = opt!(binary.operator(), "ICE: operator is missing");
    let operator = lower_binary_operator(operator)?;

    let rhs = opt!(binary.rhs(), "ICE: rhs is missing");
    let rhs = lower_expression(rhs)?;

    Ok((Box::new(lhs), operator, Box::new(rhs)))
}

pub fn lower_block_expression(
    block: lst::BlockExpression,
) -> Result<Vec<ast::Statement>, Diagnostic> {
    let mut statements = Vec::new();
    for statement in block.statements() {
        statements.push(lower_statement(statement)?);
    }

    Ok(statements)
}

pub fn lower_let_expression(
    let_expression: lst::LetExpression,
) -> Result<(ast::Pattern, Option<Box<ast::Expression>>), Diagnostic> {
    let pattern = opt!(let_expression.pattern(), "ICE: pattern is missing");
    let pattern = lower_pattern(pattern)?;

    let expression = if let Some(expression) = let_expression.expression() {
        Some(Box::new(lower_expression(expression)?))
    } else {
        None
    };

    Ok((pattern, expression))
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
) -> Result<(ast::UnaryOperator, Box<ast::Expression>), Diagnostic> {
    let operator = opt!(unary.operator(), "ICE: operator is missing");
    let operator = lower_unary_operator(operator)?;

    let operand = opt!(unary.expression(), "ICE: operand is missing");
    let operand = lower_expression(operand)?;

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

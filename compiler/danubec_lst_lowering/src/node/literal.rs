use danubec_diagnostic::Diagnostic;
use danubec_middle::{ast, lst};

pub fn lower_literal(literal: lst::Literal) -> Result<ast::Literal, Diagnostic> {
    let kind = match literal {
        lst::Literal::Boolean(boolean_literal) => {
            let literal = lower_boolean_literal(boolean_literal)?;

            ast::LiteralKind::Boolean(literal)
        }
        lst::Literal::Char(char_literal) => {
            let char_literal = lower_char_literal(char_literal)?;

            ast::LiteralKind::Char(char_literal)
        }
        lst::Literal::Numeric(numeric_literal) => match lower_numeric_literal(numeric_literal)? {
            NumberLiteral::Integer(integer) => ast::LiteralKind::Integer(integer),
            NumberLiteral::Float(float) => ast::LiteralKind::Float(float),
        },
        lst::Literal::String(string_literal) => {
            let string_literal = lower_string_literal(string_literal)?;

            ast::LiteralKind::String(string_literal)
        }
    };

    Ok(ast::Literal { kind })
}

pub fn lower_boolean_literal(boolean_literal: lst::BooleanLiteral) -> Result<bool, Diagnostic> {
    if boolean_literal.true_token().is_some() {
        Ok(true)
    } else if boolean_literal.false_token().is_some() {
        Ok(false)
    } else {
        error!("ICE: Invalid boolean literal")
    }
}

pub fn lower_char_literal(char_literal: lst::CharLiteral) -> Result<char, Diagnostic> {
    let raw = opt!(char_literal.raw(), "ICE: Invalid char literal");
    let raw = raw.to_string();

    if raw.len() == 1 {
        Ok(raw.chars().next().unwrap())
    } else {
        error!("ICE: Invalid char literal")
    }
}

pub enum NumberLiteral {
    Integer(i64),
    Float(f64),
}

pub fn lower_numeric_literal(
    numeric_literal: lst::NumericLiteral,
) -> Result<NumberLiteral, Diagnostic> {
    let kind = opt!(numeric_literal.kind(), "ICE: Invalid numeric literal");
    let kind = lower_numeric_literal_kind(kind)?;

    Ok(kind)
}

#[allow(unused)]
pub fn lower_string_literal(string_literal: lst::StringLiteral) -> Result<String, Diagnostic> {
    let mut string = String::new();
    for fragment in string_literal.fragments() {
        match fragment {
            lst::StringLiteralFragment::Raw(raw) => string.push_str(&raw.to_string()),
            lst::StringLiteralFragment::Escape(escape) => std::todo!(),
            lst::StringLiteralFragment::Interpolation(interpolation) => std::todo!(),
        }
    }

    Ok(string)
}

pub fn lower_numeric_literal_kind(
    numeric_literal_kind: lst::NumericLiteralKind,
) -> Result<NumberLiteral, Diagnostic> {
    match numeric_literal_kind {
        lst::NumericLiteralKind::Decimal(decimal_numeric_literal) => {
            let literal = decimal_numeric_literal.to_string();

            match lower_float(&literal) {
                Ok(number) => Ok(NumberLiteral::Float(number)),
                Err(_) => Ok(NumberLiteral::Integer(lower_integer(&literal)?)),
            }
        }
        lst::NumericLiteralKind::Binary(binary_numeric_literal) => {
            let literal = binary_numeric_literal.to_string();
            let literal = lower_integer(&literal)?;

            Ok(NumberLiteral::Integer(literal))
        }
        lst::NumericLiteralKind::Octal(octal_numeric_literal) => {
            let literal = octal_numeric_literal.to_string();
            let literal = lower_integer(&literal)?;

            Ok(NumberLiteral::Integer(literal))
        }
        lst::NumericLiteralKind::Hex(hex_numeric_literal) => {
            let literal = hex_numeric_literal.to_string();
            let literal = lower_integer(&literal)?;

            Ok(NumberLiteral::Integer(literal))
        }
    }
}

pub fn lower_float(literal: &str) -> Result<f64, Diagnostic> {
    match literal.parse() {
        Ok(float) => Ok(float),
        Err(_) => error!("ICE: Invalid float literal"),
    }
}

pub fn lower_integer(literal: &str) -> Result<i64, Diagnostic> {
    match literal.parse() {
        Ok(integer) => Ok(integer),
        Err(_) => error!("ICE: Invalid integer literal"),
    }
}

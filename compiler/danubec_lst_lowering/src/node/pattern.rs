use super::{lower_literal, lower_path};
use danubec_diagnostic::Diagnostic;
use danubec_middle::{ast, lst};

pub fn lower_pattern(pattern: lst::Pattern) -> Result<ast::Pattern, Diagnostic> {
    let kind = opt!(pattern.kind(), "ICE: PatternKind not found");
    let kind = lower_pattern_kind(kind)?;

    Ok(ast::Pattern { kind })
}

pub fn lower_pattern_kind(pattern_kind: lst::PatternKind) -> Result<ast::PatternKind, Diagnostic> {
    match pattern_kind {
        lst::PatternKind::Never(_) => Ok(ast::PatternKind::Never),
        lst::PatternKind::Placeholder(_) => Ok(ast::PatternKind::Placeholder),
        lst::PatternKind::Rest(_) => Ok(ast::PatternKind::Rest),
        lst::PatternKind::Path(path) => {
            let path = opt!(path.path(), "ICE: Path not found");
            let path = lower_path(path)?;

            Ok(ast::PatternKind::Path(path))
        }
        lst::PatternKind::Tuple(tuple) => {
            let mut elements = vec![];
            for element in tuple.elements() {
                let pattern = opt!(element.pattern(), "ICE: Pattern not found");
                let pattern = lower_pattern(pattern)?;

                elements.push(pattern);
            }

            Ok(ast::PatternKind::Tuple(elements))
        }
        lst::PatternKind::Array(array) => {
            let mut elements = vec![];
            for element in array.elements() {
                let pattern = opt!(element.pattern(), "ICE: Pattern not found");
                let pattern = lower_pattern(pattern)?;

                elements.push(pattern);
            }

            Ok(ast::PatternKind::Array(elements))
        }
        lst::PatternKind::Literal(literal) => {
            let literal = opt!(literal.literal(), "ICE: Literal not found");
            let literal = lower_literal(literal)?;

            Ok(ast::PatternKind::Literal(literal))
        }
        lst::PatternKind::Or(or) => {
            let lhs = opt!(or.lhs(), "ICE: LHS not found");
            let lhs = lower_pattern(lhs)?;

            let rhs = opt!(or.rhs(), "ICE: RHS not found");
            let rhs = opt!(rhs.pattern(), "ICE: Pattern not found");
            let rhs = lower_pattern(rhs)?;

            Ok(ast::PatternKind::Or(Box::new(lhs), Box::new(rhs)))
        }
        lst::PatternKind::Named(named) => {
            let path = opt!(named.path(), "ICE: Path not found");
            let path = lower_path(path)?;

            let mut elements = vec![];
            for element in named.elements() {
                let path = opt!(element.path(), "ICE: Path not found");
                let path = lower_path(path)?;

                let pattern = if let Some(pattern) = element.pattern() {
                    Some(lower_pattern(pattern)?)
                } else {
                    None
                };

                elements.push((path, pattern));
            }

            Ok(ast::PatternKind::Named(path, elements))
        }
        lst::PatternKind::Unnamed(unnamed) => {
            let path = opt!(unnamed.path(), "ICE: Path not found");
            let path = lower_path(path)?;

            let mut elements = vec![];
            for element in unnamed.elements() {
                let pattern = opt!(element.pattern(), "ICE: Pattern not found");
                let pattern = lower_pattern(pattern)?;

                elements.push(pattern);
            }

            Ok(ast::PatternKind::Unnamed(path, elements))
        }
    }
}

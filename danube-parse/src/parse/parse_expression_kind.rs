use super::*;

pub(super) fn parse_expression_kind(t: Tokens) -> ParseResult<ExpressionKind> {
    let (t, node) = parse_prefixable_expression_node(t)?;

    parse_postfix_expression_node(t, Precedence::Lowest, node)
}

fn parse_prefixable_expression_node(t: Tokens) -> ParseResult<ExpressionKind> {
    alt((parse_prefix_expression_node, parse_atomic_expression_node))(t)
}

fn parse_atomic_expression_node(t: Tokens) -> ParseResult<ExpressionKind> {
    alt((
        map(parse_expression_struct_node, ExpressionKind::Struct),
        map(parse_expression_tuple_node, ExpressionKind::Tuple),
        map(parse_path_node, ExpressionKind::Path),
        map(parse_conditional_node, ExpressionKind::Conditional),
        map(parse_loop_node, ExpressionKind::Loop),
        map(parse_while_node, ExpressionKind::While),
        map(parse_for_node, ExpressionKind::For),
        map(parse_pattern_match_node, ExpressionKind::PatternMatch),
        map(parse_closure_node, ExpressionKind::Closure),
        map(parse_literal_kind, ExpressionKind::Literal),
        map(parse_break, |_| ExpressionKind::Break),
        map(parse_continue, |_| ExpressionKind::Continue),
        map(parse_return_node, ExpressionKind::Return),
        map(parse_array, ExpressionKind::Array),
        map(parse_block_node, ExpressionKind::Block),
    ))(t)
}

fn parse_expression_struct_node(t: Tokens) -> ParseResult<ExpressionKindStructNode> {
    map(parse_expression_field_list, |(field_list, rest)| {
        ExpressionKindStructNode {
            path: None,
            field_list,
            rest,
        }
    })(t)
}

fn parse_expression_tuple_node(t: Tokens) -> ParseResult<TupleNode> {
    map(parse_tuple_operator, |argument_list| TupleNode {
        field: None,
        argument_list,
    })(t)
}

fn parse_prefix_expression_node(t: Tokens) -> ParseResult<ExpressionKind> {
    map(
        tuple((parse_unary_operator_kind, parse_expression_kind)),
        |(kind, value)| {
            ExpressionKind::UnaryOperator(UnaryOperatorNode {
                kind,
                value: Box::new(value),
            })
        },
    )(t)
}

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Sum,
    Mul,
    Comparison,
    LazyBoolean,
    ChainArrow,
}

fn parse_postfix_expression_node(
    t: Tokens,
    precedence: Precedence,
    lhs: ExpressionKind,
) -> ParseResult<ExpressionKind> {
    match parse_operator_kind(t.clone()) {
        Ok((t, OperatorKind::Await)) => {
            let node = ExpressionKind::Await(Box::new(lhs));

            parse_postfix_expression_node(t, precedence, node)
        }
        Ok((t, OperatorKind::Try)) => {
            let node = ExpressionKind::Try(Box::new(lhs));

            parse_postfix_expression_node(t, precedence, node)
        }
        Ok((t, OperatorKind::Tuple(argument_list))) => {
            let node = ExpressionKind::Tuple(TupleNode {
                field: Some(Box::new(lhs)),
                argument_list,
            });

            parse_postfix_expression_node(t, precedence, node)
        }
        Ok((t, OperatorKind::Index(rhs))) => {
            let node = ExpressionKind::Index(IndexNode {
                array: Box::new(lhs),
                index: Box::new(rhs),
            });

            parse_postfix_expression_node(t, precedence, node)
        }
        Ok((t, OperatorKind::Generic(generic_list))) => {
            let node = ExpressionKind::Generic(ExpressionGenericNode {
                expression: Box::new(lhs),
                generic_list,
            });

            parse_postfix_expression_node(t, precedence, node)
        }
        Ok((t, OperatorKind::Field(rhs))) => {
            let node = ExpressionKind::Field(ExpressionKindFieldNode {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });

            parse_postfix_expression_node(t, precedence, node)
        }
        Ok((_, OperatorKind::InfixOperator(_))) => parse_infix_expression_node(t, precedence, lhs),
        _ => {
            if let ExpressionKind::Path(path) = lhs.clone() {
                if let Ok((s, (field_list, rest))) = parse_expression_field_list(t.clone()) {
                    let node = ExpressionKind::Struct(ExpressionKindStructNode {
                        path: Some(path),
                        field_list,
                        rest,
                    });

                    parse_postfix_expression_node(s, precedence, node)
                } else {
                    Ok((t, lhs))
                }
            } else {
                Ok((t, lhs))
            }
        }
    }
}

fn parse_infix_expression_node(
    t: Tokens,
    precedence: Precedence,
    lhs: ExpressionKind,
) -> ParseResult<ExpressionKind> {
    if let Ok((tt, kind)) = parse_infix_operator_kind(t.clone()) {
        let right_precedence = infix_binding_power(kind.clone());
        if right_precedence < precedence {
            Ok((t, lhs))
        } else {
            let (t, rhs) = parse_prefixable_expression_node(tt)?;
            let (t, rhs) = parse_postfix_expression_node(t, right_precedence, rhs)?;
            let rhs = ExpressionKind::InfixOperator(InfixOperatorNode {
                kind,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });

            parse_infix_expression_node(t, precedence, rhs)
        }
    } else {
        Ok((t, lhs))
    }
}

fn infix_binding_power(kind: InfixOperatorKind) -> Precedence {
    match kind {
        InfixOperatorKind::Add | InfixOperatorKind::Sub => Precedence::Sum,
        InfixOperatorKind::Mul
        | InfixOperatorKind::Div
        | InfixOperatorKind::Mod
        | InfixOperatorKind::BitAnd
        | InfixOperatorKind::BitOr
        | InfixOperatorKind::BitXor
        | InfixOperatorKind::BitLeft
        | InfixOperatorKind::BitRight => Precedence::Mul,
        InfixOperatorKind::Equal
        | InfixOperatorKind::NotEqual
        | InfixOperatorKind::LessThan
        | InfixOperatorKind::LessThanOrEqual
        | InfixOperatorKind::GreaterThan
        | InfixOperatorKind::GreaterThanOrEqual => Precedence::Comparison,
        InfixOperatorKind::And | InfixOperatorKind::Or => Precedence::LazyBoolean,
        InfixOperatorKind::ChainArrow => Precedence::ChainArrow,
    }
}

enum OperatorKind {
    Await,
    Try,
    Tuple(Vec<TupleArgumentNode>),
    Index(ExpressionKind),
    Generic(Vec<ExpressionGenericKind>),
    Field(IdentNode),
    InfixOperator(InfixOperatorKind),
}

fn parse_operator_kind(t: Tokens) -> ParseResult<OperatorKind> {
    alt((
        map(parse_await_operator, |_| OperatorKind::Await),
        map(parse_try_operator, |_| OperatorKind::Try),
        map(parse_tuple_operator, OperatorKind::Tuple),
        map(parse_index_operator, OperatorKind::Index),
        map(parse_generic_kind, OperatorKind::Generic),
        map(parse_field_operator, OperatorKind::Field),
        map(parse_infix_operator_kind, OperatorKind::InfixOperator),
    ))(t)
}

fn parse_tuple_operator(t: Tokens) -> ParseResult<Vec<TupleArgumentNode>> {
    map(
        tuple((
            parse_symbol(Symbol::LeftParens),
            separated_list(
                parse_symbol(Symbol::Comma),
                map(
                    tuple((
                        opt(map(
                            tuple((parse_ident_node, parse_symbol(Symbol::Assign))),
                            |(name, _)| name,
                        )),
                        parse_expression_kind,
                    )),
                    |(name, value)| TupleArgumentNode { name, value },
                ),
            ),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::RightParens),
        )),
        |(_, expression_list, _, _)| expression_list,
    )(t)
}

fn parse_index_operator(t: Tokens) -> ParseResult<ExpressionKind> {
    map(
        tuple((
            parse_symbol(Symbol::LeftBracket),
            parse_expression_kind,
            parse_symbol(Symbol::RightBracket),
        )),
        |(_, expression, _)| expression,
    )(t)
}

fn parse_generic_kind(t: Tokens) -> ParseResult<Vec<ExpressionGenericKind>> {
    map(
        tuple((
            parse_symbol(Symbol::LessThan),
            separated_nonempty_list(
                parse_symbol(Symbol::Comma),
                alt((
                    map(
                        tuple((
                            parse_ident_node,
                            parse_symbol(Symbol::Assign),
                            parse_type_kind,
                        )),
                        |(ident, _, type_kind)| ExpressionGenericKind::Output(ident, type_kind),
                    ),
                    map(parse_type_kind, ExpressionGenericKind::Input),
                )),
            ),
            parse_symbol(Symbol::GreaterThan),
        )),
        |(_, generic_list, _)| generic_list,
    )(t)
}

fn parse_await_operator(t: Tokens) -> ParseResult<()> {
    map(
        tuple((parse_symbol(Symbol::Dot), parse_keyword(Keyword::Await))),
        |_| (),
    )(t)
}

fn parse_try_operator(t: Tokens) -> ParseResult<()> {
    map(parse_symbol(Symbol::Question), |_| ())(t)
}

fn parse_field_operator(t: Tokens) -> ParseResult<IdentNode> {
    map(
        tuple((parse_symbol(Symbol::Dot), parse_ident_node)),
        |(_, ident)| ident,
    )(t)
}

fn parse_expression_field_list(
    t: Tokens,
) -> ParseResult<(Vec<ExpressionKindStructField>, Option<Box<ExpressionKind>>)> {
    map(
        tuple((
            parse_symbol(Symbol::LeftBrace),
            separated_nonempty_list(
                parse_symbol(Symbol::Comma),
                tuple((
                    parse_ident_node,
                    opt(map(
                        tuple((parse_symbol(Symbol::DoubleColon), parse_expression_kind)),
                        |(_, expression)| expression,
                    )),
                )),
            ),
            opt(parse_symbol(Symbol::Comma)),
            opt(map(
                tuple((parse_symbol(Symbol::RangeClose), parse_expression_kind)),
                |(_, expression)| Box::new(expression),
            )),
            parse_symbol(Symbol::RightBrace),
        )),
        |(_, expression_list, _, rest, _)| (expression_list, rest),
    )(t)
}

fn parse_break(t: Tokens) -> ParseResult<()> {
    map(parse_keyword(Keyword::Break), |_| ())(t)
}

fn parse_continue(t: Tokens) -> ParseResult<()> {
    map(parse_keyword(Keyword::Continue), |_| ())(t)
}

fn parse_array(t: Tokens) -> ParseResult<Vec<ExpressionKind>> {
    map(
        tuple((
            parse_symbol(Symbol::LeftBracket),
            separated_list(parse_symbol(Symbol::Comma), parse_expression_kind),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::RightBracket),
        )),
        |(_, expression_list, _, _)| expression_list,
    )(t)
}

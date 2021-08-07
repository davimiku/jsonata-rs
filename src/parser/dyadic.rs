//! Parsers for dyadic expressions
//!
//! - comparison
//! - map (in relation to path operators)

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{separated_pair, tuple},
    IResult,
};
use nom_recursive::recursive_parser;

use crate::ast::{
    dyadic::{CompareExpression, CompareOpType},
    expr::{Expression, VariableBindingExpression},
    path::MapExpression,
};

use super::{expr_parser, ident::variable_ident, trim, Span};

/// Map expression
///
/// The Map expression is part of the family of path operators and is
/// a dyadic expression with the LHS evaluated and passed as the context to the
/// RHS.
///
/// ## Example
///
/// ```
/// Account.Name
/// ```
///
/// This operator is left associative meaning that the expression a.b.c.d
/// is evaluated like ((a.b).c).d; i.e. left to right
#[recursive_parser]
pub(super) fn map_expr(s: Span) -> IResult<Span, Expression> {
    map(
        separated_pair(trim(expr_parser), tag("."), trim(expr_parser)),
        |(lhs, rhs)| {
            MapExpression {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }
            .into()
        },
    )(s)
}

/// Compare expression
///
/// The compare expression may be the variants as defined in `CompareType`.
///
/// ## Example
///
/// ```
/// bugs > features
/// ```
///
/// The CompareExpression is constructed with the LHS, RHS, and which comparison
/// operator is used between them.
#[recursive_parser]
pub(super) fn comparison_expr(s: Span) -> IResult<Span, Expression> {
    map(
        tuple((trim(expr_parser), comparison_operator, trim(expr_parser))),
        |(lhs, compare_type, rhs)| {
            CompareExpression {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                compare_type,
            }
            .into()
        },
    )(s)
}

/// Parses looking for a comparison operator
///
/// The valid operators are defined in the CompareType enum
fn comparison_operator(span: Span) -> IResult<Span, CompareOpType> {
    map(
        alt((
            tag(">="),
            tag("<="),
            tag("!="),
            tag(">"),
            tag("<"),
            tag("="),
        )),
        |s: Span| CompareOpType::from(*s.fragment()),
    )(span)
}

/// Variable binding expressions bind a value to a variable
/// and also return that value.
///
/// ```
/// $my_var := "hello, world"  // also returns "hello, world"
/// ```
///
#[recursive_parser]
pub(super) fn variable_binding_expr(s: Span) -> IResult<Span, Expression> {
    map(
        separated_pair(trim(variable_ident), tag(":="), trim(expr_parser)),
        |(s, val)| {
            VariableBindingExpression {
                var_name: s.to_string(),
                bound_expression: Box::new(val),
            }
            .into()
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use crate::{ast::literal::LiteralExpression, parser::make_span};

    use super::*;

    #[test]
    fn variable_binding_parser() {
        let input = "$myvar := true";
        let (_, actual) = variable_binding_expr(make_span(input)).unwrap();
        assert_eq!(
            actual,
            VariableBindingExpression {
                var_name: "myvar".to_string(),
                bound_expression: Box::new(LiteralExpression::from(true).into())
            }
            .into()
        )
    }
}

//! Parsers for dyadic expressions
//!
//! - comparison
//! - map (in relation to path operators)

use std::num::ParseIntError;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    error::{FromExternalError, ParseError},
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::ast::{
    dyadic::{CompareExpression, CompareType},
    expr::Expression,
    path::MapExpression,
};

use super::{expr_parser, trim};

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
pub(super) fn map_expr<'a, E>(input: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    map(
        separated_pair(trim(expr_parser), tag("."), trim(expr_parser)),
        |(lhs, rhs)| {
            MapExpression {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }
            .into()
        },
    )(input)
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
pub(super) fn comparison_expr<'a, E>(input: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
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
    )(input)
}

/// Parses looking for a comparison operator
///
/// The valid operators are defined in the CompareType enum
fn comparison_operator<'a, E>(input: &'a str) -> IResult<&'a str, CompareType, E>
where
    E: ParseError<&'a str>,
{
    map(
        alt((
            tag(">="),
            tag("<="),
            tag("!="),
            tag(">"),
            tag("<"),
            tag("="),
        )),
        |s| match s {
            ">=" => CompareType::GreaterEquals,
            "<=" => CompareType::LessEquals,
            "!=" => CompareType::NotEquals,
            ">" => CompareType::Greater,
            "<" => CompareType::Less,
            "=" => CompareType::Equals,
            _ => unreachable!(),
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use crate::ast::path::PathExpression;

    use super::*;

    #[test]
    fn map_expr_simple() {
        let input = "address.city";
        let actual = map_expr::<(&str, ErrorKind)>(input).unwrap().1;

        let expected = MapExpression {
            lhs: Box::new(
                PathExpression {
                    ident: "address".to_string(),
                }
                .into(),
            ),
            rhs: Box::new(
                PathExpression {
                    ident: "city".to_string(),
                }
                .into(),
            ),
        }
        .into();

        assert_eq!(actual, expected);
    }
}

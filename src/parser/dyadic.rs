//! Parsers for dyadic expressions
//!
//! - comparison
//! - map (in relation to path operators)

use std::num::ParseIntError;

use nom::{
    bytes::complete::tag,
    combinator::map,
    error::{FromExternalError, ParseError},
    sequence::separated_pair,
    IResult,
};

use crate::ast::{expr::Expression, path::MapExpression};

use super::{expr_parser, trim};

/// Map expression
///
/// The Map expression is part of the family of path operators and is
/// a dyadic expression with the LHS evaluated and passed as the context to the
/// RHS.
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

fn comparison<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Expression, E> {
    todo!()
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

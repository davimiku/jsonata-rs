//! Parsers for various expressions
//!
//! - variable binding expression var_name: (), bound_expression: () var_name: (), bound_expression: () var_name: (), bound_expression: ()

use std::num::ParseIntError;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    error::{FromExternalError, ParseError},
    sequence::separated_pair,
    IResult,
};

use crate::ast::{
    expr::{Expression, VariableBindingExpression},
    path::{MapExpression, PathExpression},
};

use super::{
    ident::{path_ident, variable_ident},
    literal::literal_expr,
    trim,
};

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
fn map_expr<'a, E>(input: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    map(
        separated_pair(trim(expr), tag("."), trim(expr)),
        |(lhs, rhs)| {
            MapExpression {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }
            .into()
        },
    )(input)
}

/// Path expressions represent a location in the parsed JSON
/// to query from.
///
/// ```
/// Account
/// ```
///
/// ```
/// `Hello World`
/// ```
///
/// The path expression is either a non-delimited sequence of alphanumeric
/// (such as Account), where the first character must be alphabetic. Alternatively,
/// backticks can delimit the expression (such as `Hello World`) which is often used
/// when there is a space or other character in the identifier.
fn path_expr<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Expression, E> {
    map(path_ident, |ident| {
        PathExpression {
            ident: ident.to_string(),
        }
        .into()
    })(input)
}

/// Variable binding expressions bind a value to a variable
/// and also return that value.
///
/// ```
/// $my_var := "hello, world"  // also returns "hello, world"
/// ```
///
fn variable_binding_expr<'a, E>(input: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let parser = separated_pair(variable_ident, trim(tag(":=")), expr);
    map(parser, |(s, val)| {
        VariableBindingExpression {
            var_name: s.to_string(),
            bound_expression: Box::new(val),
        }
        .into()
    })(input)
}

/// Top-level function for expression parsing
///
/// Calls each of the other parsers in order until a parser
/// yiels success, or returns a ParseError
pub(super) fn expr<'a, E>(input: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((path_expr, literal_expr, variable_binding_expr))(input)
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use crate::ast::literal::LiteralExpression;

    use super::*;

    #[test]
    fn path_expr_non_delimited() {
        let input = "name";
        let actual = path_expr::<(&str, ErrorKind)>(input).unwrap().1;

        let expected = PathExpression {
            ident: "name".to_string(),
        }
        .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn path_expr_delimited() {
        let input = "`hello world`";
        let actual = path_expr::<(&str, ErrorKind)>(input).unwrap().1;

        let expected = PathExpression {
            ident: "hello world".to_string(),
        }
        .into();
        assert_eq!(actual, expected);
    }

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

    #[test]
    fn variable_binding_parser<'a>() {
        let input = "$myvar := true";
        let res = variable_binding_expr::<(&str, ErrorKind)>(input);
        assert_eq!(
            res.unwrap().1,
            VariableBindingExpression {
                var_name: "myvar".to_string(),
                bound_expression: Box::new(LiteralExpression::from(true).into())
            }
            .into()
        )
    }

    #[test]
    fn expression_parser<'a>() {
        let input = "$myvar := null";
        let res = expr::<(&str, ErrorKind)>(input);
        assert_eq!(
            res.unwrap().1,
            VariableBindingExpression {
                var_name: "myvar".to_string(),
                bound_expression: Box::new(LiteralExpression::from(()).into())
            }
            .into()
        )
    }
}

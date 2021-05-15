//! Parsers for various expressions
//!
//! - variable binding expression var_name: (), bound_expression: () var_name: (), bound_expression: () var_name: (), bound_expression: ()

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    error::{FromExternalError, ParseError},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::ast::{
    expression::{Expression, VariableBindingExpression},
    path::PathExpression,
};

use super::{
    ident::{path_ident, variable_ident},
    literal::literal_expression,
    trim,
};

/// Path expressions represent a location in the parsed JSON
/// to query from.
///
/// ```
/// Account.Name
/// ```
///
/// The path expression above is the ident "Account", which
/// also has a "member" expression for the ident "Name". The
/// "Name" expression has no more member path expression.
fn path<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Expression, E> {
    map(separated_list1(tag("."), path_ident), |v| {
        let first_ident = match v.get(0) {
            Some(s) => *s,
            None => "", // FIXME: Can this scenario ever happen? Can separated_list1 yield a zero-element vec? (I don't think so)
        };

        let first_expr = PathExpression {
            ident: first_ident.to_string(),
            member: None,
        };
        let final_expr = v.into_iter().skip(1).fold(first_expr, |mut acc, el| {
            acc.member = Some(Box::new(PathExpression {
                ident: el.to_string(),
                member: None,
            }));
            acc
        });
        Expression::Path(final_expr)
    })(input)
}

/// Variable binding expressions bind a value to a variable
/// and also return that value.
///
/// ```
/// $my_var := "hello, world"  // also returns "hello, world"
/// ```
///
fn variable_binding<'a, E>(input: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let parser = separated_pair(variable_ident, trim(tag(":=")), parse_expression);
    map(parser, |(s, val)| {
        Expression::Variable(VariableBindingExpression {
            var_name: s.to_string(),
            bound_expression: Box::new(val),
        })
    })(input)
}

/// Top-level function for expression parsing
///
/// Calls each of the other parsers in order until a parser
/// yiels success, or returns a ParseError
pub(super) fn parse_expression<'a, E>(input: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((literal_expression, variable_binding))(input)
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use crate::ast::literal::LiteralExpression;

    use super::*;

    #[test]
    fn path_one_level() {
        let input = "name";
        let actual = path::<(&str, ErrorKind)>(input).unwrap().1;

        let expected = PathExpression {
            ident: "name".to_string(),
            member: None,
        }
        .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn path_two_levels() {
        let input = "address.city";
        let actual = path::<(&str, ErrorKind)>(input).unwrap().1;

        let expected = PathExpression {
            ident: "address".to_string(),
            member: Some(Box::new(PathExpression {
                ident: "city".to_string(),
                member: None,
            })),
        }
        .into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn path_three_levels() {
        let input = "address.location.latitude";
        let actual = path::<(&str, ErrorKind)>(input).unwrap().1;

        let expected = PathExpression {
            ident: "address".to_string(),
            member: Some(Box::new(PathExpression {
                ident: "location".to_string(),
                member: Some(Box::new(PathExpression {
                    ident: "latitude".to_string(),
                    member: None,
                })),
            })),
        }
        .into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn variable_binding_parser<'a>() {
        let input = "$myvar := true";
        let res = variable_binding::<(&str, ErrorKind)>(input);
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
        let res = parse_expression::<(&str, ErrorKind)>(input);
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

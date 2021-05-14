//! Parsers for various expressions
//!
//! - variable binding expression var_name: (), bound_expression: () var_name: (), bound_expression: () var_name: (), bound_expression: ()

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    error::{FromExternalError, ParseError},
    sequence::separated_pair,
    IResult,
};

use crate::ast::expression::{Expression, VariableBindingExpression};

use super::{ident::parse_variable_ident, literal::literal_expression, trim};

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
    let parser = separated_pair(parse_variable_ident, trim(tag(":=")), parse_expression);
    map(parser, |(s, val)| {
        Expression::Variable(VariableBindingExpression {
            var_name: s.to_string(),
            bound_expression: Box::new(val),
        })
    })(input)
}

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
    fn variable_binding_parser<'a>() {
        let input = "$myvar := true";
        let res = variable_binding::<(&str, ErrorKind)>(input);
        assert_eq!(
            res.unwrap().1,
            Expression::Variable(VariableBindingExpression {
                var_name: "myvar".to_string(),
                bound_expression: Box::new(Expression::Literal(LiteralExpression::from(true)))
            })
        )
    }

    #[test]
    fn expression_parser<'a>() {
        let input = "$myvar := false";
        let res = parse_expression::<(&str, ErrorKind)>(input);
        assert_eq!(
            res.unwrap().1,
            Expression::Variable(VariableBindingExpression {
                var_name: "myvar".to_string(),
                bound_expression: Box::new(Expression::Literal(LiteralExpression::from(false)))
            })
        )
    }
}

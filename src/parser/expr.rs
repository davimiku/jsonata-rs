//! Parsers for various expressions
//!
//! - variable binding expression

use nom::{
    bytes::complete::tag,
    combinator::map,
    error::{FromExternalError, ParseError},
    sequence::separated_pair,
    IResult,
};

use crate::ast::expr::{Expression, VariableBindingExpression};

use super::{expr_parser, ident::variable_ident, trim};

/// Variable binding expressions bind a value to a variable
/// and also return that value.
///
/// ```
/// $my_var := "hello, world"  // also returns "hello, world"
/// ```
///
pub(super) fn variable_binding_expr<'a, E>(input: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let parser = separated_pair(variable_ident, trim(tag(":=")), expr_parser);
    map(parser, |(s, val)| {
        VariableBindingExpression {
            var_name: s.to_string(),
            bound_expression: Box::new(val),
        }
        .into()
    })(input)
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use crate::ast::literal::LiteralExpression;

    use super::*;

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
}

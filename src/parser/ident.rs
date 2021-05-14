//! Parses identifiers
//!
//! Identifiers can be any of the following:
//!
//! - Built-in JSONata functions (same syntax as variables)
//! - Variables from a variable binding
//! - Field reference
//!
//! Field references can be inside of backticks

use nom::{
    bytes::complete::{tag, take_while},
    error::ParseError,
    sequence::preceded,
    IResult,
};

// Notes about the current JSONata implementation (TODO: remove)
// - variables must start with `$`
// - local variable bindings shadow built-ins
// - using an undefined variable is not an error sometimes, but other times it is
// - field reference starting with "$" doesn't work unless it has quotes, which means...
// - quoted values might be literal strings or field references
// - field references that are quoted don't handle unicode correctly

/// Parses a variable ident
///
/// Variable names always start with a `$` symbol
pub(super) fn parse_variable_ident<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    preceded(
        tag("$"),
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    )(input)
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use super::*;
    #[test]
    fn variable_ident_parser() {
        let input = "$myVar";
        let res = parse_variable_ident::<(&str, ErrorKind)>(input);
        assert_eq!(res.unwrap().1, "myVar");
    }

    #[test]
    fn variable_ident_parser_underscore() {
        let input = "$my_var";
        let res = parse_variable_ident::<(&str, ErrorKind)>(input);
        assert_eq!(res.unwrap().1, "my_var");
    }
}

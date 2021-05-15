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
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::alphanumeric1,
    error::ParseError,
    sequence::{delimited, preceded},
    IResult,
};

// Notes about the current JSONata implementation (TODO: remove)
// - variables must start with `$`
// - local variable bindings shadow built-ins
// - using an undefined variable is not an error sometimes, but other times it is
// - field reference starting with "$" doesn't work unless it has quotes, which means...
// - quoted values might be literal strings or field references
// - field references that are quoted don't handle unicode correctly

/// Parses an identifier for a path element
///
/// Identifiers may start with an alphabetic character
/// then followed by one or more alphanumeric characters, or
/// may be any string inside backticks.
pub(super) fn path_ident<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    alt((
        delimited(tag("`"), take_while(|c: char| c != '`'), tag("`")),
        alphanumeric1,
    ))(input)
}

/// Parses a variable ident
///
/// Variable names always start with a `$` symbol
pub(super) fn variable_ident<'a, E: ParseError<&'a str>>(
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
    fn path_regular() {
        let input = "account";
        let res = path_ident::<(&str, ErrorKind)>(input);
        assert_eq!(res.unwrap().1, "account");
    }

    #[test]
    fn path_with_dot() {
        let input = "account.name";
        let res = path_ident::<(&str, ErrorKind)>(input);
        assert_eq!(res.unwrap(), (".name", "account"));
    }

    #[test]
    fn path_backticks() {
        let input = "`account name`";
        let res = path_ident::<(&str, ErrorKind)>(input);
        assert_eq!(res.unwrap().1, "account name");
    }
    #[test]
    fn variable_ident_parser() {
        let input = "$myVar";
        let res = variable_ident::<(&str, ErrorKind)>(input);
        assert_eq!(res.unwrap().1, "myVar");
    }

    #[test]
    fn variable_ident_parser_underscore() {
        let input = "$my_var";
        let res = variable_ident::<(&str, ErrorKind)>(input);
        assert_eq!(res.unwrap().1, "my_var");
    }
}

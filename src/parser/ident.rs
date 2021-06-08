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
    bytes::complete::{tag, take_till1, take_while},
    character::complete::{alpha1, alphanumeric1},
    combinator::recognize,
    error::ParseError,
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult,
};

/// Parses an identifier for a path element
///
/// Identifiers may start with an alphabetic or underscore character
/// then followed by one or more alphanumeric characters, or
/// may be any string inside backticks.
pub(super) fn path_ident<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    alt((
        delimited(tag("`"), take_till1(|c| c == '`'), tag("`")),
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
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
        assert_eq!(res, Ok(("", "account")));
    }

    #[test]
    fn path_start_with_underscore() {
        let input = "_account";
        let res = path_ident::<(&str, ErrorKind)>(input);
        assert_eq!(res, Ok(("", "_account")));
    }

    #[test]
    fn path_numeric_first_char_bad() {
        let input = "1account";
        let res = path_ident::<(&str, ErrorKind)>(input);
        // assert_eq!(Err(Error));
        println!("{:?}", res);
    }

    #[test]
    fn path_with_dot() {
        let input = "account.name";
        let res = path_ident::<(&str, ErrorKind)>(input);
        assert_eq!(res, Ok((".name", "account")));
    }

    #[test]
    fn path_backticks() {
        let input = "`account name`";
        let res = path_ident::<(&str, ErrorKind)>(input);
        assert_eq!(res, Ok(("", "account name")));
    }
    #[test]
    fn variable_ident_parser() {
        let input = "$myVar";
        let res = variable_ident::<(&str, ErrorKind)>(input);
        assert_eq!(res, Ok(("", "myVar")));
    }

    #[test]
    fn variable_ident_parser_underscore() {
        let input = "$my_var";
        let res = variable_ident::<(&str, ErrorKind)>(input);
        assert_eq!(res, Ok(("", "my_var")));
    }
}

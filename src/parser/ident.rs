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
    combinator::{map, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult,
};

use super::Span;

/// Parses an identifier for a path element
///
/// Identifiers may start with an alphabetic or underscore character
/// then followed by one or more alphanumeric characters, or
/// may be any string inside backticks.
pub(super) fn path_ident(span: Span) -> IResult<Span, String> {
    // let z = map(
    //     delimited(tag("`"), take_till1(|c| c == '`'), tag("`")),
    //     |s: &str| s.to_string(),
    // )(&span);
    alt((path_ident_backticks, path_ident_plain))(span)
}

/// Parses an identifier for a path element delimited by backticks
///
/// Path identifiers can be delimited by backticks which allows the
/// identifier to contain spaces and other characters not normally
/// allowed.
fn path_ident_backticks(span: Span) -> IResult<Span, String> {
    map(
        delimited(tag("`"), take_till1(|c| c == '`'), tag("`")),
        |s: Span| s.fragment().to_string(),
    )(span)
}

/// Parses an identifier for a path element not delimited by backticks
///
/// The path identifier must start with an alphabetic character or underscore,
/// then contains any number of following characters that are alphanumeric or
/// underscore.
///
/// ```
/// field_1  // yes
/// _field   // yes
/// 1field   // no
/// ______   // yes
/// ```
fn path_ident_plain(span: Span) -> IResult<Span, String> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: Span| s.fragment().to_string(),
    )(span)
}

/// Parses a variable ident
///
/// Variable names always start with a `$` symbol
pub(super) fn variable_ident(span: Span) -> IResult<Span, String> {
    map(
        preceded(
            tag("$"),
            take_while(|c: char| c.is_alphanumeric() || c == '_'),
        ),
        |s: Span| s.fragment().to_string(),
    )(span)
}

#[cfg(test)]
mod tests {

    use crate::parser::make_span;

    use super::*;

    #[test]
    fn path_regular() {
        let input = "account";
        let (_, actual) = path_ident(make_span(input)).unwrap();
        assert_eq!(actual, "account".to_string());
    }

    #[test]
    fn path_start_with_underscore() {
        let input = "_account";
        let (_, actual) = path_ident(make_span(input)).unwrap();
        assert_eq!(actual, "_account".to_string());
    }

    #[test]
    fn path_numeric_first_char_bad() {
        let input = "1account";
        let res = path_ident(make_span(input));
        // TODO:
        // assert_eq!(Err(Error));
        println!("{:?}", res);
    }

    #[test]
    fn path_with_dot() {
        let input = "account.name";
        let (remainder, actual) = path_ident(make_span(input)).unwrap();
        assert_eq!(actual, "account".to_string());
        assert_eq!(remainder.fragment(), &".name",);
    }

    #[test]
    fn path_backticks() {
        let input = "`account name`";
        let (_, actual) = path_ident(make_span(input)).unwrap();
        assert_eq!(actual, "account name".to_string());
    }

    #[test]
    fn variable_ident_parser() {
        let input = "$myVar";
        let (_, actual) = variable_ident(make_span(input)).unwrap();
        assert_eq!(actual, "myVar".to_string());
    }

    #[test]
    fn variable_ident_parser_underscore() {
        let input = "$my_var";
        let (_, actual) = variable_ident(make_span(input)).unwrap();
        assert_eq!(actual, "my_var".to_string());
    }
}

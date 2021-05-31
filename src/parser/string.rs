//! This module parses strings that may contain escaped characters.
//!
//! - String must be enclosed by double quotes
//! - May contain any character unescaped except / and ""
//! - Unicode code points are represented as \u{X} where X is 1-6 hex characters
//! FIXME: JSONata uses the JSON escaping which is \uXXXX (only 4 characters)

use nom::{
    branch::alt,
    bytes::complete::{is_not, take_while_m_n},
    combinator::{map, map_opt, map_res, verify},
    error::{FromExternalError, ParseError},
    multi::fold_many0,
    sequence::{delimited, preceded},
    IResult,
};
use nom::{character::complete::char as nom_char, combinator::value};

use crate::ast::literal::LiteralValue;
/// Parses a str containing `u{X}` where X
/// is a hexadecimal number {0,1,2,3,4,5,6,7,8,9,a,b,c,d,e,f}
/// occuring between 1 and 6 times to a char (Unicode scalar value)
fn parse_unicode<'a, E>(input: &'a str) -> IResult<&str, char, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    // parses `u{X}` for between 1 and 6 X that are hex digits
    let parse_hex = preceded(
        nom_char('u'),
        delimited(
            nom_char('{'),
            take_while_m_n(1usize, 6usize, |c: char| c.is_ascii_hexdigit()),
            nom_char('}'),
        ),
    );

    // Parses the X{1,6} string value into a u32
    let parse_u32 = map_res(parse_hex, move |hex| u32::from_str_radix(hex, 16));

    // Maps the u32 value to its char value
    map_opt(parse_u32, |value| std::char::from_u32(value))(input)
}

/// Parses a JSONata escape character (these are the same as JSON escape characters)
fn parse_escaped_char<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    preceded(
        nom_char('\\'),
        alt((
            parse_unicode,
            value('\n', nom_char('n')),
            value('\r', nom_char('r')),
            value('\t', nom_char('t')),
            value('\u{08}', nom_char('b')),
            value('\u{0C}', nom_char('f')),
            value('\\', nom_char('\\')),
            value('"', nom_char('"')),
        )),
    )(input)
}

/// Parses text that does not contain a quote or backslash character and is not empty
fn parse_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let not_quote_or_slash = is_not("\"\\");

    verify(not_quote_or_slash, |s: &str| !s.is_empty())(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
}

/// Parses a single fragment of a string, which can be a literal str or
/// a character that has been escaped (JSON escape char or unicode scalar value)
fn parse_fragment<'a, E>(input: &'a str) -> IResult<&'a str, StringFragment<'a>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
    ))(input)
}

/// Parses the entire string input which is defined by text surrounded
/// by double quotes.
/// An unescaped double quote ends the string parsing.
///
/// TODO: JSONata allows string literals to be delimited by single quotes
pub(crate) fn parse_string<'a, E>(input: &'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let build_string = fold_many0(parse_fragment, String::new(), |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(s),
            StringFragment::EscapedChar(c) => string.push(c),
        }
        string
    });

    delimited(nom_char('"'), build_string, nom_char('"'))(input)
}

pub(crate) fn literal_string<'a, E>(input: &'a str) -> IResult<&'a str, LiteralValue, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    map(parse_string, |s| LiteralValue::from(s))(input)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use super::*;

    #[test]
    fn unicode_parser<'a>() {
        let inputs = vec![
            ("u{20}", ' '),
            ("u{A3}", '£'),
            ("u{100}", 'Ā'),
            ("u{10A0}", 'Ⴀ'),
            ("u{1F605}", '😅'),
        ];

        for (input, expected) in inputs {
            assert_eq!(
                Ok(("", expected)),
                parse_unicode::<'a, VerboseError<&str>>(input)
            );
        }
    }

    #[test]
    fn escaped_char_parser<'a>() {
        let inputs = vec![
            (r#"\n"#, '\n'),
            (r#"\t"#, '\t'),
            (r#"\r"#, '\r'),
            (r#"\\"#, '\\'),
            (r#"\""#, '"'),
            (r#"\b"#, '\u{08}'),
            (r#"\f"#, '\u{0C}'),
            (r#"\u{1F605}"#, '😅'),
        ];

        for (input, expected) in inputs {
            assert_eq!(
                Ok(("", expected)),
                parse_escaped_char::<'a, VerboseError<&str>>(input)
            );
        }
    }

    #[test]
    fn literal_parser<'a>() {
        let inputs = vec![
            ("hello", "hello", ""),
            ("before\"after", "before", "\"after"),
            ("before\\after", "before", "\\after"),
        ];

        for (input, expected, remainder) in inputs {
            assert_eq!(
                Ok((remainder, expected)),
                parse_literal::<'a, VerboseError<&str>>(input)
            )
        }
    }

    #[test]
    fn fragment_parser<'a>() {
        let inputs = vec![
            ("hello", StringFragment::Literal("hello"), ""),
            (r#"\\"#, StringFragment::EscapedChar('\\'), ""),
            (
                r#"before\\after"#,
                StringFragment::Literal("before"),
                r#"\\after"#,
            ),
            (
                r#"\u{1F605}after"#,
                StringFragment::EscapedChar('😅'),
                "after",
            ),
        ];

        for (input, expected, remainder) in inputs {
            assert_eq!(
                Ok((remainder, expected)),
                parse_fragment::<'a, VerboseError<&str>>(input)
            );
        }
    }

    #[test]
    fn string_parser<'a>() {
        let inputs = vec![
            (r#""hello""#, "hello".to_string(), ""),
            (r#""emoji\u{1F605}""#, "emoji😅".to_string(), ""),
        ];

        for (input, expected, remainder) in inputs {
            assert_eq!(
                Ok((remainder, expected)),
                parse_string::<'a, VerboseError<&str>>(input)
            )
        }
    }
}

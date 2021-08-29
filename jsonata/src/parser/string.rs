//! This module parses strings that may contain escaped characters.
//!
//! - String must be enclosed by double quotes
//! - May contain any character unescaped except / and ""
//! - Unicode code points are represented as \u{X} where X is 1-6 hex characters
//! FIXME: JSONata uses the JSON escaping which is \uXXXX (only 4 characters)

use nom::{
    branch::alt,
    bytes::complete::{is_not, take_while_m_n},
    combinator::{map, map_opt},
    multi::fold_many0,
    sequence::{delimited, preceded},
    IResult,
};
use nom::{character::complete::char as nom_char, combinator::value};

use crate::ast::literal::LiteralValue;

use super::Span;

/// Parses a str containing `uXXXX` where X
/// is a hexadecimal number {0,1,2,3,4,5,6,7,8,9,A,B,C,D,E,F}
/// occuring exactly 4 times to a char (Unicode scalar value)
fn parse_unicode(span: Span) -> IResult<Span, char> {
    map_opt(parse_unicode_to_u32, |value| std::char::from_u32(value))(span)
}

/// Parses the 4 hexadecimal characters after the 'u' character
/// into its numerical represenation as a u32
///
/// 'Ā' is u+0100 which is 256 in decimal
///
/// TODO: Investigate if u32 parsing could ever error. Refactor to remove `.unwrap` if so
/// (map the error into the Err side of IResult), otherwise if it can't error (I believe it
/// can't because it will always be given appropriate input) then leave a comment justifying
/// the `.unwrap`
fn parse_unicode_to_u32(span: Span) -> IResult<Span, u32> {
    map(parse_uxxxx, |hex| u32::from_str_radix(hex, 16).unwrap())(span)
}

fn parse_uxxxx(span: Span) -> IResult<Span, &str> {
    map(
        preceded(
            nom_char('u'),
            take_while_m_n(4, 4, |c: char| c.is_ascii_hexdigit()),
        ),
        |s: Span| *s.fragment(),
    )(span)
}

/// Parses a JSONata escape character (these are the same as JSON escape characters)
fn parse_escaped_char(span: Span) -> IResult<Span, char> {
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
    )(span)
}

/// Parses text that does not contain a quote or backslash character
fn parse_literal(span: Span) -> IResult<Span, String> {
    map(is_not("\"\\"), |s: Span| s.fragment().to_string())(span)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StringFragment {
    Literal(String),
    EscapedChar(char),
}

/// Parses a single fragment of a string, which can be a literal str or
/// a character that has been escaped (JSON escape char or unicode scalar value)
fn parse_fragment(span: Span) -> IResult<Span, StringFragment> {
    alt((
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
    ))(span)
}

/// Parses the entire string input which is defined by text surrounded
/// by double quotes.
/// An unescaped double quote ends the string parsing.
///
/// TODO: JSONata allows string literals to be delimited by single quotes
pub(crate) fn parse_string(span: Span) -> IResult<Span, String> {
    let build_string = fold_many0(parse_fragment, String::new(), |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(&s),
            StringFragment::EscapedChar(c) => string.push(c),
        }
        string
    });

    delimited(nom_char('"'), build_string, nom_char('"'))(span)
}

pub(crate) fn literal_string(span: Span) -> IResult<Span, LiteralValue> {
    map(parse_string, |s| LiteralValue::from(s))(span)
}

#[cfg(test)]
mod tests {

    use crate::parser::make_span;

    use super::*;

    #[test]
    fn unicode_parser() {
        let inputs = vec![
            ("u0020", ' '),
            ("u00A3", '£'),
            ("u0100", 'Ā'),
            ("u10A0", 'Ⴀ'),
            // ("u{1F605}", '😅'), // TODO: multiple code points
        ];

        for (input, expected) in inputs {
            let (_, actual) = parse_unicode(make_span(input)).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn escaped_char_parser() {
        let inputs = vec![
            (r#"\n"#, '\n'),
            (r#"\t"#, '\t'),
            (r#"\r"#, '\r'),
            (r#"\\"#, '\\'),
            (r#"\""#, '"'),
            (r#"\b"#, '\u{08}'),
            (r#"\f"#, '\u{0C}'),
            (r#"\u00A3"#, '£'),
            // (r#"\u{1F605}"#, '😅'), // TODO: multiple code points
        ];

        for (input, expected) in inputs {
            let (_, actual) = parse_escaped_char(make_span(input)).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn literal_parser() {
        let inputs = vec![
            ("hello", "hello", ""),
            ("before\"after", "before", "\"after"),
            ("before\\after", "before", "\\after"),
        ];

        for (input, expected, expected_remainder) in inputs {
            let (actual_remainder, actual) = parse_literal(make_span(input)).unwrap();
            assert_eq!(expected.to_string(), actual);
            assert_eq!(expected_remainder, *actual_remainder);
        }
    }

    #[test]
    fn fragment_parser() {
        let inputs = vec![
            ("hello", StringFragment::Literal("hello".to_string()), ""),
            (r#"\\"#, StringFragment::EscapedChar('\\'), ""),
            (
                r#"before\\after"#,
                StringFragment::Literal("before".to_string()),
                r#"\\after"#,
            ),
            (r#"\u0100after"#, StringFragment::EscapedChar('Ā'), "after"),
        ];

        for (input, expected, expected_remainder) in inputs {
            let (actual_remainder, actual) = parse_fragment(make_span(input)).unwrap();
            assert_eq!(expected, actual);
            assert_eq!(expected_remainder, *actual_remainder)
        }
    }

    #[test]
    fn string_parser() {
        let inputs = vec![
            (r#""hello""#, "hello".to_string(), ""),
            (r#""pound\u00A3""#, "pound£".to_string(), ""),
        ];

        for (input, expected, expected_remainder) in inputs {
            let (actual_remainder, actual) = parse_string(make_span(input)).unwrap();
            assert_eq!(expected, actual);
            assert_eq!(expected_remainder, *actual_remainder);
        }
    }
}

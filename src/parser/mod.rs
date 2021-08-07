mod dyadic;
mod ident;
mod monadic;
mod string;
#[cfg(test)]
mod tests;

use nom::combinator::map;
use nom::multi::separated_list0;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::space0,
    combinator::value,
    error::ParseError,
    sequence::{delimited, tuple},
    AsChar, IResult, InputIter, InputLength, InputTake, InputTakeAtPosition, Parser,
};
use nom_locate::LocatedSpan;
use nom_recursive::RecursiveInfo;

use crate::ast::expr::Expression;
use crate::ast::expr::MultiExpression;

use self::dyadic::comparison_expr;
use self::dyadic::map_expr;
use self::dyadic::variable_binding_expr;
use self::monadic::{literal_expr, path_expr};

/// Type alias for the internal output of parser functions. The nom-locate and nom-recursive
/// crates are used to provide location information of the parsed input and to allow
/// for left-recursion.
type Span<'a> = LocatedSpan<&'a str, RecursiveInfo>;

pub type ParseResult<'a> = Result<Expression, nom::Err<nom::error::Error<Span<'a>>>>;
// Result<Expression, nom::Err<nom::error::Error<LocatedSpan<&'a str, RecursiveInfo>>>>;

fn make_span(s: &str) -> Span {
    LocatedSpan::new_extra(s, RecursiveInfo::new())
}

/// Parses the provided parser, ignoring spaces before
/// and after the matching input.
fn trim<'a, F, I, O, E: ParseError<I>>(parser: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, O, E>,
    I: InputLength + InputIter + InputTakeAtPosition + InputTake + Clone,
    <I as InputIter>::Item: AsChar + Clone,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(space0, parser, space0)
}

/// Parses a C-Style comment
///
/// Comments begin with the `/*` characters and close with the `*/` characters.
/// TODO: Decide if we want to retain comments in the AST
fn comment(span: Span) -> IResult<Span, ()> {
    map(
        value((), tuple((tag("/*"), take_until("*/"), tag("*/")))),
        |()| (),
    )(span)
}

fn not_whitespace(i: &str) -> IResult<&str, &str> {
    is_not(" \t")(i)
}

fn escaped_tab(i: &str) -> IResult<&str, &str> {
    nom::combinator::recognize(nom::character::complete::char('\t'))(i)
}

fn escaped_backslash(i: &str) -> IResult<&str, &str> {
    nom::combinator::recognize(nom::character::complete::char('\\'))(i)
}

fn transform_escaped(i: &str) -> IResult<&str, std::string::String> {
    nom::bytes::complete::escaped_transform(
        nom::bytes::complete::is_not("\\"),
        '\\',
        nom::branch::alt((escaped_backslash, escaped_tab)),
    )(i)
}

/// Parses a block delimited by parentheses into a vector of expressions
///
/// ```
/// (
///    Account;
///    true;
///    Address
/// )
/// ```
/// The above JSONata program is parsed into a MultiExpression where the
/// internal Vec<Expression> contains PathExpression(Account), LiteralExpression(true),
/// and PathExpression(Address).
///
/// The final semi-colon is optional.
fn multiexpression_parser(input: Span) -> IResult<Span, Expression> {
    map(
        delimited(tag("("), separated_list0(tag(";"), expr_parser), tag(")")),
        |expressions| MultiExpression { expressions }.into(),
    )(input)
}

// fn expr_test(span: Span) -> IResult<Span, String> {
//     alt((comparison_expr_test, term))(span)
// }

/// Main function for expression parsing
///
/// Calls each of the other parsers in order until a parser
/// yields success, or returns a ParseError
fn expr_parser(span: Span) -> IResult<Span, Expression> {
    alt((
        multiexpression_parser,
        literal_expr,
        variable_binding_expr,
        path_expr,
        map_expr,
        comparison_expr,
    ))(span)
}

/// Parses the given input to produce an AST of expressions
///
/// The result of this function is always a single Expression node,
/// however, Expressions may have many Expressions contained within them.
///
/// ```
/// (
///    /* Get the person's name, i.e. 'John' */
///    $name := Account.Name;
///    /* Return how many orders, i.e. 'John: 5 orders' */
///    $name & ": " & $count(Orders) & " orders"
/// )
/// ```
/// The above JSONata program evaluates to a single top-level expression,
/// in this case, a MultiExpression which has a Vec<Expression> and evaluates
/// each in order and uses the final value as its return value.
pub(super) fn parse(input: &str) -> ParseResult {
    expr_parser(make_span(input)).map(|(span, expr)| {
        if !span.is_empty() {
            panic!("Unparsed input, remaining: '{}'", span.fragment())
        }
        expr
    })
}

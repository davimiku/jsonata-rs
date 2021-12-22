use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some(Self::Item { kind, text })
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Logos, FromPrimitive, ToPrimitive,
)]
pub(crate) enum SyntaxKind {
    // Keywords
    #[token("function")]
    FunctionKeyword,

    // Identifiers
    #[regex("\\$[A-Za-z0-9]+")]
    Ident,

    #[token("$")]
    Dollar,

    #[regex("[0-9]+")]
    Number,

    // Operators
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("<")]
    LAngle,

    #[token(">")]
    RAngle,

    #[token(":=")]
    ColonEquals,

    #[token("=")]
    Equals,

    #[token("..")]
    DotDot,

    #[token(".")]
    Dot,

    #[token("~>")]
    TildeRAngle,

    // Delimiters
    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("?")]
    Question,

    #[token(":")]
    Colon,

    #[token(";")]
    SemiColon,

    // Non-code
    #[token("/*", |lex| {
        let len = lex.remainder().find("*/")?;
        lex.bump(len + 2); // include len of `*/`
        Some(())
    })]
    Comment,

    #[regex("[ \n]+")]
    Whitespace,

    #[error]
    Error,

    Root,
    BinaryExpr,
    Literal,
    ParenExpr,
    PrefixExpr,
    VariableRef,
}

impl SyntaxKind {
    pub(crate) fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Lexeme<'a> {
    pub(crate) kind: SyntaxKind,
    pub(crate) text: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Lexeme { kind, text: input }));
    }

    #[test]
    fn lex_function_keyword() {
        check("function", SyntaxKind::FunctionKeyword);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("$abcd", SyntaxKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("$ab123cde456", SyntaxKind::Ident);
    }

    #[test]
    fn lex_numeric_identifier() {
        check("$2", SyntaxKind::Ident);
    }

    #[test]
    fn lex_single_char_identifier() {
        check("$x", SyntaxKind::Ident);
    }

    #[test]
    fn lex_dollar() {
        check("$", SyntaxKind::Dollar);
    }

    #[test]
    fn lex_number() {
        check("123456", SyntaxKind::Number);
    }

    #[test]
    fn lex_plus() {
        check("+", SyntaxKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", SyntaxKind::Minus);
    }

    #[test]
    fn lex_star() {
        check("*", SyntaxKind::Star);
    }

    #[test]
    fn lex_left_angle() {
        check("<", SyntaxKind::LAngle);
    }

    #[test]
    fn lex_right_angle() {
        check(">", SyntaxKind::RAngle);
    }

    #[test]
    fn lex_slash() {
        check("/", SyntaxKind::Slash);
    }

    #[test]
    fn lex_colon_equals() {
        check(":=", SyntaxKind::ColonEquals);
    }

    #[test]
    fn lex_equals() {
        check("=", SyntaxKind::Equals);
    }

    #[test]
    fn lex_dotdot() {
        check("..", SyntaxKind::DotDot);
    }

    #[test]
    fn lex_dot() {
        check(".", SyntaxKind::Dot);
    }

    #[test]
    fn lex_tilde_right_angle() {
        check("~>", SyntaxKind::TildeRAngle);
    }

    #[test]
    fn lex_left_brace() {
        check("{", SyntaxKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check("}", SyntaxKind::RBrace);
    }

    #[test]
    fn lex_left_paren() {
        check("(", SyntaxKind::LParen);
    }

    #[test]
    fn lex_right_paren() {
        check(")", SyntaxKind::RParen);
    }

    #[test]
    fn lex_left_bracket() {
        check("[", SyntaxKind::LBracket);
    }

    #[test]
    fn lex_right_bracket() {
        check("]", SyntaxKind::RBracket);
    }

    #[test]
    fn lex_question() {
        check("?", SyntaxKind::Question);
    }

    #[test]
    fn lex_colon() {
        check(":", SyntaxKind::Colon);
    }

    #[test]
    fn lex_semicolon() {
        check(";", SyntaxKind::SemiColon);
    }

    #[test]
    fn lex_spaces() {
        check("   ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_comment() {
        check("/* test */", SyntaxKind::Comment);
    }

    #[test]
    fn lex_empty_comment() {
        check("/**/", SyntaxKind::Comment);
    }

    #[test]
    fn lex_star_comment() {
        check("/***/", SyntaxKind::Comment);
    }

    #[test]
    fn lex_spaces_and_newlines() {
        check("  \n ", SyntaxKind::Whitespace);
    }
}

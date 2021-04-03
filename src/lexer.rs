use logos::{Lexer, Logos};

pub fn lex_tokens(input: &str) -> Vec<Token> {
    Token::lexer(input).collect::<Vec<Token>>()
}

#[derive(Logos, Debug, PartialEq)]
#[logos(subpattern decimal = r"[0-9][_0-9]*")]
#[logos(subpattern hex = r"[0-9a-fA-F][_0-9a-fA-F]*")]
#[logos(subpattern octal = r"[0-7][_0-7]*")]
#[logos(subpattern binary = r"[0-1][_0-1]*")]
#[logos(subpattern exp = r"[eE][+-]?[0-9][_0-9]*")]
pub enum Token {
    // Words
    #[token("function")]
    Function,
    #[token("in")]
    In,
    #[token("and")]
    And,
    #[token("or")]
    Or,

    // Grouping
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,

    // Combination symbols
    #[token(":=")]
    ColonEqual,
    #[token("<=")]
    LeftCaretEqual,
    #[token(">=")]
    RightCaretEqual,
    #[token("~>")]
    TildeRightCaret,
    #[token("**")]
    StarStar,
    #[token("..")]
    DotDot,

    // Single symbols
    #[token("^")]
    Caret,
    #[token("<")]
    LeftCaret,
    #[token(">")]
    RightCaret,
    #[token("=")]
    Equal,
    #[token("*")]
    Star,
    #[token("#")]
    Pound,
    #[token("@")]
    At,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("&")]
    Ampersand,
    #[token("|")]
    Pipe,
    #[token("%")]
    Percent,
    #[token(".")]
    Dot,
    #[token("$")]
    Dollar,
    #[token("+")]
    Plus,
    #[token("-")]
    Dash,
    #[token("/")]
    Slash,
    #[token(",")]
    Comma,

    // Variables
    // #[regex("\\$[a-zA-Z]+")]
    // Variable(String),
    #[regex("(?&decimal)", |lex| lex.slice().parse())]
    Integer(i32),

    // Identifiers
    #[regex("`([^`])+`", backtick_identifier)]
    #[regex("[a-zA-Z]+", |lex| lex.slice().to_string())]
    Ident(String),

    // Skip spaces and fallthrough for errors
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)] // any space
    #[regex(r"/\*([^*]|\*+[^*/])*\*+/", logos::skip)] // block comments
    Error,
}

// TODO: Figure out if there's a better way to capture what's between
// the backticks as a &str

/// Remove the backticks from the captured identifier
fn backtick_identifier(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    Some(slice[1..slice.len() - 1].into())
}

#[cfg(test)]
mod tests {

    use super::*;
    use logos::Logos;

    #[test]
    fn single_dot() {
        let actual = lex_tokens(".");
        let expected = vec![Token::Dot];

        assert_eq!(expected, actual);
    }

    #[test]
    fn single_ident() {
        let mut lex = Token::lexer("Surname");
        assert_eq!(lex.next(), Some(Token::Ident("Surname".to_string())));
        assert_eq!(lex.slice(), "Surname");
    }

    #[test]
    fn field_reference() {
        let actual = lex_tokens("Address.City");
        let expected = vec![
            Token::Ident("Address".to_string()),
            Token::Dot,
            Token::Ident("City".to_string()),
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn range() {
        let actual = lex_tokens("[1..5]");
        let expected = vec![
            Token::LeftBracket,
            Token::Integer(1),
            Token::DotDot,
            Token::Integer(5),
            Token::RightBracket,
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn backtick_ident() {
        let mut lex = Token::lexer("`one two`");
        assert_eq!(lex.next(), Some(Token::Ident("one two".to_string())));
    }
}

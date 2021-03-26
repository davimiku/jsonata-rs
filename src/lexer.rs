use logos::{Lexer, Logos};

// TODO: Figure out if there's a way to capture what's between the
// backticks as a &str

fn backtick_identifier(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    Some(slice[1..slice.len() - 1].into())
}

fn identifier(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    Some(slice.into())
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // Literals

    // Punctuation
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,

    #[token("=")]
    Equal,
    #[token("**")]
    StarStar,
    #[token("*")]
    Star,

    #[token(".")]
    Dot,

    #[token("$")]
    Dollar,

    #[regex("`([^`])*`", backtick_identifier)]
    #[regex("[a-zA-Z]+", identifier)]
    Ident(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[cfg(test)]
mod tests {

    use super::Token;
    use logos::Logos;

    fn get_token_vector(input: &str) -> Vec<Token> {
        Token::lexer(input).collect::<Vec<Token>>()
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn just_dot() {
        let actual = get_token_vector(".");
        let expected = vec![Token::Dot];

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_ident() {
        let mut lex = Token::lexer("Surname");
        assert_eq!(lex.next(), Some(Token::Ident("Surname".to_string())));
        assert_eq!(lex.slice(), "Surname");
    }

    #[test]
    fn field_reference() {
        let mut lex = Token::lexer("Address.City");
        assert_eq!(lex.next(), Some(Token::Ident("Address".to_string())));
        assert_eq!(lex.next(), Some(Token::Dot));
        assert_eq!(lex.next(), Some(Token::Ident("City".to_string())));
    }

    #[test]
    fn test_backticks() {
        let mut lex = Token::lexer("`one`");
        assert_eq!(lex.next(), Some(Token::Ident("one".to_string())));
    }

    #[test]
    fn test_slice() {
        let s = "`test`";
        let s2 = &s[1..];
        assert_eq!(s2, "test`");
    }
}

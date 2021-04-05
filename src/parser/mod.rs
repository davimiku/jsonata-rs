pub(crate) mod concat;

use std::iter::Peekable;

use crate::ast::{literal::LiteralExpression, path::PathExpression, Program};
use crate::lexer::Token;

pub fn parse<I: Iterator<Item = Token>>(tok_iter: I) -> Result<Program, ParseError> {
    let mut parser = Parser::new(tok_iter);
    parser.parse()
}

pub struct Parser<I: Iterator<Item = Token>> {
    // tokens: Vec<Token>,
    tok_iter: Peekable<I>,
    // curr: Option<&'a Token>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tok_iter: I) -> Self {
        let peekable_iter = tok_iter.peekable();
        Parser {
            tok_iter: peekable_iter,
            // curr: None,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Program::default();

        loop {
            // could clean this up with `match` or `if let`, TBD
            if self.tok_iter.peek().is_none() {
                break;
            }
            let token = self.tok_iter.next().unwrap();

            match token {
                Token::LeftParen => {
                    // The program may have multiple statements
                }
                Token::IntegerLiteral(i) => {
                    let expr = LiteralExpression::from_int(i);
                }
                Token::StringLiteral(s) => {
                    let expr = LiteralExpression::from_string(s);
                    match self.tok_iter.next() {
                        Some(Token::Ampersand) => {
                            // String concatenation
                            self.parse_string_concat(expr);
                        }
                        Some(_) => {}
                        None => {}
                    }
                }

                Token::Ident(s) => {
                    let path = self.parse_path(&s)?;
                    program.return_expression = Some(Box::new(PathExpression {
                        ident: s,
                        member: path.member,
                    }));
                }
                Token::RightParen => {
                    // We've safely reached the end of the program
                }
                _ => {}
            }
        }
        Ok(program)
    }

    fn parse_path(&mut self, s: &String) -> Result<PathExpression, ParseError> {
        let mut path = PathExpression {
            ident: s.to_string(),
            member: None,
        };
        match self.tok_iter.next() {
            Some(Token::Dot) => {
                // object property
                match self.tok_iter.next() {
                    Some(Token::Ident(s)) => {
                        let inner_path = self.parse_path(&s)?;
                        path.member = Some(Box::new(inner_path));
                    }
                    Some(_) => return Err(ParseError::NotImplemented),
                    None => return Err(ParseError::Syntax), // unexpected end of input
                }
            }
            Some(Token::LeftBracket) => {
                // Array index, predicate expression, or array constructor
            }
            Some(Token::Caret) => {
                // Order-by
            }
            Some(Token::LeftCurly) => {
                // Object constructor or "reduce"
            }
            Some(Token::Semicolon) => {
                // Syntax error
            }

            Some(_) => {
                // End of path expression
            }
            None => {
                // end of input, also implies end of path expression
            }
        }

        Ok(path)
    }

    fn parse_string_literal(&mut self, s: String) -> Result<LiteralExpression, ()> {
        Ok(LiteralExpression::from_string(s))
    }

    fn parse_integer_literal(&mut self, i: i64) -> Result<LiteralExpression, ()> {
        Ok(LiteralExpression::from_int(i))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseError {
    Syntax,

    NotImplemented, // FIXME: Implement specific errors
}

#[cfg(test)]
mod tests {

    impl<I: Iterator<Item = Token>> Parser<I> {
        /// Advances the token iterator in the Parser
        ///
        /// Useful for tests to allow focusing on specific input
        /// parsing.
        fn advance(&mut self) {
            self.tok_iter.next();
        }
    }

    use std::vec::IntoIter;

    use super::*;
    use crate::lexer::lex_tokens;

    fn make_parser(input: &str) -> Parser<IntoIter<Token>> {
        Parser::new(lex_tokens(input).into_iter())
    }

    #[test]
    fn one_level_path() {
        let input = "name";
        let mut parser = make_parser(input);
        parser.advance(); // simulate first token already processed

        let actual = parser.parse_path(&input.to_string()).unwrap();
        let expected = PathExpression {
            ident: "name".to_string(),
            member: None,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn two_level_path() {
        let input = "address.city";
        let mut parser = make_parser(input);
        parser.advance(); // simulate first token already processed

        let actual = parser.parse_path(&"address".to_string()).unwrap();

        let expected = PathExpression {
            ident: "address".to_string(),
            member: Some(Box::new(PathExpression {
                ident: "city".to_string(),
                member: None,
            })),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn three_level_path() {
        let input = "address.location.latitude";
        let mut parser = make_parser(input);
        parser.advance(); // simulate first token already processed

        let actual = parser.parse_path(&"address".to_string()).unwrap();

        let expected = PathExpression {
            ident: "address".to_string(),
            member: Some(Box::new(PathExpression {
                ident: "location".to_string(),
                member: Some(Box::new(PathExpression {
                    ident: "latitude".to_string(),
                    member: None,
                })),
            })),
        };

        assert_eq!(actual, expected);
    }
}

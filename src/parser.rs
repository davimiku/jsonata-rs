use std::iter::Peekable;

use crate::ast::{path::PathExpression, Program};
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

        let mut next = self.tok_iter.next();

        loop {
            if next.is_none() {
                break;
            }
            let token = next.unwrap();

            match token {
                Token::LeftParen => {
                    // The program may have multiple statements
                }
                Token::Ident(s) => {
                    let path = self.parse_path(&s)?;
                    program.return_expression = Some(Box::new(PathExpression {
                        ident: s,
                        member: path.member,
                    }));
                }
                _ => {}
            }
            next = self.tok_iter.next();
        }
        Ok(program)
    }

    fn parse_path(&mut self, s: &String) -> Result<PathExpression, ParseError> {
        let mut path = PathExpression {
            ident: s.to_string(),
            member: None,
        };
        match self.tok_iter.peek() {
            Some(Token::Dot) => {
                // object property
                self.tok_iter.next(); // dot
                let next = self.tok_iter.next();
                match next {
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
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseError {
    Syntax,

    NotImplemented, // FIXME: Implement specific errors
}

#[cfg(test)]
mod tests {

    // use super::*;
    // use crate::{ast::PathExpression, evaluate::Evaluatable};

    // fn make_program<E: Evaluatable>(expr: Expression<E>) -> Program {
    //     let program = Program::default();
    //     program.return_expression = Some(Box::new(expr));
    //     program
    // }

    // #[test]
    // fn single_path_identifier() {
    //     let result = parse("Surname");
    //     let evalutated = result.unwrap().return_expression.unwrap();
    //     let actual = PathExpression {
    //         ident: evalutated.
    //     }
    //     let expected = PathExpression {
    //         ident: "Surname".to_string(),
    //         member: None,
    //     };
    //     assert_eq!(*actual, expected);
    // }

    // #[test]
    // fn one_level_path() {
    //     let result = parse("Address.City");
    //     let actual = result.unwrap();
    //     let expected = Expression::Path(PathExpression {
    //         ident: "Address".to_string(),
    //         member: Box::new(Some(PathExpression {
    //             ident: "City".to_string(),
    //             member: Box::new(None),
    //         })),
    //     });

    //     assert_eq!(actual, make_program(expected));
    // }
}

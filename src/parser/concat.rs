use crate::{ast::expression::Expression, lexer::Token};

use super::Parser;

impl<I: Iterator<Item = Token>> Parser<I> {
    pub(crate) fn parse_string_concat<E>(&mut self, expr: E) -> Result<(), ()>
    where
        E: Expression,
    {
        todo!()
    }
}

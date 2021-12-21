mod event;
mod expr;
mod sink;
mod source;

use crate::lexer::{Lexeme, Lexer, SyntaxKind};
use crate::syntax::SyntaxNode;
use expr::expr;
use rowan::GreenNode;

use self::event::Event;
use self::sink::Sink;
use self::source::Source;

pub fn parse(input: &str) -> Parse {
    let lexemes: Vec<_> = Lexer::new(input).collect();
    let parser = Parser::new(&lexemes);
    let events = parser.parse();
    let sink = Sink::new(&lexemes, events);

    Parse {
        green_node: sink.finish(),
    }
}

struct Parser<'l, 'input> {
    source: Source<'l, 'input>,
    events: Vec<Event>,
}

impl<'l, 'input> Parser<'l, 'input> {
    fn new(lexemes: &'l [Lexeme<'input>]) -> Self {
        Self {
            source: Source::new(lexemes),
            events: Vec::new(),
        }
    }

    fn parse(mut self) -> Vec<Event> {
        self.start_node(SyntaxKind::Root);
        expr(&mut self);
        self.finish_node();

        self.events
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    fn bump(&mut self) {
        let Lexeme { kind, text } = self
            .source
            .next_lexeme()
            .expect("bump is only called when there is a next lexeme");

        self.events.push(Event::AddToken {
            kind: *kind,
            text: (*text).into(),
        });
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.events.push(Event::StartNode { kind });
    }

    fn start_node_at(&mut self, checkpoint: usize, kind: SyntaxKind) {
        self.events.push(Event::StartNodeAt { kind, checkpoint });
    }

    fn finish_node(&mut self) {
        self.events.push(Event::FinishNode);
    }

    fn checkpoint(&self) -> usize {
        self.events.len()
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // Remove the newline from the end
        formatted[0..formatted.len() - 1].to_string()
    }
}

enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl InfixOp {
    /// Binding power tuple of (left, right)
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

enum PrefixOp {
    Neg,
}

impl PrefixOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use super::*;

    pub(super) fn check(input: &str, expected_tree: Expect) {
        let parse = parse(input);
        expected_tree.assert_eq(&parse.debug_tree());
    }

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Number@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_variable_ref() {
        check(
            "$counter",
            expect![[r#"
Root@0..8
  Ident@0..8 "$counter""#]],
        );
    }

    #[test]
    fn parse_negation() {
        check(
            "-10",
            expect![[r#"
Root@0..3
  PrefixExpr@0..3
    Minus@0..1 "-"
    Number@1..3 "10""#]],
        )
    }

    #[test]
    fn parse_nested_parentheses() {
        check(
            "((((((10))))))",
            expect![[r#"
Root@0..14
  LParen@0..1 "("
  LParen@1..2 "("
  LParen@2..3 "("
  LParen@3..4 "("
  LParen@4..5 "("
  LParen@5..6 "("
  Number@6..8 "10"
  RParen@8..9 ")"
  RParen@9..10 ")"
  RParen@10..11 ")"
  RParen@11..12 ")"
  RParen@12..13 ")"
  RParen@13..14 ")""#]],
        );
    }

    #[test]
    fn parse_whitespace() {
        check(
            "   ",
            expect![[r#"
Root@0..3
  Whitespace@0..3 "   ""#]],
        );
    }

    #[test]
    fn parse_number_preceded_by_whitespace() {
        check(
            "   9876",
            expect![[r#"
Root@0..7
  Whitespace@0..3 "   "
  Number@3..7 "9876""#]],
        );
    }

    #[test]
    fn parse_number_followed_by_whitespace() {
        check(
            "999   ",
            expect![[r#"
Root@0..6
  Number@0..3 "999"
  Whitespace@3..6 "   ""#]],
        );
    }

    #[test]
    fn parse_number_surrounded_by_whitespace() {
        check(
            " 123     ",
            expect![[r#"
Root@0..9
  Whitespace@0..1 " "
  Number@1..4 "123"
  Whitespace@4..9 "     ""#]],
        );
    }

    #[test]
    fn parse_binary_expression_with_whitespace() {
        check(
            " 1 +   2* 3 ",
            expect![[r#"
Root@0..12
  Whitespace@0..1 " "
  BinaryExpr@1..12
    Number@1..2 "1"
    Whitespace@2..3 " "
    Plus@3..4 "+"
    Whitespace@4..7 "   "
    BinaryExpr@7..12
      Number@7..8 "2"
      Star@8..9 "*"
      Whitespace@9..10 " "
      Number@10..11 "3"
      Whitespace@11..12 " ""#]],
        );
    }
}

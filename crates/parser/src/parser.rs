use syntax::SyntaxKind;

use crate::event::Event;
use crate::expr::expr;
use crate::marker::Marker;
use crate::source::Source;

pub(crate) struct Parser<'t, 'input> {
    pub(crate) source: Source<'t, 'input>,
    pub(crate) events: Vec<Event>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub(crate) fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
        }
    }

    pub(crate) fn parse(mut self) -> Vec<Event> {
        let m = self.start();
        expr(&mut self);
        m.complete(&mut self, SyntaxKind::Root);

        self.events
    }

    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(pos)
    }

    pub(crate) fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind().map(|kind| kind.into())
    }

    pub(crate) fn bump(&mut self) {
        self.source
            .next_token()
            .expect("bump is only called when there is a next token");

        self.events.push(Event::AddToken);
    }

    pub(crate) fn at(&mut self, kind: SyntaxKind) -> bool {
        self.peek() == Some(kind)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::check;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
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
    fn parse_comment() {
        check(
            "/* hello! */",
            expect![[r#"
Root@0..12
  Comment@0..12 "/* hello! */""#]],
        );
    }
}

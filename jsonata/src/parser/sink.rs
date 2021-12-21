use super::event::Event;
use crate::{
    lexer::{Lexeme, SyntaxKind},
    syntax::JsonataLanguage,
};
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub(super) struct Sink<'l, 'input> {
    builder: GreenNodeBuilder<'static>,
    lexemes: &'l [Lexeme<'input>],
    cursor: usize,
    events: Vec<Event>,
}

impl<'l, 'input> Sink<'l, 'input> {
    pub(super) fn new(lexemes: &'l [Lexeme<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            lexemes,
            cursor: 0,
            events,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
        // We need to simulate start_node_at by preprocessing the events vector
        // because Rowan's Checkpoint has private members and cannot be constructed
        // manually, so Rowan's GreenNodeBuilder::start_node_at cannot be used in this way.
        let mut reordered_events = self.events.clone();
        for (i, event) in self.events.iter().enumerate() {
            if let Event::StartNodeAt { kind, checkpoint } = event {
                reordered_events.remove(i);
                reordered_events.insert(*checkpoint, Event::StartNode { kind: *kind });
            }
        }

        for event in reordered_events {
            match event {
                Event::StartNode { kind } => {
                    self.builder.start_node(JsonataLanguage::kind_to_raw(kind))
                }
                Event::AddToken { kind, text } => self.token(kind, text),
                Event::FinishNode => self.builder.finish_node(),
                // Unreachable due to mutation above
                Event::StartNodeAt { .. } => unreachable!(),
            }

            self.eat_whitespace();
        }

        self.builder.finish()
    }

    fn eat_whitespace(&mut self) {
        while let Some(lexeme) = self.lexemes.get(self.cursor) {
            if lexeme.kind != SyntaxKind::Whitespace {
                break;
            }

            self.token(lexeme.kind, lexeme.text.into());
        }
    }

    fn token(&mut self, kind: SyntaxKind, text: String) {
        self.builder
            .token(JsonataLanguage::kind_to_raw(kind), &text);
        self.cursor += 1;
    }
}

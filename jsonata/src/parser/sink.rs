use super::event::Event;
use crate::{lexer::Lexeme, syntax::JsonataLanguage};
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub(super) struct Sink<'l, 'input> {
    builder: GreenNodeBuilder<'static>,
    lexemes: &'l [Lexeme<'input>],
    events: Vec<Event>,
}

impl<'l, 'input> Sink<'l, 'input> {
    pub(super) fn new(lexemes: &'l [Lexeme<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            lexemes,
            events,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
        // We need to simulate start_node_at by preprocessing the events vector
        // because Rowan's Checkpoint has private members and cannot be constructed
        // manually, so Rowan's GreenNodeBuilder::start_node_at cannot be used in this way.
        let mut reordered_events = self.events.clone();
        for (i, event) in self.events.into_iter().enumerate() {
            if let Event::StartNodeAt { kind, checkpoint } = event {
                reordered_events.remove(i);
                reordered_events.insert(checkpoint, Event::StartNode { kind });
            }
        }

        for event in reordered_events {
            match event {
                Event::StartNode { kind } => {
                    self.builder.start_node(JsonataLanguage::kind_to_raw(kind))
                }
                Event::AddToken { kind, text } => self
                    .builder
                    .token(JsonataLanguage::kind_to_raw(kind), &text),
                Event::FinishNode => self.builder.finish_node(),
                // Unreachable due to mutation above
                Event::StartNodeAt { kind, checkpoint } => unreachable!(),
            }
        }

        self.builder.finish()
    }
}

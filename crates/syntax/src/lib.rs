mod syntax_kind;

use num_traits::{FromPrimitive, ToPrimitive};

pub use syntax_kind::SyntaxKind;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum JsonataLanguage {}

impl rowan::Language for JsonataLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<JsonataLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<JsonataLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<JsonataLanguage>;

use std::fmt;

use syntax::SyntaxKind;
use text_size::TextRange;

#[derive(Debug, PartialEq)]
pub(crate) struct ParseError {
    pub(super) expected: Vec<SyntaxKind>,
    pub(super) found: Option<SyntaxKind>,
    pub(super) range: TextRange,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error at {}..{}: expected ",
            u32::from(self.range.start()),
            u32::from(self.range.end()),
        )?;

        let num_expected = self.expected.len();
        let is_first = |idx| idx == 0;
        let is_last = |idx| idx == num_expected - 1;

        for (idx, expected_kind) in self.expected.iter().enumerate() {
            if is_first(idx) {
                write!(f, "{}", expected_kind)?;
            } else if is_last(idx) && num_expected > 2 {
                write!(f, ", or {}", expected_kind)?;
            } else if is_last(idx) {
                write!(f, " or {}", expected_kind)?;
            } else {
                write!(f, ", {}", expected_kind)?;
            }
        }

        if let Some(found) = self.found {
            write!(f, ", but found {}", found)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range as StdRange;

    fn check(
        expected: Vec<SyntaxKind>,
        found: Option<SyntaxKind>,
        range: StdRange<u32>,
        output: &str,
    ) {
        let error = ParseError {
            expected,
            found,
            range: {
                let start = range.start.into();
                let end = range.end.into();
                TextRange::new(start, end)
            },
        };

        assert_eq!(format!("{}", error), output);
    }

    #[test]
    fn one_expected_did_find() {
        check(
            vec![SyntaxKind::Equals],
            Some(SyntaxKind::VariableIdent),
            10..20,
            "error at 10..20: expected ‘=’, but found identifier",
        )
    }

    #[test]
    fn multiple_expected_did_find() {
        check(
            vec![
                SyntaxKind::Number,
                SyntaxKind::VariableIdent,
                SyntaxKind::Minus,
                SyntaxKind::LParen,
            ],
            Some(SyntaxKind::FunctionKeyword),
            100..105,
            "error at 100..105: expected number, identifier, ‘-’, or ‘(’, but found ‘function’",
        );
    }

    #[test]
    fn two_expected_did_find() {
        check(
            vec![SyntaxKind::Plus, SyntaxKind::Minus],
            Some(SyntaxKind::Equals),
            0..1,
            "error at 0..1: expected ‘+’ or ‘-’, but found ‘=’",
        );
    }
}

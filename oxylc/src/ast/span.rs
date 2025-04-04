#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span {
    /// The index of the starting byte of the span in the source code, inclusive.
    start_byte: usize,

    /// The index of the ending byte of the span in the source code, exclusive.
    /// NOTE: Exclusive
    end_byte: usize,

    /// The line number the span starts on, inclusive.
    start_line: usize,

    /// The line number the span ends on, exclusive.
    /// NOTE: Excusive
    end_line: usize,

    /// The column number the span starts at.
    col: usize,
}

impl Span {
    /// From an `&str`, returns the string that the span represents.
    /// WARN: This will panic if the span is not valid for the string.
    pub fn slice_str<'a>(&self, s: &'a str) -> &'a str {
        s[self.start_byte..self.end_byte].as_ref()
    }
}

impl<'i, R> From<&pest::iterators::Pair<'i, R>> for Span
where
    R: pest::RuleType,
{
    fn from(pair: &pest::iterators::Pair<'i, R>) -> Self {
        let (start_line, col) = pair.line_col();
        let span = pair.as_span();
        Span {
            start_byte: span.start(),
            end_byte: span.end(),
            start_line,
            end_line: start_line + span.lines_span().count(),
            col,
        }
    }
}

impl<'i, R> From<&mut pest::iterators::Pair<'i, R>> for Span
where
    R: pest::RuleType,
{
    fn from(pair: &mut pest::iterators::Pair<'i, R>) -> Self {
        let (start_line, col) = pair.line_col();
        let span = pair.as_span();
        Span {
            start_byte: span.start(),
            end_byte: span.end(),
            start_line,
            end_line: start_line + span.lines_span().count(),
            col,
        }
    }
}

impl<'i, R> From<pest::iterators::Pair<'i, R>> for Span
where
    R: pest::RuleType,
{
    fn from(pair: pest::iterators::Pair<'i, R>) -> Self {
        let (start_line, col) = pair.line_col();
        let span = pair.as_span();
        Span {
            start_byte: span.start(),
            end_byte: span.end(),
            start_line,
            end_line: start_line + span.lines_span().count(),
            col,
        }
    }
}

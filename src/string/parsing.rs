use std::collections::HashSet;


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatchStatus {
    /// Continue speculatively. If followed by a return of [MatchStatus::End], matching fails.
    Continue = 0,
    /// Continue Successful match. If followed by a return of [MatchStatus::End], matching succeeds.
    ContinueSuccess = 1,
    /// Matching was a total success, and should end here.
    Success = 2,
    /// Matching was a total failure.
    Failure = 3,
    /// Matching ended, and success is determined by the previous [MatchStatus]:
    /// [MatchStatus::Continue] => Failure
    /// [MatchStatus::ContinueSuccess] => Success
    End = 4,
}

impl MatchStatus {
    #[must_use]
    #[inline]
    pub const fn text(self) -> &'static str {
        match self {
            Self::Continue => "Continue",
            Self::ContinueSuccess => "ContinueValid",
            Self::Success => "Success",
            Self::Failure => "Failure",
            Self::End => "End",
        }
    }
}

/// To maintain validity.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum ValidState {
    #[default]
    Init,
    Valid,
    Invalid,
}

impl ValidState {
    pub fn validate(&mut self, status: MatchStatus) {
        *self = match status {
            MatchStatus::Continue => Self::Invalid,
            MatchStatus::ContinueSuccess => Self::Valid,
            MatchStatus::Success => Self::Valid,
            MatchStatus::Failure => Self::Invalid,
            MatchStatus::End => return,
        };
    }
}

impl std::fmt::Display for MatchStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Parser<'a> {
    source: &'a str,
    start: usize,
    cursor: usize,
}

// equality is based on whether or not they have the same pointer, as well as the same start and cursor.
impl<'a> std::cmp::PartialEq<Parser<'a>> for Parser<'a> {
    fn eq(&self, other: &Parser<'a>) -> bool {
        std::ptr::eq(self.source, other.source)
        && self.start == other.start
        && self.cursor == other.cursor
    }

    fn ne(&self, other: &Parser<'a>) -> bool {
        !std::ptr::eq(self.source, other.source)
        || self.start != other.start
        || self.cursor != other.cursor
    }
}

impl<'a> std::cmp::Eq for Parser<'a> {}

impl<'a> Parser<'a> {
    #[inline]
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            cursor: 0,
        }
    }

    /// Returns None on end-of-file.
    #[inline]
    pub fn peek(&self) -> Option<char> {
        self.source[self.cursor..].chars().next()
    }

    /// Returns None on end-of-file.
    #[inline]
    pub fn next(&mut self) -> Option<char> {
        let next = self.source[self.cursor..].chars().next()?;
        self.cursor += next.len_utf8();
        Some(next)
    }

    #[inline]
    pub fn same_source(&self, other: &Self) -> bool {
        std::ptr::eq(self.source, other.source)
    }

    /// Returns false if end-of-file.
    #[inline]
    pub fn advance1(&mut self) -> bool {
        match self.source[self.cursor..].chars().next() {
            Some(next) => {
                self.cursor += next.len_utf8();
                true
            },
            None => false,
        }
    }

    /// Advance the parser by `len` characters, and returns the number of characters advanced. (not bytes!).
    #[inline]
    pub fn advance(&mut self, len: usize) -> usize {
        for index in 0..len {
            if !self.advance1() {
                return index;
            }
        }
        return len;
    }

    /// Advances the cursor by `advance` bytes.
    #[inline]
    pub fn advance_cursor(&mut self, advance: usize) -> usize {
        let next_pos = self.source.len().min(self.cursor + advance);
        let count = next_pos - self.cursor;
        self.cursor = next_pos;
        count
    }

    /// Consumes a single character if it is matched by the matcher function.
    pub fn match_char_fn<F: FnOnce(char) -> bool>(&mut self, matcher: F) -> Option<char> {
        let next = self.source[self.cursor..].chars().next()?;
        if matcher(next) {
            self.cursor += next.len_utf8();
            Some(next)
        } else {
            None
        }
    }

    #[inline]
    pub fn peek_char_fn<F: FnOnce(char) -> bool>(&self, matcher: F) -> Option<char> {
        let mut fork = self.fork();
        fork.match_char_fn(matcher)
    }

    pub fn match_str_fn<F: FnMut(char) -> MatchStatus>(&mut self, mut matcher: F) -> Option<&'a str> {
        let mut validation = ValidState::Init;
        let mut fork = self.fork();
        loop {
            let Some(peek) = fork.peek() else {
                match validation {
                    ValidState::Init => return None,
                    ValidState::Valid => {
                        self.merge(fork);
                        return Some(fork.substr_from_span());
                    },
                    ValidState::Invalid => {
                        return None;
                    },
                }
            };
            match matcher(peek) {
                cont @ (MatchStatus::Continue | MatchStatus::ContinueSuccess) => {
                    validation.validate(cont);
                    fork.cursor += peek.len_utf8();
                },
                MatchStatus::Success => {
                    fork.cursor += peek.len_utf8();
                    self.merge(fork);
                    return Some(fork.substr_from_span());
                },
                MatchStatus::Failure =>  return None,
                MatchStatus::End => {
                    match validation {
                        ValidState::Init | ValidState::Invalid => return None,
                        ValidState::Valid => {
                            self.merge(fork);
                            return Some(fork.substr_from_span());
                        },
                    }
                }
            }
        }
    }

    pub fn peek_str_fn<F: FnMut(char) -> MatchStatus>(&self, matcher: F) -> Option<&'a str> {
        let mut fork = self.fork();
        fork.match_str_fn(matcher)
    }

    /// Attempts to match the exact string, and if the match succeeds, advances the parser past the match.
    pub fn match_exact(&mut self, exact: &str) -> bool {
        if self.source[self.cursor..].starts_with(exact) {
            self.cursor += exact.len();
            true
        } else {
            false
        }
    }

    pub fn peek_exact(&self, exact: &str) -> bool {
        let mut fork = self.fork();
        fork.match_exact(exact)
    }

    /// This will return an empty string if the callback returns `true` right away.
    pub fn match_until<F: FnMut(char) -> bool>(&mut self, until: F) -> &'a str {
        // fork the parser so that we can create a substring from the resulting span.
        let mut fork = self.fork();
        fork.match_str_fn(parse_until(until));
        let result = fork.substr_from_span();
        // Merge the fork back into self to advance the cursor.
        self.merge(fork);
        result
    }

    pub fn peek_until<F: FnMut(char) -> bool>(&mut self, until: F) -> &'a str {
        let mut fork = self.fork();
        fork.match_until(until)
    }

    /// Attemps to match a single character, and if the match succeeds, advances the parser past the match.
    pub fn match_exact_char(&mut self, exact: char) -> bool {
        if let Some(peek) = self.peek()
        && peek == exact {
            self.cursor += peek.len_utf8();
            true
        } else {
            false
        }
    }

    pub fn peek_exact_char(&self, exact: char) -> bool {
        let mut fork = self.fork();
        fork.match_exact_char(exact)
    }

    pub fn fork(&self) -> Parser<'a> {
        Parser { source: self.source, cursor: self.cursor, start: self.cursor }
    }

    /// The fork must have been created from the same source, and the cursor must be at or ahead of the current cursor.
    /// If these conditions are not met, it results in a panic in debug builds.
    pub fn merge(&mut self, fork: Parser<'_>) {
        debug_assert!(std::ptr::eq(self.source, fork.source) && fork.cursor >= self.cursor);
        self.cursor = fork.cursor;
    }

    #[inline(always)]
    pub fn substr_to_cursor(&self) -> &'a str {
        &self.source[..self.cursor]
    }

    #[inline(always)]
    pub fn substr_from_span(&self) -> &'a str {
        &self.source[self.span()]
    }

    #[inline(always)]
    pub fn substr_after_cursor(&self) -> &'a str {
        &self.source[self.cursor..]
    }

    #[inline(always)]
    pub fn source(&self) -> &'a str {
        self.source
    }

    #[inline(always)]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Represents the offset (in bytes) in the underlying source string.
    #[inline(always)]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    #[inline(always)]
    pub fn at_end(&self) -> bool {
        self.cursor == self.source.len()
    }

    #[inline(always)]
    pub fn span(&self) -> std::ops::Range<usize> {
        self.start..self.cursor
    }
}

pub fn parse_while<F: FnMut(char) -> bool>(mut matcher: F) -> impl FnMut(char) -> MatchStatus {
    move |c| {
        if matcher(c) {
            MatchStatus::ContinueSuccess
        } else {
            MatchStatus::End
        }
    }
}

/// A parse function that parses until either the end of the stream or the given character is met. Does
/// not consume the match character.
#[must_use]
#[inline(always)]
pub fn parse_until<F: FnMut(char) -> bool>(mut until: F) -> impl FnMut(char) -> MatchStatus {
    move |c| {
        if until(c) {
            MatchStatus::End
        } else {
            MatchStatus::ContinueSuccess
        }
    }
}

#[must_use]
#[inline(always)]
pub fn match_char(to_match: char) -> impl Fn(char) -> MatchStatus {
    move |c| {
        if c == to_match {
            MatchStatus::Success
        } else {
            MatchStatus::Failure
        }
    }
}

#[must_use]
#[inline]
pub fn match_any_char(chars: &str) -> impl Fn(char) -> MatchStatus {
    move |c| {
        if chars.contains(c) {
            MatchStatus::Success
        } else {
            MatchStatus::Failure
        }
    }
}

#[must_use]
#[inline]
pub fn match_from_set(chars: &HashSet<char>) -> impl Fn(char) -> MatchStatus {
    move |c| {
        if chars.contains(&c) {
            MatchStatus::Success
        } else {
            MatchStatus::Failure
        }
    }
}

pub fn singleline_str_literal_matcher() -> impl FnMut(char) -> MatchStatus {
    let mut first = true;
    let mut skip1 = false;
    move |c| {
        if first {
            if c == '"' {
                first = false;
                return MatchStatus::Continue;
            } else {
                return MatchStatus::Failure;
            }
        }
        if skip1 {
            skip1 = false;
            return MatchStatus::Continue;
        }
        match c {
            '\\' => {
                skip1 = true;
                MatchStatus::Continue
            }
            '"' => MatchStatus::Success,
            _ => MatchStatus::Continue,
        }
    }
}

#[must_use]
#[inline(always)]
pub fn match_singleline_str_literal(source: &str) -> Option<&str> {
    let mut parser = Parser::new(source);
    parser.match_str_fn(singleline_str_literal_matcher())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // if peek_match_fn_test passes, that means that Parser::match_fn also works.
    #[test]
    fn parser_test() {
        let source = "  \t\t\n1234";
        let parser = Parser::new(source);
        let peeked = parser.peek_str_fn(parse_while(|c| c.is_whitespace()));
        if let Some(match_str) = peeked {
            assert_eq!(match_str, "  \t\t\n");
        } else {
            panic!("No match.");
        }
        let source = "Hello, world!";
        let parser = Parser::new(source);
        let peeked = parser.peek_str_fn(parse_until(|c| c == '!'));
        if let Some(match_str) = peeked {
            assert_eq!(match_str, "Hello, world");
        } else {
            panic!("No match!");
        }
        let parser = Parser::new("foo");
        assert!(parser.peek_str_fn(match_char('f')).is_some());
        assert!(parser.peek_str_fn(match_char('x')).is_none());
        assert!(parser.peek_exact_char('f'));
        let source = r#""Hello, \"world\"!", this is a test."#;
        if let Some(string_lit) = match_singleline_str_literal(source) {
            assert_eq!(string_lit, r#""Hello, \"world\"!""#);
        } else {
            panic!("Failed to match string literal.");
        }

        assert_eq!(match_singleline_str_literal("not a string literal"), None);
        assert_eq!(match_singleline_str_literal("\"not a complete string literal"), None);
        assert_eq!(match_singleline_str_literal("not a string literal\""), None);
    }
}
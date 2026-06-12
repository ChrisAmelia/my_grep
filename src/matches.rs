use std::fmt::Display;

/// A `Match` is the result of finding the input characters
#[derive(Debug, PartialEq)]
pub struct Match {
    pub line_number: u32,
    pub line: String
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.line_number + 1, self.line)
    }
}

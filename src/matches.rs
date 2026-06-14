use std::fmt::Display;

/// `index` is the line number, 0-based.
#[derive(Debug, PartialEq)]
pub struct Line {
    pub index: u32,
    pub content: String
}

/// A `FileMatch` is essentially a 2-tuple of a filename and a list of [Line]
/// matching the pattern.
#[derive(Debug, PartialEq)]
pub struct FileMatch {
    pub filename: String,
    pub lines: Vec<Line>,
}

impl Display for FileMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\x1b[4m{}\x1b[0m", self.filename)?;
        let spacing = " ".repeat(6);
        for line in &self.lines {
            writeln!(f, "\x1b[32m{}\x1b[0m:{}{}", line.index, spacing, line.content)?;
        }
        Ok(())
    }
}

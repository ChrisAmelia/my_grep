use std::io::{BufRead, BufReader};
use std::fs::File;

use crate::Flag;

mod grep_test;

/// A `Match` is the result of finding the input characters
#[derive(Debug, PartialEq)]
pub struct Match {
    pub line_number: u32,
    pub line: String
}

/// Returns a list of matches for the given `pattern`.
///
/// # Parameters
///
/// * `filename`: if `Some(filename)`, searches for `word` in that file.
/// * `word`: The word too look for.
///
/// # Returns
///
/// * `Ok(Vec<Match>)`: a vector of matches.
/// * `Err(&'static str)`: An error message if the file cannot be read or accessed.
pub fn grep(filename: &str, word: &str, flags: &[Flag]) -> Result<Vec<Match>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let case_sensitive = !flags
        .contains(&Flag::Insensitive);

    let matches: Vec<Match> = reader
        .lines()
        .enumerate()
        .filter_map(|(line_number, line)| {
            let line = line.ok()?;

            if (case_sensitive && line.contains(word))
                || (!case_sensitive && line.to_lowercase().contains(&word.to_lowercase())) {

                Some(Match { line_number: line_number as u32, line })
            } else {
                None
            }
        })
        .collect();

    Ok(matches)
}

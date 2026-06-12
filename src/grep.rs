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
/// * `query`: The word to look for.
/// * `flags`: A list of [Flag].
///
/// # Returns
///
/// * `Ok(Vec<Match>)`: a vector of matches.
/// * `Err(std::io::Error)`: An error message if the file cannot be read or accessed.
pub fn grep(filename: &str, query: &str, flags: &[Flag]) -> Result<Vec<Match>, std::io::Error> {
    let contents = std::fs::read_to_string(filename)?;

    let case_insensitive = flags
        .contains(&Flag::Insensitive);

    let matches = if case_insensitive {
        search_case_insensitive(&contents, query)
    } else {
        search(&contents, query)
    };


    Ok(matches)
}

/// Searches for `query` in `contents`.
///
/// Returns the list of [Match].
fn search(contents: &str, query: &str) -> Vec<Match> {
    contents
        .lines()
        .enumerate()
        .filter_map(|(line_number, line)| {
            if line.contains(query) {
                Some(Match { line_number: line_number as u32, line: line.to_string() })
            } else {
                None
            }
        })
    .collect()
}

/// Searches for `query` (insensitive) in `contents`.
///
/// Returns a list of [Match].
fn search_case_insensitive(contents: &str, query: &str) -> Vec<Match> {
    contents
        .lines()
        .enumerate()
        .filter_map(|(line_number, line)| {
            let line = line.to_lowercase();

            if line.contains(&query.to_lowercase()) {
                Some(Match { line_number: line_number as u32, line })
            } else {
                None
            }
        })
    .collect()
}

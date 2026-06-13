use std::{fs, io};

use crate::{matches::{FileMatch, Line}, Flag};

mod grep_test;

/// Returns a list of matches for the given `pattern`.
///
/// # Parameters
///
/// * `filename`: searches for `word` in that file.
/// * `query`: The word to look for.
/// * `flags`: A list of [Flag].
///
/// # Returns
///
/// * `Ok(Vec<Match>)`: a vector of matches.
/// * `Err(std::io::Error)`: An error message if the file cannot be read or accessed.
pub fn search_in_one_file(filename: &str, query: &str, flags: &[Flag]) -> Result<FileMatch, io::Error> {
    let contents = std::fs::read_to_string(filename)?;

    let case_insensitive = flags
        .contains(&Flag::Insensitive);

    let lines = if case_insensitive {
        search_case_insensitive(&contents, query)
    } else {
        search(&contents, query)
    };

    Ok(FileMatch { filename: filename.to_string(), lines })
}

pub fn search_in_all_files_in_cwd(query: &str, flags: &[Flag]) -> Result<Vec<FileMatch>, io::Error> {
    let entries = fs::read_dir(".")?;
    let filenames: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();

            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
    .collect();

    let matches: Vec<FileMatch> = filenames
        .iter()
        .map(|filename| search_in_one_file(filename, query, flags))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(matches)
}

/// Searches for `query` in `contents`.
///
/// Returns the list of [Match].
fn search(contents: &str, query: &str) -> Vec<Line> {
    contents
        .lines()
        .enumerate()
        .filter_map(|(index, content)| {
            if content.contains(query) {
                Some(Line { index: index as u32, content: content.to_string() })
            } else {
                None
            }
        })
    .collect()
}

/// Searches for `query` (insensitive) in `contents`.
///
/// Returns a list of [Match].
fn search_case_insensitive(contents: &str, query: &str) -> Vec<Line> {
    contents
        .lines()
        .enumerate()
        .filter_map(|(index, content)| {
            let content = content.to_lowercase();

            if content.contains(&query.to_lowercase()) {
                Some(Line { index: index as u32, content })
            } else {
                None
            }
        })
    .collect()
}

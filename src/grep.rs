use std::{fs, io};

use crate::{matches::{FileMatch, Line}, Flag};

mod grep_test;

/// Returns a list of matches for the given `query` in the given `filename`.
///
/// # Parameters
///
/// * `filename`: searches for `word` in that file.
/// * `query`: the pattern to look for.
/// * `flags`: a list of [Flag].
///
/// # Returns
///
/// * `Ok(Vec<FileMatch>)`: a vector of [FileMatch].
///
/// # Errors
///
/// This function will error if `std::fs::read_to_string()` fails.
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

/// Returns a list of matches for the given `query`.
///
/// This function will look in every file in the current working directory.
///
/// # Parameters
///
/// * `query`: the pattern to look for.
/// * `flags`: a list of [Flag].
///
/// # Errors
///
/// This function will return an error if `std::fs::read_dir()` fails or if `search_in_one_file()`
/// fails.
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
        .filter_map(|result| result.ok())
        .filter(|file_match| !file_match.lines.is_empty())
        .collect();

    Ok(matches)
}

/// Searches for `query` in `contents`.
///
/// Returns the list of [Line].
///
/// # Parameters
///
/// * `contents`: file's contents.
/// * `query`: the pattern to look for.
///
/// # Returns
///
/// All the lines matching `query`, result can be empty.
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
///
/// # Parameters
///
/// * `contents`: file's contents.
/// * `query`: the pattern to look for.
///
/// # Returns
///
/// All the lines matching `query` (insensitive), result can be empty.
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

use std::io;
use std::io::ErrorKind;

use crate::Flag;

mod cli_test;

/// Returns the filenames
pub fn get_filenames<'a>(args: &'a [&'a str]) -> Option<Vec<&'a str>> {
    let args = args.iter().skip(2); // skip program name and first argument (pattern)

    let filenames: Vec<&str> = args
        .filter(|arg| !arg.starts_with("-"))
        .copied()
        .collect();

    if filenames.is_empty() {
        None
    } else {
        Some(filenames)
    }
}

/// Returns the pattern.
pub fn get_pattern<'a>(args: &'a [&'a str]) -> Result<&'a str, io::Error> {
    let mut args = args.iter().skip(1);

    match args.find(|arg| !arg.starts_with("-")) {
        Some(arg) => Ok(arg),
        None      => Err(io::Error::new(ErrorKind::NotFound, "No pattern given"))
    }
}

/// Parses flags into `[Flag]`.
pub fn parse_flags(args: &[&str]) -> Vec<Flag> {
    let mut flags = Vec::new();
    let args = args.iter().skip(1);

    for arg in args {
        match *arg {
            "-i" | "--insensitive"      => flags.push(Flag::Insensitive),
            _ => continue,
        }
    }
 
    flags
}

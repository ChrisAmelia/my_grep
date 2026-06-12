use std::io;
use std::io::ErrorKind;

use crate::Flag;

mod cli_test;

/// Returns the filename.
pub fn get_filename<'a>(args: &'a [&'a str]) -> Result<&'a str, io::Error> {
    let mut args = args.iter().skip(1);

    match args.find(|arg| !arg.starts_with("-")) {
        Some(arg) => Ok(arg),
        None      => Err(io::Error::new(ErrorKind::NotFound, "No filename given"))
    }
}

/// Returns the pattern.
pub fn get_pattern<'a>(args: &'a [&'a str]) -> Result<&'a str, io::Error> {
    let mut args = args.iter().skip(1).filter(|arg| !arg.starts_with("-"));

    match args.nth(1) {
        Some(arg) => Ok(arg),
        None      => Err(io::Error::new(ErrorKind::NotFound, "No pattern given")),
    }
}

/// Parses flags into `[Flag]`.
pub fn parse_flags(args: &[&str]) -> Vec<Flag> {
    let mut flags = Vec::new();
    let args = args.iter().skip(1);

    for arg in args {
        match *arg {
            "-i" | "--insensitive"      => flags.push(Flag::Insensitive),
            "-n" | "--show-line-number" => flags.push(Flag::ShowLineNumber),
            _ => continue,
        }
    }
 
    flags
}

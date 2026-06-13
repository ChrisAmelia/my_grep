use std::env;

use my_grep::cli;
use my_grep::grep::search_in_one_file;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let args = &args.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let filename = cli::get_filename(args)?;
    let word = cli::get_pattern(args)?;
    let flags = cli::parse_flags(args);

    let matches = search_in_one_file(filename, word, &flags)?;

    println!("{matches}");

    Ok(())
}

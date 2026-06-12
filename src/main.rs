use std::env;

use my_grep::cli;
use my_grep::grep::grep;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let args = &args.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let filename = cli::get_filename(args)?;
    let word = cli::get_pattern(args)?;
    let flags = cli::parse_flags(args);

    let matches = grep(filename, word, &flags);

    matches?
        .iter()
        .for_each(|result| println!("{result}"));

    Ok(())
}

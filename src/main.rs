use std::env;

use my_grep::cli;
use my_grep::grep;
use my_grep::matches::FileMatch;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let args = &args.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let filenames = cli::get_filenames(args);
    let pattern = cli::get_pattern(args)?;
    let flags = cli::parse_flags(args);

    if let Some(filenames) = filenames {
        let matches: Vec<FileMatch> = filenames
            .iter()
            .map(|filename| grep::search_in_one_file(filename, pattern, &flags))
            .collect::<Result<Vec<_>, _>>()?;

        matches.iter().for_each(|x| println!("{x}"));
    } else {
        let matches = grep::search_in_all_files_in_cwd(pattern, &flags)?;

        matches.iter().for_each(|x| println!("{x}"));
    }

    Ok(())
}

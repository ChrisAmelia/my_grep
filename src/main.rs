use my_grep::grep;
use my_grep::matches::FileMatch;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Case-sensitive grep-like utility")]
struct Args {
    /// Search pattern
    pattern: String,

    /// Enable case-insensitive search
    #[arg(short, long)]
    insensitive: bool,

    /// Files to search
    files: Vec<String>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let files = args.files;
    let pattern = args.pattern;
    let insensitive = args.insensitive;

    if !files.is_empty() {
        let matches: Vec<FileMatch> = files
            .iter()
            .map(|filename| grep::search_in_one_file(filename, &pattern, insensitive))
            .collect::<Result<Vec<_>, _>>()?;

        matches.iter().for_each(|x| println!("{x}"));
    } else {
        let matches = grep::search_in_all_files_in_cwd(&pattern, insensitive)?;

        matches.iter().for_each(|x| println!("{x}"));
    }

    Ok(())
}

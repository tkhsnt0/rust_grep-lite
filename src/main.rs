use anyhow::Result;
use clap::{ArgGroup, Parser};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(group(ArgGroup::new("how_to_input").required(true).args(["filepath", "stdin"])))]
struct Args {
    #[arg(
        long = "pattern",
        help = "The string pattern to search for each lines."
    )]
    pattern: String,
    #[arg(long = "file", help = "Input from file(specify file path).")]
    filepath: Option<String>,
    #[arg(long = "stdin", help = "Input from stdin.")]
    stdin: bool,
}

fn process_lines<T: BufRead>(reader: T, regex: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        let contains_substring = regex.find(&line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let regex = Regex::new(&args.pattern)?;

    if args.stdin == true {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, regex);
    } else {
        let file = File::open(&args.filepath.unwrap_or("not specified".to_string()))?;
        let reader = BufReader::new(file);
        process_lines(reader, regex);
    }
    Ok(())
}

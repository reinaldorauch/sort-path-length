use std::fs;
use std::io::{stdin, stdout, Read};

use sort_path_length::sort_path_length;

use clap::Parser;

/// Simple program to sort a list of paths by its components length
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// File to get the paths of. If not defined, reads from stdin
    file: Option<String>,

    /// Inverts the sorting order
    #[arg(short, long, default_value_t = false)]
    invert: bool,
}

fn read_from_stdin() -> String {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    input
}

fn main() {
    let args = CliArgs::parse();

    let file = match args.file {
        None => read_from_stdin(),
        Some(name) => fs::read_to_string(name).unwrap(),
    };

    sort_path_length(file.as_str(), args.invert, &mut stdout());
}

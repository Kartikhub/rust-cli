use std::env;
use std::process;

use rust_cli::Arguments;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cli_args = Arguments::build(&args).unwrap_or_else(|err| {
        eprintln!("Problems parsing arguements: {err}");
        process::exit(1);
    });

    if let Err(e) = rust_cli::run(cli_args) {
        eprintln!("Unable to read the file: {e}");
        process::exit(1);
    }
}


use std::error::Error;
use std::{env, fs, process};
use twograms::*;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("usage: twograms <filename> [<max-suggestions>]");
        eprintln!(
            "       <filename>           The text file used to generate the n-grams structure"
        );
        eprintln!("       [<max-suggestions>]  Optional number, how many suggestions to keep for each word");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filename = config.filename;
    let text = fs::read_to_string(filename)?;
    let ngrams = generate_ngrams(&text, config.keep);
    let json = serde_json::to_string(&ngrams).unwrap();
    println!("{}", json);
    Ok(())
}

struct Config {
    filename: String,
    keep: usize,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // the program name is the first argument
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let keep = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => 100000,
        };

        Ok(Config { filename, keep })
    }
}

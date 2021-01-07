use std::error::Error;
use std::{env, fs, process};
mod lib;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        println!("usage: twograms <filename> [<max-suggestions>]");
        println!(
            "       <filename>           The text file used to generate the n-grams structure"
        );
        println!("       [<max-suggestions>]  An optional number between 1-5");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filename = config.filename;
    let text = fs::read_to_string(filename)?;
    let ngrams = lib::generate_ngrams(&text, config.keep);
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
            Some(arg) => match arg.as_str() {
                "1" => 1,
                "2" => 2,
                "3" => 3,
                "4" => 4,
                "5" => 5,
                _ => 0,
            },
            None => 0,
        };

        Ok(Config { filename, keep })
    }
}

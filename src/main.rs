use std::{env, fs};
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Need to pass (only) filename");
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let json = lib::to_json(&contents);
    println!("{}", json);
}

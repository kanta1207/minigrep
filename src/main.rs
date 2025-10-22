use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = generate_config(&args);

    println!("Searching for {} in {}", config.query, config.filename);

    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("ファイルが開けませんでした");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("ファイルが読み込めませんでした");

    println!("File contents: {}", contents);
}

fn generate_config(args: &[String]) -> Config {
    Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    })
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
         return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
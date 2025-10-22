extern crate minigrep;

use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::generate_config(&args);
    println!("Searching for {} in {}", config.query, config.filename);

    println!("In file {}", config.filename);

    
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}


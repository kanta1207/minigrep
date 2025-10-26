extern crate minigrep;

use std::process;


fn main() {
    let config = minigrep::generate_config(); 

    
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}


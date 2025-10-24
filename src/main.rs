extern crate minigrep;

use std::process;


fn main() {
    let args = minigrep::get_args();

    let config = minigrep::generate_config(&args); 

    
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}


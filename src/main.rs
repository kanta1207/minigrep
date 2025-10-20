use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {} in {}", query, file_path);

    println!("In file {}", file_path);

    let mut f = File::open(file_path).expect("ファイルが開けませんでした");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("ファイルが読み込めませんでした");

    println!("File contents: {}", contents);
}



mod cfront;
//mod defs;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Take in file.
    let file_contents = fs::read_to_string(filename).expect("Could not read file.");
    print!("=====Contents of file=====\n{}", file_contents);
    print!("=====End of file contents=====\n");
    println!();

    // Convert to tokens
    let token_vec = cfront::lexer(&file_contents).unwrap();
    cfront::print_tokens(&token_vec);
}

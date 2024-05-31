mod lexer;
//mod parser;

use std::env;
use std::fs;
use lexer::lexing::tokenize;
//use parser::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <test_file>", args[0]);
        return;
    };
    let filename = &args[1];
    let input = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };
    let input = input.replace("\r\n", "\n");
    let input_bytes = input.as_bytes();
    println!("This is the input:\n\n{}\n", input);
    println!("This is the input as bytes:\n\n{:?}", input_bytes);
    println!("This is the input as tokens:\n{:?}",tokenize(&input.as_str()));
    //parse(test_input);
}
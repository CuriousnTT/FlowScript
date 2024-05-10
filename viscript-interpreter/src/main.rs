mod lexer;
//mod parser;

use std::env;
use std::fs;
use lexer::lexing::lex;
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
    let input_bytes = input.as_bytes();
    println!("Hello, this is input: {}", input);
    println!("Hello, this is input as bytes: {:?}", input_bytes);
    println!("Is there a float in there? {:?}",lex(input_bytes));
    //parse(test_input);
}

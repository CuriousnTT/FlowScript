mod lexer;
pub mod parser;

use std::fs::File;
use lexer::lexing::lex;
use parser::parse;

fn main() {
    
    let test_input: String = "let Test String here! 1.09".to_string();
    let input_bytes = test_input.as_bytes();
    println!("{}", test_input);
    println!("{:?}", input_bytes);
    println!("{:?}",lex(input_bytes));
    parse(test_input);
}

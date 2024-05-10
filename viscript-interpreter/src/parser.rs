mod float_parser;

use super::lexer::lexing::lex;
use float_parser::handle_float;

pub fn parse(input: String) {
    let input_bytes = input.as_bytes();
    if lex(input) {
        let float_input = input;
        handle_float(input);
    } else {
        println!("No float in input: {}", input);
    }
}
mod lexer;
mod tokens;
mod nodes;
mod parser;
use std::fs;
use crate::lexer::Lexer;
use crate::parser::Parser;
fn main() {

    let contents = fs::read_to_string("interpret.math")
        .expect("Something went wrong reading the file");

    let mut test_lexer = Lexer::new(contents.to_string());
    test_lexer.generate_tokens();
    let mut test_parser = Parser::new(test_lexer.tokens);
    test_parser.parse();

}
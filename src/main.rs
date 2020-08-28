mod tokenizer;
mod parser;
mod ast;
mod gen;

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_contents = fs::read_to_string(&args[1])
        .expect("Failed to read from file");

    let tokens = tokenizer::generate_tokens(&file_contents[..]);

    // print_tokens(&tokens); //Debug

    let ast = parser::parse_tokens(&tokens).unwrap();
    gen::write_assembly(&ast);
}

fn print_tokens(tokens: &[tokenizer::Token]) {
    for token in tokens.iter() {
        println!("Type: {:?}, value: {}", token.token_type, token.value);
    }
}

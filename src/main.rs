mod tokenizer;
mod parser;
mod ast;

fn main() {
    let tokens = tokenizer::generate_tokens("examples/sample.c");
    for token in tokens.iter() {
        println!("Type: {:?}, value: {}", token.token_type, token.value);
    }
}

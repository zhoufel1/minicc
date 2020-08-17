mod tokenizer;
mod parser;

fn main() {
    let tokens = tokenizer::generate_tokens("examples/sample.c");
    for token in tokens.iter() {
        println!("Type: {:?}, value: {}", token.token_type, token.value);
    }
    if let Some(v) = parser::parse_tokens(&tokens) {
        println!("{}, {}", v.func.name, v.func.body.return_value.val);
    }
}

mod tokenizer;

fn main() {
    let tokens = tokenizer::generate_tokens("examples/sample.c");
    for token in tokens.iter() {
        println!("{}", token.content);
    }
}

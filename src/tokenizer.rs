use std::fs;

pub fn generate_tokens(file: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let contents = fs::read_to_string(file).expect("Failed to read file");

    for line in contents.lines() {
        let mut char_buffer = String::new();

        for ch in line.trim().chars() {
            match ch {
                ' ' => _flush(&mut char_buffer, &mut tokens),
                '(' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push("(".to_string());
                },
                ')' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(")".to_string());
                },
                '{' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push("{".to_string());
                },
                '}' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push("}".to_string());
                },
                ';' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(";".to_string());
                }
                _ =>  {
                    char_buffer.push(ch);
                }
            };
        }
        _flush(&mut char_buffer, &mut tokens);
    }

    tokens
}

fn _flush(chars: &mut String, tokens: &mut Vec<String>) {
    if !chars.is_empty() {
        if let Ok(_) = chars.parse::<u128>() {
            tokens.push(chars.to_string());
        } else {
            tokens.push(chars.to_string());
        }
        chars.clear();
    }
}
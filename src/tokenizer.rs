use std::fs;

const KEYWORDS: [&str; 1] = [
    "return"
];

const TYPES: [&str; 1] = [
    "int"
];

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token {
            token_type,
            value,
        }
    }
}

#[derive(Debug)]
pub enum TokenType {
    Brace,
    Paren,
    Semicolon,
    Identifier,
    Constant,
    Keyword,
    Decl,
}

pub fn generate_tokens(file: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let contents = fs::read_to_string(file).expect("Failed to read file");

    for line in contents.lines() {
        let mut char_buffer = String::new();

        for ch in line.trim().chars() {
            match ch {
                ' ' => _flush(&mut char_buffer, &mut tokens),
                '(' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Paren, "(".to_string()));
                },
                ')' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Paren, ")".to_string()));
                },
                '{' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Brace, "{".to_string()));
                },
                '}' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Brace, "}".to_string()));
                },
                ';' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Semicolon, ";".to_string()));
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

fn _flush(chars: &mut String, tokens: &mut Vec<Token>) {
    if !chars.is_empty() {
        if let Ok(_) = chars.parse::<u128>() {
            tokens.push(Token::new(TokenType::Constant, chars.to_string()));
        } else if TYPES.contains(&&chars[..]) {
            tokens.push(Token::new(TokenType::Decl, chars.to_string()));
        } else if KEYWORDS.contains(&&chars[..]) {
            tokens.push(Token::new(TokenType::Keyword, chars.to_string()));
        } else {
            tokens.push(Token::new(TokenType::Identifier, chars.to_string()));
        }
        chars.clear();
    }
}
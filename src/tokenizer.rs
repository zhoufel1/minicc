use std::fs;

const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z'
];

const ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y',
    'Z'
];

const ASCII_NUMERIC: [char; 10] = [
    '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

pub struct Token {
    pub token_type: TokenType,
    pub content: String,
}

impl Token {
    pub fn new(token_type: TokenType, content: String) -> Token {
        Token {
            token_type,
            content,
        }
    }
}

pub enum TokenType {
    Brace,
    Parenthesis,
    Semicolon,
    Keyword,
    Identifier,
    Literal,
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
                    tokens.push(Token::new(TokenType::Parenthesis, "(".to_string()));
                },
                ')' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Parenthesis, ")".to_string()));
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
            tokens.push(Token::new(TokenType::Literal, chars.to_string()));
        } else {
            tokens.push(Token::new(TokenType::Identifier, chars.to_string()));
        }
        chars.clear();
    }
}
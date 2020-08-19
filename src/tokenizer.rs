
pub const KEYWORDS: [&str; 1] = [
    "return"
];

pub const TYPES: [&str; 1] = [
    "int"
];

pub const OPERATORS: [&str; 3] = [
    "-", "~", "!"
];

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: &str) -> Token {
        Token {
            token_type,
            value: value.to_string(),
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
    Op,
}

pub fn generate_tokens(contents: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for line in contents.lines() {
        let mut char_buffer = String::new();

        for ch in line.trim().chars() {
            match ch {
                ' ' => _flush(&mut char_buffer, &mut tokens),
                '(' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Paren, "("));
                },
                ')' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Paren, ")"));
                },
                '{' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Brace, "{"));
                },
                '}' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Brace, "}"));
                },
                ';' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Semicolon, ";"));
                },
                '-' =>  {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Op, "-"));
                },
                '~' =>  {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Op, "~"));
                },
                '!' =>  {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Op, "!"));
                },
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
        if let Ok(_) = chars.parse::<u8>() {
            tokens.push(Token::new(TokenType::Constant, chars));
        } else if TYPES.contains(&&chars[..]) {
            tokens.push(Token::new(TokenType::Decl, chars));
        } else if KEYWORDS.contains(&&chars[..]) {
            tokens.push(Token::new(TokenType::Keyword, chars));
        } else {
            tokens.push(Token::new(TokenType::Identifier, chars));
        }
        chars.clear();
    }
}
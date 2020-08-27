use regex::Regex;

pub const KEYWORDS: [&str; 2] = [
    "return", "int"
];

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: &str) -> Token {
        Token {
            token_type,
            value: value.to_string()
        }
    }
}

#[derive(Debug)]
pub enum TokenType {
    Brace,
    Paren,
    Semicolon,
    Identifier,
    Literal,
    Keyword,
    Op,
    Invalid
}

pub fn generate_tokens(contents: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut line_pos: u32 = 1;

    for line in contents.lines() {
        let mut char_buffer = String::new();
        let mut col_pos = 1;

        for ch in line.trim().chars() {
            let ch_s = &ch.to_string()[..];
            match ch {
                ' ' => _flush(&mut char_buffer, &mut tokens),
                '('
                | ')' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Paren, ch_s));
                },
                '{'
                | '}' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Brace, ch_s));
                },
                ';' => {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Semicolon, ch_s));
                },
                '-'
                | '~'
                | '!' =>  {
                    _flush(&mut char_buffer, &mut tokens);
                    tokens.push(Token::new(TokenType::Op, ch_s));
                },
                _ =>  {
                    char_buffer.push(ch);
                }
            };
            col_pos += 1;
        }
        _flush(&mut char_buffer, &mut tokens);
        line_pos += 1;
    }
    tokens
}

fn _flush(chars: &mut String, tokens: &mut Vec<Token>) {
    if !chars.is_empty() {
        if let Some(_) = Regex::new(r"^[0-9]+$").unwrap().captures(&chars) {
            tokens.push(Token::new(TokenType::Literal, chars))
        } else if KEYWORDS.contains(&&chars[..]) {
            tokens.push(Token::new(TokenType::Keyword, chars));
        } else if let Some(_) = Regex::new(r"^[a-zA-z_]\w+$").unwrap().captures(&chars) {
            tokens.push(Token::new(TokenType::Identifier, chars));
        } else {
            tokens.push(Token::new(TokenType::Invalid, chars));
        }
        chars.clear();
    }
}
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
        for word in line.trim().trim_end_matches(';').split(" ") {
            let word = word.to_string();

            if word == "{" || word == "}" {
                // Check brace
                tokens.push(Token::new(TokenType::Brace, word));
            } else if word == "return" {
                // Check Return keywordk (refactor later)
                tokens.push(Token::new(TokenType::Keyword, word));
            } else if word.contains("(") && word.contains(")") {
                // Check function name and paren (refactor)
                let open_pos = word.find("(").unwrap();
                let close_pos = word.find(")").unwrap();

                if open_pos < close_pos && open_pos != 0 && close_pos == word.len() - 1 {
                    tokens.push(Token::new(TokenType::Identifier, word[..open_pos].to_string()));
                    tokens.push(Token::new(TokenType::Parenthesis, "(".to_string()));
                    tokens.push(Token::new(TokenType::Parenthesis, ")".to_string()));
                }
            }  else if let Ok(_) = word.parse::<u128>() {
                // Check numeric literal (int)
                tokens.push(Token::new(TokenType::Literal, word));
            } else if word != "" && word.chars().all(|c| ASCII_LOWER.contains(&c) || ASCII_UPPER.contains(&c) || ASCII_NUMERIC.contains(&c)) {
                // Check for identifier
                tokens.push(Token::new(TokenType::Identifier, word));
            }
        }
        if line.contains(";") {
            tokens.push(Token::new(TokenType::Semicolon, ";".to_string()));
        }
    }
    tokens
}
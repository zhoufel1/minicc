use crate::tokenizer::{Token, TokenType};

pub struct Program {
    pub main: Function,
}

pub struct Function {
    pub name: String,
    pub body: Return,
}

pub struct Return {
    pub return_value: Const,
}

pub struct Const {
    pub val: u32,
}

pub fn parse_tokens(tokens: &Vec<Token>) -> Option<Program> {
    let main = parse_function(&tokens)?;
    Some(Program {
        main,
    })
}

pub fn parse_function(tokens: &[Token]) -> Option<Function> {
    let mut tok = tokens.iter();
    let mut next = tok.next()?;
    match next.token_type {
        TokenType::Decl => (),
        _ => panic!("Missing type declaration"),
    }
    next = tok.next()?;
    let name = match next.token_type {
        TokenType::Identifier => next.value.to_string(),
        _ => panic!("Missing function identifier"),
    };
    next = tok.next()?;
    if next.value != "(" {
        panic!("Mismatch parenthesis");
    }
    next = tok.next()?;
    if next.value != ")" {
        panic!("Mismatch parenthesis");
    }
    next = tok.next()?;
    if next.value != "{" {
        panic!("Mismatch brace");
    }
    let statement = parse_statement(&tokens[5..]).unwrap();
    let func = Function {
        name,
        body: statement,
    };
    next = tok.last()?;
    if next.value != "}" {
        panic!("Mismatch brace");
    }
    Some(func)
}

pub fn parse_statement(tokens: &[Token]) -> Option<Return> {
    let mut tok = tokens.iter();
    let mut next = tok.next()?;
    if next.value != "return" {
        panic!("Missing return value");
    }
    next = tok.next()?;
    match next.token_type {
        TokenType::Constant => (),
        _ => panic!("Invalid syntax after return"),
    }
    let expr = parse_expr(&tokens[1..])?;
    let statement = Return { return_value: expr };
    next = tok.next()?;
    match next.token_type {
        TokenType::Semicolon => (),
        _ => panic!("Missing semicolon"),
    }
    Some(statement)
}

pub fn parse_expr(tokens: &[Token]) -> Option<Const> {
    if let Some(tok) = tokens.iter().next() {
        // check if expression is value
        match tok.token_type {
            TokenType::Constant => {
                let expr = Const {
                    val: tok.value.parse::<u32>().unwrap(),
                };
                Some(expr)
            }
            _ => None,
        }
    } else {
        None
    }
}
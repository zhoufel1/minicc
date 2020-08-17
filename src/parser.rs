use crate::tokenizer::{Token, TokenType};

pub struct Program {
    pub func: Function,
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
    let main = parse_function(&tokens).unwrap();
    Some(Program {
        func: main,
    })
}

pub fn parse_function(tokens: &[Token]) -> Option<Function> {
    let mut tok = tokens.iter();
    let mut next = tok.next().unwrap();
    match next.token_type {
        TokenType::Decl => (),
        _ => panic!(),
    }
    next = tok.next().unwrap();
    let name = match next.token_type {
        TokenType::Identifier => next.value.to_string(),
        _ => panic!(),
    };
    next = tok.next().unwrap();
    if next.value != "(" {
        panic!();
    }
    next = tok.next().unwrap();
    if next.value != ")" {
        panic!();
    }
    next = tok.next().unwrap();
    if next.value != "{" {
        panic!();
    }
    let statement = parse_statement(&tokens[5..]).unwrap();
    let func = Function {
        name,
        body: statement,
    };
    next = tok.last().unwrap();
    if next.value != "}" {
        panic!();
    }
    Some(func)
}

pub fn parse_statement(tokens: &[Token]) -> Option<Return> {
    let mut tok = tokens.iter();
    let mut next = tok.next().unwrap();
    if next.value != "return" {
        panic!();
    }
    next = tok.next().unwrap();
    match next.token_type {
        TokenType::Constant => (),
        _ => panic!(),
    }
    let expr = parse_expr(&tokens[1..]).unwrap();
    let statement = Return { return_value: expr };
    next = tok.next().unwrap();
    match next.token_type {
        TokenType::Semicolon => (),
        _ => panic!(),
    }
    Some(statement)
}

pub fn parse_expr(tokens: &[Token]) -> Option<Const> {
    if let Some(t) = tokens.iter().next() {
        let val = t.value.parse::<u32>().unwrap();
        match t.token_type {
            TokenType::Constant => Some(Const { val }),
            _ => None,
        }
    } else {
        None
    }
}
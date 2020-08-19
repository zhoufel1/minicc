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
    let mut tokens = tokens.iter();
    let main = parse_function(&mut tokens)?;
    Some(Program {
        main,
    })
}

pub fn parse_function<'a, I>(tokens: &mut I) -> Option<Function>
where
    I:Iterator<Item = &'a Token>
{
    let mut tok = tokens;
    let mut next = tok.next().unwrap();
    match next.token_type {
        TokenType::Decl => (),
        _ => panic!("Missing type declaration"),
    }
    next = tok.next().unwrap();
    let name = match next.token_type {
        TokenType::Identifier => next.value.to_string(),
        _ => panic!("Missing function identifier"),
    };
    next = tok.next().unwrap();
    if next.value != "(" {
        panic!("Mismatch parenthesis");
    }
    next = tok.next().unwrap();
    if next.value != ")" {
        panic!("Mismatch parenthesis");
    }
    next = tok.next().unwrap();
    if next.value != "{" {
        panic!("Mismatch brace");
    }
    let statement = parse_statement(&mut tok).unwrap();
    let func = Function {
        name,
        body: statement,
    };
    next = tok.next().unwrap();
    if next.value != "}" {
        panic!("Mismatch brace");
    }
    Some(func)
}

pub fn parse_statement<'a, I>(mut tokens: &mut I) -> Option<Return>
where
    I:Iterator<Item = &'a Token>
{
    let mut next = tokens.next()?;
    if next.value != "return" {
        panic!("Missing return value");
    }
    let expr = parse_expr(&mut tokens).unwrap();
    let statement = Return { return_value: expr };
    next = tokens.next()?;
    match next.token_type {
        TokenType::Semicolon => (),
        _ => panic!("Missing semicolon"),
    }
    Some(statement)
}

pub fn parse_expr<'a, I>(tokens: &mut I) -> Option<Const>
where
    I:Iterator<Item = &'a Token>
{
    let next = tokens.next()?;
    match next.token_type {
        TokenType::Constant => {
            let expr = Const {
                val: next.value.parse::<u32>().unwrap(),
            };
            return Some(expr)
        },
        _ => return None,
    }
}
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
    let program = Program {
        main: parse_function(&mut tokens)?,
    };
    Some(program)
}

pub fn parse_function<'a, I>(mut tokens: &mut I) -> Option<Function>
where
    I:Iterator<Item = &'a Token>
{
    // only checks for return statement
    let mut tok = tokens.next()?;
    match tok.token_type {
        TokenType::Decl => (),
        _ => panic!("Missing type declaration"),
    }

    tok = tokens.next()?;
    let name = match tok.token_type {
        TokenType::Identifier => tok.value.to_string(),
        _ => panic!("Missing function identifier"),
    };

    if tokens.next()?.value != "(" {
        panic!("Mismatch parenthesis");
    }
    if tokens.next()?.value != ")" {
        panic!("Mismatch parenthesis");
    }
    if tokens.next()?.value != "{" {
        panic!("Mismatch brace");
    }

    let body = parse_statement(&mut tokens)?;
    let func = Function {
        name,
        body,
    };

    if tokens.next()?.value != "}" {
        panic!("Mismatch brace");
    }
    Some(func)
}

pub fn parse_statement<'a, I>(mut tokens: &mut I) -> Option<Return>
where
    I:Iterator<Item = &'a Token>
{
    // only parses return statement
    let mut tok = tokens.next()?;
    if tok.value != "return" {
        panic!("Missing return value");
    }

    let expr = parse_expr(&mut tokens)?;
    let statement = Return { return_value: expr };

    tok = tokens.next()?;
    match tok.token_type {
        TokenType::Semicolon => (),
        _ => panic!("Missing semicolon"),
    }
    Some(statement)
}

pub fn parse_expr<'a, I>(tokens: &mut I) -> Option<Const>
where
    I:Iterator<Item = &'a Token>
{
    // only checks for single int expression
    let tok = tokens.next()?;
    match tok.token_type {
        TokenType::Constant => {
            let expr = Const {
                val: tok.value.parse::<u32>().unwrap(),
            };
            return Some(expr)
        },
        _ => return None,
    }
}
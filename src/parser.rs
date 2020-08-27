use crate::tokenizer::{Token, TokenType};
use crate::ast::{
    Program,
    Function,
    Return,
    Expr,
    UnaryOp,
    Int,
};

pub fn parse_tokens(tokens: &Vec<Token>) -> Option<Program> {
    let mut tokens = tokens.iter();
    let program = Program {
        main: parse_function(&mut tokens)?
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
        TokenType::Keyword => (),
        _ => panic!("Missing type declaration")
    }

    tok = tokens.next()?;
    let name = match tok.token_type {
        TokenType::Identifier => tok.value.to_string(),
        _ => panic!("Missing function identifier")
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
        body
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
        _ => panic!("Missing semicolon")
    }
    Some(statement)
}

pub fn parse_expr<'a, I>(tokens: &mut I) -> Option<Expr>
where
    I:Iterator<Item = &'a Token>
{
    // only checks for single int expression
    let tok = tokens.next()?;
    match tok.token_type {
        TokenType::Literal => {
            let expr = Expr::Const {
                int: Int {
                    val: tok.value.parse::<u32>().unwrap()
                }
            };
            Some(expr)
        },
        TokenType::Op => {
            let inner = parse_expr(tokens)?;
            let expr = Expr::Expression {
                op: UnaryOp {
                    val: tok.value.to_string()
                },
                expr: Box::new(inner)
            };
            Some(expr)
        }
        _ => None
    }
}
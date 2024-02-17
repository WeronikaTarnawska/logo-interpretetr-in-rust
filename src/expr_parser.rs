use crate::{lexer::Token, parser::Expr};
use std::collections::VecDeque;

pub fn parse(tokens: &mut VecDeque<Token>) -> Box<Expr> {
    parse_eq(tokens)
}

fn parse_eq(tokens: &mut VecDeque<Token>) -> Box<Expr> {
    let mut left_operand = parse_addition(tokens);
    loop {
        match tokens.front() {
            Some(&Token::Lt) => {
                tokens.pop_front();
                let right_operand = parse_addition(tokens);
                left_operand = Box::new(Expr::Lt(left_operand, right_operand));
            }
            Some(&Token::Eq) => {
                tokens.pop_front();
                let right_operand = parse_addition(tokens);
                left_operand = Box::new(Expr::Eq(left_operand, right_operand));
            }
            _ => break,
        }
    }
    left_operand
}

fn parse_addition(tokens: &mut VecDeque<Token>) -> Box<Expr> {
    let mut left_operand = parse_multiplication(tokens);
    loop {
        match tokens.front() {
            Some(&Token::Add) => {
                tokens.pop_front();
                let right_operand = parse_multiplication(tokens);
                left_operand = Box::new(Expr::Add(left_operand, right_operand));
            }
            Some(&Token::Sub) => {
                tokens.pop_front();
                let right_operand = parse_multiplication(tokens);
                left_operand = Box::new(Expr::Sub(left_operand, right_operand));
            }
            _ => break,
        }
    }

    left_operand
}

fn parse_multiplication(tokens: &mut VecDeque<Token>) -> Box<Expr> {
    let mut left_operand = parse_operand(tokens);
    loop {
        match tokens.front() {
            Some(&Token::Mul) => {
                tokens.pop_front();
                let right_operand = parse_operand(tokens);
                left_operand = Box::new(Expr::Mul(left_operand, right_operand));
            }
            Some(&Token::Div) => {
                tokens.pop_front();
                let right_operand = parse_operand(tokens);
                left_operand = Box::new(Expr::Div(left_operand, right_operand));
            }
            _ => break,
        }
    }

    left_operand
}

fn parse_operand(tokens: &mut VecDeque<Token>) -> Box<Expr> {
    match tokens.pop_front() {
        Some(Token::Number(Some(num))) => Box::new(Expr::Number(num)),
        Some(Token::Variable(name)) => Box::new(Expr::Variable(name)),
        Some(Token::LParen) => {
            let result = parse_addition(tokens);
            assert!(tokens.pop_front() == Some(Token::RParen), "Missing ')'");
            result
        }
        Some(Token::RParen) => panic!("Unexpected ')' without '('"),
        Some(Token::Sub) => {
            let right = parse_operand(tokens);
            return Box::new(Expr::Minus(right));
        }
        Some(Token::Random) => {
            let right = parse_operand(tokens);
            return Box::new(Expr::Rand(right));
        }
        Some(Token::Pick) => {
            let from = parse_list(tokens);
            Box::new(Expr::Pick(from))
        }
        Some(Token::Red)    => Box::new(Expr::Color("red".to_string())),
        Some(Token::Orange) => Box::new(Expr::Color("orange".to_string())),
        Some(Token::Yellow) => Box::new(Expr::Color("yellow".to_string())),
        Some(Token::Green)  => Box::new(Expr::Color("green".to_string())),
        Some(Token::Blue)   => Box::new(Expr::Color("blue".to_string())),
        Some(Token::Violet) => Box::new(Expr::Color("violet".to_string())),
        Some(Token::Black)  => Box::new(Expr::Color("black".to_string())),
        _ => panic!("Invalid expression"),
    }
}

fn parse_list(tokens: &mut VecDeque<Token>) -> VecDeque<Expr> {
    let mut exprs = VecDeque::new();
    if let Some(Token::LBracket) = tokens.pop_front() {
        while let Some(token) = tokens.front() {
            match token {
                Token::Red
                | Token::Orange
                | Token::Yellow
                | Token::Green
                | Token::Blue
                | Token::Violet
                | Token::Black
                | Token::Number(_)
                | Token::Variable(_)
                | Token::LParen
                | Token::RParen
                | Token::Sub
                | Token::Random
                | Token::Pick
                | Token::Mul
                | Token::Div
                | Token::Add
                | Token::Eq
                | Token::Lt => {
                    let expr = parse(tokens);
                    exprs.push_back(*expr);
                }
                Token::RBracket => break,
                _ => panic!("parse list invalid expr"),
            }
        }
    }
    if let Some(Token::RBracket) = tokens.pop_front() {
        exprs
    } else {
        panic!("expr list: no closing ']'")
    }
}

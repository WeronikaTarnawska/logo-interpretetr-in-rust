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
            // Some(&Token::Leq) => {
            //     tokens.pop_front();
            //     let right_operand = parse_addition(tokens);
            //     left_operand = Box::new(Expr::Leq(left_operand, right_operand));
            // }
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
                tokens.pop_front(); // Consume the '+'
                let right_operand = parse_multiplication(tokens);
                left_operand = Box::new(Expr::Add(left_operand, right_operand));
            }
            Some(&Token::Sub) => {
                tokens.pop_front(); // Consume the '-'
                let right_operand = parse_multiplication(tokens);
                left_operand = Box::new(Expr::Sub(left_operand, right_operand));
            }
            _ => break,
        }
    }
    // while let Some(&Token::Add) = tokens.front() {
    //     tokens.pop_front(); // Consume the '+'
    //     let right_operand = parse_multiplication(tokens);
    //     left_operand = Box::new(Expr::Add(left_operand, right_operand));
    // }

    left_operand
}

fn parse_multiplication(tokens: &mut VecDeque<Token>) -> Box<Expr> {
    let mut left_operand = parse_operand(tokens);
    loop {
      match tokens.front() {
          Some(&Token::Mul) => {
            tokens.pop_front(); // Consume the '*'
            let right_operand = parse_operand(tokens);
            left_operand = Box::new(Expr::Mul(left_operand, right_operand));
          }
          Some(&Token::Div) => {
            tokens.pop_front(); // Consume the '/'
            let right_operand = parse_operand(tokens);
            left_operand = Box::new(Expr::Div(left_operand, right_operand));
          }
          _ => break,
      }
  }
    // while let Some(&Token::Mul) = tokens.front() {
    //     tokens.pop_front(); // Consume the '*'
    //     let right_operand = parse_operand(tokens);
    //     left_operand = Box::new(Expr::Mul(left_operand, right_operand));
    // }

    left_operand
}

fn parse_operand(tokens: &mut VecDeque<Token>) -> Box<Expr> {
    match tokens.pop_front() {
        Some(Token::Number(Some(num))) => Box::new(Expr::Number(num)),
        Some(Token::Variable(name)) => Box::new(Expr::Variable(name)),
        Some(Token::LParen) => {
            let result = parse_addition(tokens);
            assert!(tokens.pop_front() == Some(Token::RParen), "Missing ')'"); // Consume the ')'
            result
        }
        Some(Token::RParen) => panic!("Unexpected ')' without '('"),
        Some(Token::Sub) => {
          let right = parse_operand(tokens);
          return Box::new(Expr::Minus(right));
        }
        _ => panic!("Invalid expression"),
    }
}

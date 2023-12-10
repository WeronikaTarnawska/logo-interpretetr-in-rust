
use crate::{lexer::Token, parser::Expr};
use std::collections::VecDeque;

pub fn parse_addition(tokens: &mut VecDeque<Token>) -> Box<Expr> {
  let mut left_operand = parse_multiplication(tokens);

  while let Some(&Token::Add) = tokens.front() {
      tokens.pop_front(); // Consume the '+'
      let right_operand = parse_multiplication(tokens);
      left_operand = Box::new(Expr::Add(left_operand, right_operand));
  }

  left_operand
}

fn parse_multiplication(tokens: &mut VecDeque<Token>) -> Box<Expr> {
  let mut left_operand = parse_operand(tokens);

  while let Some(&Token::Mul) = tokens.front() {
      tokens.pop_front(); // Consume the '*'
      let right_operand = parse_operand(tokens);
      left_operand = Box::new(Expr::Mul(left_operand, right_operand));
  }

  left_operand
}

fn parse_operand(tokens: &mut VecDeque<Token>) -> Box<Expr> {
  match tokens.pop_front() {
      Some(Token::Number(Some(num))) => Box::new(Expr::Number(num)),
      Some(Token::LParen) => {
          let result = parse_addition(tokens);
          assert!(tokens.pop_front()== Some(Token::RParen), "Missing ')'"); // Consume the ')'
          result
      }
      Some(Token::RParen) => panic!("Unexpected ')' without '('"),
      _ => panic!("Invalid expression"),
  }
}



use crate::expr_parser;
use crate::lexer::Token;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Variable(String),
    Number(f32),
    Mul(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Forward(Expr),
    Backward(Expr),
    Right(Expr),
    Left(Expr),
    Show(Expr),
    Repeat(Expr, VecDeque<Command>),
    FunctionDeclaration(String, Vec<String>, VecDeque<Command>),
    FunctionCall(String, Vec<Expr>),
    // List(Vec<Expr>),
}

pub fn parse(tokens: &mut VecDeque<Token>) -> VecDeque<Command> {
    let mut commands = VecDeque::new();

    while let Some(token) = tokens.pop_front() {
        match token {
            Token::Repeat => {
                let iters = parse_expr(tokens);
                let body = parse_block(tokens);
                commands.push_back(Command::Repeat(iters, body))
            }

            Token::To => {
                let name = parse_name(tokens);
                let args = parse_args(tokens);
                let body = parse_block(tokens);
                commands.push_back(Command::FunctionDeclaration(name, args, body));
            }

            Token::Function(name) => {
                let args = parse_expr_seq(tokens);
                commands.push_back(Command::FunctionCall(name, args))
            }

            Token::Forward | Token::Backward | Token::Right | Token::Left | Token::Show => {
                let expr = parse_expr(tokens);
                commands.push_back(match token {
                    Token::Forward => Command::Forward(expr),
                    Token::Backward => Command::Backward(expr),
                    Token::Right => Command::Right(expr),
                    Token::Left => Command::Left(expr),
                    Token::Show => Command::Show(expr),
                    _ => unreachable!(),
                });
            }
            Token::RBracket | Token::End => {
                return commands;
            }
            _ => {
                // TODO
            }
        }
    }
    commands
}

fn parse_expr_seq(tokens: &mut VecDeque<Token>) -> Vec<Expr> {
    let mut args: Vec<Expr> = vec![];
    loop {
        match tokens.front() {
            Some(Token::Number(Some(_))) | Some(Token::Variable(_)) | Some(Token::LParen) => {
                let expr = parse_expr(tokens);
                args.push(expr);
            }
            Some(Token::Backward)
            | Some(Token::Forward)
            | Some(Token::Function(_))
            | Some(Token::Left)
            | Some(Token::Repeat)
            | Some(Token::Right)
            | Some(Token::Show)
            | Some(Token::To) => {
                break;
            }
            _ => panic!("Parse expr seq: unexpected token {:?}", tokens.front()),
        }
    }
    args
}

fn parse_args(tokens: &mut VecDeque<Token>) -> Vec<String> {
    let mut args: Vec<String> = vec![];
    loop {
        let tok = tokens.pop_front();
        match tok {
            Some(Token::Variable(name)) => {
                args.push(name);
            }
            Some(tok) => {
                tokens.push_front(tok);
                break;
            }
            _ => {break;}
        }
    }
    args
}
fn parse_name(tokens: &mut VecDeque<Token>) -> String {
    match tokens.pop_front() {
        Some(Token::Function(name)) => name,
        _ => panic!("Expected function name after TO"),
    }
}

fn parse_block(tokens: &mut VecDeque<Token>) -> VecDeque<Command> {
    match tokens.pop_front() {
        Some(Token::LBracket) => parse(tokens),
        _ => panic!("Parse block: block should start with a '['"),
    }
}

fn parse_expr(tokens: &mut VecDeque<Token>) -> Expr {
    *expr_parser::parse_addition(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::process;

    #[test]
    fn test_parser() {
        let input = "forward 3 right 30+60 backward 4.5 left 40+4*5 show 6+2*8+5*9";
        let mut tokens = process(input);
        let ast = parse(&mut tokens);

        let expected = vec![
            Command::Forward(Expr::Number(3.0)),
            Command::Right(Expr::Add(
                Box::new(Expr::Number(30.0)),
                Box::new(Expr::Number(60.0)),
            )),
            Command::Backward(Expr::Number(4.5)),
            Command::Left(Expr::Add(
                Box::new(Expr::Number(40.0)),
                Box::new(Expr::Mul(
                    Box::new(Expr::Number(4.0)),
                    Box::new(Expr::Number(5.0)),
                )),
            )),
            Command::Show(Expr::Add(
                Box::new(Expr::Add(
                    Box::new(Expr::Number(6.0)),
                    Box::new(Expr::Mul(
                        Box::new(Expr::Number(2.0)),
                        Box::new(Expr::Number(8.0)),
                    )),
                )),
                Box::new(Expr::Mul(
                    Box::new(Expr::Number(5.0)),
                    Box::new(Expr::Number(9.0)),
                )),
            )),
        ];

        assert_eq!(ast, expected);
    }
}

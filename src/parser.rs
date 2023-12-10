use crate::lexer::Token;
use crate::expr_parser;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f32),
    Mul(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Forward(Expr),
    Backward(Expr),
    Right(Expr),
    Left(Expr),
    Show(Expr),
}

pub fn parse(tokens:&mut VecDeque<Token>) -> VecDeque<Command> {
    let mut commands = VecDeque::new();

    while let Some(token) = tokens.pop_front() {
        match token {
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
            _ => {
                // TODO
            }
        }
    }
    commands
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

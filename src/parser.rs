use crate::expr_parser;
use crate::lexer::Token;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Variable(String),
    Number(f32),
    Minus(Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Rand(Box<Expr>),
    Color(String),
    Pick(VecDeque<Expr>),
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
    If(Expr, VecDeque<Command>),
    IfElse(Expr, VecDeque<Command>, VecDeque<Command>),
    Clearscreen,
    Stop,
    Setcolor(Expr),
    PenUp,
    PenDown,
    ShowTurtle,
    HideTurtle,
    // List(Vec<Expr>),
}

pub fn parse(tokens: &mut VecDeque<Token>) -> VecDeque<Command> {
    let mut commands = VecDeque::new();

    while let Some(token) = tokens.pop_front() {
        match token {
            Token::If => {
                let pred = parse_expr(tokens);
                let body = parse_block_brackets(tokens);
                commands.push_back(Command::If(pred, body));
            }
            Token::IfElse => {
                let pred = parse_expr(tokens);
                let if_body = parse_block_brackets(tokens);
                let else_body = parse_block_brackets(tokens);
                commands.push_back(Command::IfElse(pred, if_body, else_body));
            }
            Token::Repeat => {
                let iters = parse_expr(tokens);
                let body = parse_block_brackets(tokens);
                commands.push_back(Command::Repeat(iters, body));
            }
            
            Token::To => {
                let name = parse_name(tokens);
                let args = parse_args(tokens);
                let body = parse_block_end(tokens);
                commands.push_back(Command::FunctionDeclaration(name, args, body));
            }

            Token::Function(name) => {
                let args = parse_expr_seq(tokens);
                commands.push_back(Command::FunctionCall(name, args));
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
                tokens.push_front(token);
                return commands;
            }
            Token::PenDown => commands.push_back(Command::PenDown),
            Token::PenUp => commands.push_back(Command::PenUp),
            Token::Clearscreen => commands.push_back(Command::Clearscreen),
            Token::Stop => commands.push_back(Command::Stop),
            Token::ShowTurtle => commands.push_back(Command::ShowTurtle),
            Token::HideTurtle => commands.push_back(Command::HideTurtle),
            Token::Setcolor => {
                let color = parse_expr(tokens);
                commands.push_back(Command::Setcolor(color));
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
            Some(Token::Number(Some(_))) | Some(Token::Variable(_)) | Some(Token::LParen) | Some(Token::Random)=> {
                let expr = parse_expr(tokens);
                args.push(expr);
            }
            _ => break
            // _ => panic!("Parse expr seq: unexpected token {:?}", tokens.front()),
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
            _ => {
                break;
            }
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

fn _parse_block(tokens: &mut VecDeque<Token>) -> VecDeque<Command> {
    parse(tokens)
}

fn parse_block_end(tokens: &mut VecDeque<Token>) -> VecDeque<Command> {
    let body = parse(tokens);
    if let Some(Token::End) = tokens.pop_front() {
        body
    } else {
        panic!("Repeat: block should end with END")
    }
}

fn parse_block_brackets(tokens: &mut VecDeque<Token>) -> VecDeque<Command> {
    if let Some(Token::LBracket) = tokens.pop_front() {
        let body = parse(tokens);
        if let Some(Token::RBracket) = tokens.pop_front() {
            body
        } else {
            panic!("Repeat: block should end with a ']'")
        }
    } else {
        panic!("Repeat: block should start with a '['")
    }
}

fn parse_expr(tokens: &mut VecDeque<Token>) -> Expr {
    *expr_parser::parse(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::process;

    fn vec_to_vecdeque(vec: Vec<Command>) -> VecDeque<Command> {
        let mut deque = VecDeque::new();
        deque.extend(vec);
        deque
    }

    #[test]
    fn test_parser_1() {
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

    

    #[test]
    fn test_parser_2() {
        let input = "repeat 2 [fd 50 rt 90 to square :size repeat 4 [fd :size rt 90] end] end";
        let mut tokens = process(input);
        let ast = parse(&mut tokens);

        let expected = vec_to_vecdeque(vec![Command::Repeat(
            Expr::Number(2.0),
            vec_to_vecdeque(vec![
                Command::Forward(Expr::Number(50.0)),
                Command::Right(Expr::Number(90.0)),
                Command::FunctionDeclaration(
                    "square".to_string(),
                    vec![":size".to_string()],
                    vec_to_vecdeque(vec![Command::Repeat(
                        Expr::Number(4.0),
                        vec_to_vecdeque(vec![
                            Command::Forward(Expr::Variable(":size".to_string())),
                            Command::Right(Expr::Number(90.0)),
                        ]),
                    )]),
                ),
            ]),
        )]);

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parser_3() {
        let input = "repeat 3 [to triangle fd 50 rt 120 end triangle]";
        let mut tokens = process(input);
        let ast = parse(&mut tokens);

        let expected = vec_to_vecdeque(vec![Command::Repeat(
            Expr::Number(3.0),
            vec_to_vecdeque(vec![
                Command::FunctionDeclaration(
                    "triangle".to_string(),
                    vec![],
                    vec_to_vecdeque(vec![
                        Command::Forward(Expr::Number(50.0)),
                        Command::Right(Expr::Number(120.0)),
                    ]),
                ),
                Command::FunctionCall("triangle".to_string(), vec![]),
            ]),
        )]);

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parser_4() {
        let input = "fd 2 * 3 + 4 / 2 - :size";
        let mut tokens = process(input);
        let ast = parse(&mut tokens);

        let expected = vec_to_vecdeque(vec![Command::Forward(Expr::Sub(
            Box::new(Expr::Add(
                Box::new(Expr::Mul(
                    Box::new(Expr::Number(2.0)),
                    Box::new(Expr::Number(3.0)),
                )),
                Box::new(Expr::Div(
                    Box::new(Expr::Number(4.0)),
                    Box::new(Expr::Number(2.0)),
                )),
            )),
            Box::new(Expr::Variable(":size".to_string())),
        ))]);

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parser_5() {
        // Test case with a repeat block containing multiple commands
        let input = "repeat 5 [fd 100 rt 144]";
        let mut tokens = process(input);
        let ast = parse(&mut tokens);

        let expected = vec_to_vecdeque(vec![Command::Repeat(
            Expr::Number(5.0),
            vec_to_vecdeque(vec![
                Command::Forward(Expr::Number(100.0)),
                Command::Right(Expr::Number(144.0)),
            ]),
        )]);

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parser_6() {
        // Test case with a function declaration and a repeat block inside
        let input = "to star :len repeat 5 [fd :len rt 144] end";
        let mut tokens = process(input);
        let ast = parse(&mut tokens);

        let expected = vec_to_vecdeque(vec![Command::FunctionDeclaration(
            "star".to_string(),
            vec![":len".to_string()],
            vec_to_vecdeque(vec![Command::Repeat(
                Expr::Number(5.0),
                vec_to_vecdeque(vec![
                    Command::Forward(Expr::Variable(":len".to_string())),
                    Command::Right(Expr::Number(144.0)),
                ]),
            )]),
        )]);

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parser_7() {
        // Test case with a function declaration, fd command, repeat block, and function call
        let input =
            "to funkcyja :xd :xdd fd 20 rt 3+6+(4+6)*8 end fd 23 repeat 123 [lt 1] funkcyja 3 4";
        let mut tokens = process(input);
        let ast = parse(&mut tokens);

        let expected = vec_to_vecdeque(vec![
            Command::FunctionDeclaration(
                "funkcyja".to_string(),
                vec![":xd".to_string(), ":xdd".to_string()],
                vec_to_vecdeque(vec![
                    Command::Forward(Expr::Number(20.0)),
                    Command::Right(Expr::Add(
                        Box::new(Expr::Add(
                            Box::new(Expr::Number(3.0)),
                            Box::new(Expr::Number(6.0)),
                        )),
                        Box::new(Expr::Mul(
                            Box::new(Expr::Add(
                                Box::new(Expr::Number(4.0)),
                                Box::new(Expr::Number(6.0)),
                            )),
                            Box::new(Expr::Number(8.0)),
                        )),
                    )),
                ]),
            ),
            Command::Forward(Expr::Number(23.0)),
            Command::Repeat(
                Expr::Number(123.0),
                vec_to_vecdeque(vec![Command::Left(Expr::Number(1.0))]),
            ),
            Command::FunctionCall(
                "funkcyja".to_string(),
                vec![Expr::Number(3.0), Expr::Number(4.0)],
            ),
        ]);

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parser_8() {
        // Test case with a complex function declaration for a tree pattern
        let input = "to tree :size forward :size*0.333 left 30 tree :size*2*0.333 right 30 forward :size*0.666 right 25 tree :size*0.5 left 25 forward :size*0.333 right 25 tree :size*0.5 left 25 forward :size*0.666 back :size end tree 150";
        let mut tokens = process(input);
        let ast = parse(&mut tokens);

        let expected = vec_to_vecdeque(vec![
            Command::FunctionDeclaration(
                "tree".to_string(),
                vec![":size".to_string()],
                vec_to_vecdeque(vec![
                    Command::Forward(Expr::Mul(
                        Box::new(Expr::Variable(":size".to_string())),
                        Box::new(Expr::Number(0.333)),
                    )),
                    Command::Left(Expr::Number(30.0)),
                    Command::FunctionCall(
                        "tree".to_string(),
                        vec![Expr::Mul(
                            Box::new(Expr::Mul(
                                Box::new(Expr::Variable(":size".to_string())),
                                Box::new(Expr::Number(2.0)),
                            )),
                            Box::new(Expr::Number(0.333)),
                        )],
                    ),
                    Command::Right(Expr::Number(30.0)),
                    Command::Forward(Expr::Mul(
                        Box::new(Expr::Variable(":size".to_string())),
                        Box::new(Expr::Number(0.666)),
                    )),
                    Command::Right(Expr::Number(25.0)),
                    Command::FunctionCall(
                        "tree".to_string(),
                        vec![Expr::Mul(
                            Box::new(Expr::Variable(":size".to_string())),
                            Box::new(Expr::Number(0.5)),
                        )],
                    ),
                    Command::Left(Expr::Number(25.0)),
                    Command::Forward(Expr::Mul(
                        Box::new(Expr::Variable(":size".to_string())),
                        Box::new(Expr::Number(0.333)),
                    )),
                    Command::Right(Expr::Number(25.0)),
                    Command::FunctionCall(
                        "tree".to_string(),
                        vec![Expr::Mul(
                            Box::new(Expr::Variable(":size".to_string())),
                            Box::new(Expr::Number(0.5)),
                        )],
                    ),
                    Command::Left(Expr::Number(25.0)),
                    Command::Forward(Expr::Mul(
                        Box::new(Expr::Variable(":size".to_string())),
                        Box::new(Expr::Number(0.666)),
                    )),
                    Command::Backward(Expr::Variable(":size".to_string())),
                ]),
            ),
            Command::FunctionCall("tree".to_string(), vec![Expr::Number(150.0)]),
        ]);

        assert_eq!(ast, expected);
    }

    #[test]
fn test_parser_if_statement() {
    use Command::{If, Show};
    use Expr::{*};

    // Test case with if statement
    let input = "if 4 [show 9]";
    let mut tokens = process(input);
    let ast = parse(&mut tokens);

    let expected = vec_to_vecdeque(vec![If(Number(4.0), vec_to_vecdeque(vec![Show(Number(9.0))]))]);
    
    assert_eq!(ast, expected);
}

#[test]
fn test_parser_ifelse_statement() {
    use Command::{IfElse, Show};
    use Expr::{*};
    // Test case with ifelse statement
    let input = "ifelse 3-3 [show 12] [show 2137]";
    let mut tokens = process(input);
    let ast = parse(&mut tokens);

    let expected = vec_to_vecdeque(vec![IfElse(Sub(Box::new(Number(3.0)), Box::new(Number(3.0))), 
        vec_to_vecdeque(vec![Show(Number(12.0))]), 
        vec_to_vecdeque(vec![Show(Number(2137.0))]))]);
    
    assert_eq!(ast, expected);
}


}

/*
Expected results

>> repeat 5 [ fd 100 rt 144 ]
Parsed to:
Repeat(Number(5.0), [Forward(Number(100.0)), Right(Number(144.0))])


>> to star :len repeat 5 [ fd :len rt 144 ] end
Parsed to:
FunctionDeclaration("star", [":len"], [Repeat(Number(5.0), [Forward(Variable(":len")), Right(Number(144.0))])])


>> to funkcyja :xd :xdd fd 20 rt 3+6+(4+6)*8 end
>> fd 23
>> repeat 123 [lt 1]
>> funkcyja 3 4
Parsed to:
FunctionDeclaration("funkcyja", [":xd", ":xdd"], [Forward(Number(20.0)), Right(Add(Add(Number(3.0), Number(6.0)), Mul(Add(Number(4.0), Number(6.0)), Number(8.0))))]),
Forward(Number(23.0)),
Repeat(Number(123.0), [Left(Number(1.0))]),
FunctionCall("funkcyja", [Number(3.0), Number(4.0)]),


>> to tree :size
>>    forward :size*0.333
>>    left 30 tree :size*2*0.333 right 30
>>    forward :size*0.666
>>    right 25 tree :size*0.5 left 25
>>    forward :size*0.333
>>    right 25 tree :size*0.5 left 25
>>    forward :size*0.666
>>    back :size
>> end
>> tree 150
Parsed to:
FunctionDeclaration("tree", [":size"], [Forward(Mul(Variable(":size"), Number(0.333))), Left(Number(30.0)), FunctionCall("tree", [Mul(Mul(Variable(":size"), Number(2.0)), Number(0.333))]), Right(Number(30.0)), Forward(Mul(Variable(":size"), Number(0.666))), Right(Number(25.0)), FunctionCall("tree", [Mul(Variable(":size"), Number(0.5))]), Left(Number(25.0)), Forward(Mul(Variable(":size"), Number(0.333))), Right(Number(25.0)), FunctionCall("tree", [Mul(Variable(":size"), Number(0.5))]), Left(Number(25.0)), Forward(Mul(Variable(":size"), Number(0.666))), Backward(Variable(":size"))]),
FunctionCall("tree", [Number(150.0)])


>>show 3+5*8+9+9/8/1-2-6+5+3-4*2/3
Parsed to:
Show(Sub(Add(Add(Sub(Sub(Add(Add(Add(Number(3.0), Mul(Number(5.0), Number(8.0))), Number(9.0)), Div(Div(Number(9.0), Number(8.0)), Number(1.0))), Number(2.0)), Number(6.0)), Number(5.0)), Number(3.0)), Div(Mul(Number(4.0), Number(2.0)), Number(3.0))))
Result:
Number(50.458332)


>> if 4 [show 9]
Parsed to:
If(Number(4.0), [Show(Number(9.0))])
>> ifelse 3-3 [show 12] [show 2137]
Parsed to:
IfElse(Sub(Number(3.0), Number(3.0)), [Show(Number(12.0))], [Show(Number(2137.0))])
*/

use logos::Logos;
use std::collections::VecDeque;

#[derive(Debug, Logos, PartialEq)]
pub enum Token {
    #[regex(r"fd|forward")]
    Forward,
    #[regex(r"bk|back|backward")]
    Backward,
    #[regex(r"lt|left")]
    Left,
    #[regex(r"rt|right")]
    Right,
    #[regex(r"[0-9]+(?:\.[0-9]+)?", |lex| lex.slice().parse::<f32>().ok())]
    Number(Option<f32>),
    #[token("*")]
    Mul,
    #[token("+")]
    Add,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("show")]
    Show,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub fn process(input: &str) -> VecDeque<Token> {
    let processed = Token::lexer(input)
        .map(|tok| match tok {
            Ok(t) => t,
            Err(()) => panic!("Syntax error"),
        })
        .collect::<VecDeque<Token>>();
    processed
}

pub fn _process_line(source: &str) {
    let tokens = process(source);
    tokens.iter().for_each(|token| match token {
        Token::Forward => println!("Forward"),
        Token::Number(Some(n)) => println!("Number {}", n),
        Token::Add => println!("Add"),
        Token::Mul => println!("Mul"),
        Token::Right => println!("Right"),
        _ => eprintln!("Unknown command / Unimplemented"),
    });
}

/*
this logo code:

forward 3 right 30+60
backward 4.5
left 40+4*5
show 6+2*8+5*9


should parse to the following:

Forward(Number(3.0))
Right(Add(Number(30.0, Number(60.0))))
Backward(Number(4.5))
Left(Add(Number(40.0), Mul(Number(4.0),Number(5.0))))
Show(Add(Add(Number(6.0),Mul(Number(2.0), Number(8.0))), Mul(Number(5.0), Number(9.0))))))
*/

/*
- function declaration
to name :arg1 :arg2
  body
end

- loop
repeat n [body]

- function call
builtin functions:
* clearscreen
* cleartext

- lists
[1 2 3]



*/

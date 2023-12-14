use logos::Logos;
use std::collections::VecDeque;

#[derive(Debug, Logos, PartialEq)]
pub enum Token {
    /* arithmetic expressions */
    #[token("*")]
    Mul,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("/")]
    Div,
    #[token("<")]
    Lt,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    /* control flow */
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("repeat")]
    Repeat,
    #[token("stop")]
    Stop,
    #[token("if")]
    If,
    #[token("ifelse")]
    IfElse,
    #[regex(r"to|TO")]
    To,
    #[regex(r"end|END")]
    End,
    /* built in procedures */
    #[token("show")]
    Show,
    #[token("wait")]
    Wait,
    #[token("pick")]
    Pick, // pick [list] - take random elem from the list
    /* colors: red orange yellow green blue violet */
    #[token("random")]
    Random,
    #[token("red")]
    Red,
    #[token("orange")]
    Orange,
    #[token("yellow")]
    Yellow,
    #[token("green")]
    Green,
    #[token("blue")]
    Blue,
    #[token("violet")]
    Violet,
    #[token("black")]
    Black,
    /* image */
    #[token("clearscreen")]
    Clearscreen,
    #[token("setcolor")]
    Setcolor,
    #[regex(r"fd|forward")]
    Forward,
    #[regex(r"bk|back|backward")]
    Backward,
    #[regex(r"lt|left")]
    Left,
    #[regex(r"rt|right")]
    Right,
    #[regex(r"pu|penup")]
    PenUp,
    #[regex(r"pd|pendown")]
    PenDown,
    #[regex(r"st|showturtle")]
    ShowTurtle,
    #[regex(r"ht|hideturtle")]
    HideTurtle,
    #[token("setturtle")]
    SetTurtle,
    /* datatypes */
    #[regex(r"[0-9]+(?:\.[0-9]+)?", |lex| lex.slice().parse::<f32>().ok())]
    Number(Option<f32>),
    #[regex(r":[a-z]+", |lex| lex.slice().to_string())]
    Variable(String),
    #[regex(r"[a-z]+", |lex| lex.slice().to_string())]
    Function(String),
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
/* TODO
- function declaration
to name :arg1 :arg2
  body
end

- function call
builtin functions:
* clearscreen
* cleartext

- lists
[1 2 3]

*/

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

use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
enum Token {
    #[regex(r"fd|forward")]
    Forward,
    // #[regex(r"bk|back")] // FIXME suspended for simplicity
    // Backward,
    // #[regex(r"lt|left")]
    // Left,
    #[regex(r"rt|right")]
    Right,
    #[regex(r"[0-9]+")]
    Number,
    #[token("*")]
    Mul,
    #[token("+")]
    Add,
    // #[error] // for providing custom lexer err handling, but it doesn't work xd
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub fn process_line(source: &str) {
  // let source = "fd 100 forward 50 rt 90 fd 4 * 5 + 2 right 60+3*4";
  let mut lexer = Token::lexer(source);

  while let Some(token) = lexer.next() {
      match token {
          Ok(Token::Forward) => println!("Forward"),
          Ok(Token::Number) => println!("Number: {}", lexer.slice()),
          Ok(Token::Add) => println!("Add"),
          Ok(Token::Mul) => println!("Mul"),
          Ok(Token::Right) => println!("Right"),
          Err(()) => eprintln!("Lexer error"),
          _ => eprintln!("Unknown command / Unimplemented")
      }
  }
}


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
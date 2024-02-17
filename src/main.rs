mod evaluator;
mod expr_parser;
mod lexer;
mod parser;
use std::collections::{HashMap, VecDeque};

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

fn get_matches() -> clap::ArgMatches<'static> {
    App::new("Logo Interpreter")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Set input file, default = stdin")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Redirect stdout")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("graphics")
                .short("g")
                .long("graphics")
                .value_name("FILE")
                .help("Save image to file, default = result.svg")
                .takes_value(true),
        )
        .get_matches()
}
fn main() {
    let matches: clap::ArgMatches<'_> = get_matches();
    let mut image = evaluator::Image::new(700.0, 700.0);
    let mut functions: HashMap<String, (Vec<String>, VecDeque<parser::Command>)> = HashMap::new();
    if let Some(input_file) = matches.value_of("input") {
        /* Parse a script - Redirect input from file */
        let file = File::open(input_file).expect("Failed to open input file");
        let mut reader = BufReader::new(file);
        let mut prog = "".to_string();
        if reader.read_to_string(&mut prog).is_err() {
            panic!("Can not read input file")
        }
        
        let mut tokens: VecDeque<lexer::Token> = lexer::process(prog.as_str());
        // println!("Tokenized to:\n{:?}\n**********************************************************", tokens);
        
        let ast: VecDeque<parser::Command> = parser::parse(&mut tokens);
        // println!("Parsed to:");
        // for cmd in ast.clone(){
        //     println!("{:?}", cmd);
        // }
        // println!("**********************************************************");
        _ = evaluator::eval_all(ast, &mut functions, &HashMap::new(), &mut image);

        
    } else {
        /* Start interactive session */
        println!("Enter Logo command (or 'exit' to quit)");
        loop {
            print!(">>");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).unwrap() == 0 {
                println!("exit");
                break;
            }

            let input = input.trim();

            if input.eq_ignore_ascii_case("exit") {
                break;
            }
            let mut tokens: VecDeque<lexer::Token> = lexer::process(input);
            let ast: VecDeque<parser::Command> = parser::parse(&mut tokens);
            println!("Parsed to:\n{:?}", ast);
            _ = evaluator::eval_all(ast, &mut functions, &HashMap::new(), &mut image);
        }
    }
    if let Some(output_file) = matches.value_of("graphisc") {
        image.save_svg(&format!("{}.svg", output_file)[..]);
    } else {
        image.save_svg("output.svg");
    }
}

mod lexer;
use std::io::{self, Write, BufRead, BufReader};
use clap::{App, Arg};
use std::fs::File;

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

    if let Some(input_file) = matches.value_of("input") {
        // Redirect input from the specified file
        let file = File::open(input_file).expect("Failed to open input file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            lexer::process_line(&line.expect("Error reading line from input file"));
        }
    } else {
    /* Start interactive session */
    println!("Enter Logo command (or 'exit' to quit)");
    loop {
        print!(">>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap()==0{
            println!("exit");
            break;
        }

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        lexer::process_line(input);
    }
}
}
/* tutorials */
/* https://rust-hosted-langs.github.io/book/introduction.html */
/* https://stopa.io/post/222 */
/* https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html */
/* https://blog.cloudflare.com/building-fast-interpreters-in-rust/ */
/* https://p3ld3v.medium.com/writing-interpreter-in-rust-using-grmtools-7a6a0458b99f */

/* book */
/* https://craftinginterpreters.com/contents.html */

/* crates and tools */
/* https://github.com/rust-bakery/nom */
/* https://lib.rs/crates/rowan */
/* https://docs.rs/logos/0.12.0/logos/index.html#logos */

/* logo interpreters */
/* https://github.com/ivansandrk/Rust-Logo4.0-Interpreter */
/* https://github.com/wojteklu/logo */
/* https://github.com/inexorabletash/jslogo */


/*
    * lexer - logo code to tokens
    * parser - list of tokens to abstract syntax tree
    * evaluator - the interpreter
    * graphics - result to svg
*/
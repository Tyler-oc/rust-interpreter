mod lexing;

use lexing::lexer::lex_program;
use lexing::token::Token;
use std::env;

fn process_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let mut optional_program: Option<String> = None;
    if args.len() == 2 {
        optional_program = Some(args[1].clone());
    }
    return optional_program;
}

//file input (make sure to actually extract the text from the file)
fn run_file(program: &str) {
    let tokens: Vec<Token> = lex_program(program);

    for token in tokens.iter() {
        println!("{:?}", token);
    }
}

//CLI listening
fn run_prompt() {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let tokens: Vec<Token> = lex_program(&input);

    for token in tokens.iter() {
        println!("{:?}", token);
    }
}

fn main() {
    let program: Option<String> = process_args();
    match program {
        Some(p) => run_file(&p),
        None => run_prompt(),
    }
}

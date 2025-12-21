mod errors;
mod lexing;

use lexing::lexer::lex_program;
use lexing::token::Token;
use std::env;
use std::fs;

//error handling but make sure to pass in specific errors which are defined in the errors crate.
pub fn error(line: &u32, message: &str) {
    report(line, "", message);
}

fn report(line: &u32, where_at: &str, message: &str) {
    panic!("Error on line {line} at {where_at}: {message}");
}

fn process_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let mut file_path: Option<String> = None;
    if args.len() == 2 {
        file_path = Some(args[1].clone());
    }
    return file_path;
}

//file input
fn run_file(program_file: &str) {
    let bytes = fs::read(program_file);

    match bytes {
        Ok(file_bytes) => {
            let program: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&file_bytes);
            let tokens: Vec<Token> = lex_program(&program);
            for token in tokens.iter() {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            println!("Error: {e}");
        }
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
    let program_file: Option<String> = process_args();
    match program_file {
        Some(p) => run_file(&p),
        None => run_prompt(),
    }
}

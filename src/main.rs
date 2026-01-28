mod errors;
mod interpreting;
mod lexing;
mod parsing;

use lexing::lexer::lex_program;
use lexing::token::Token;
use std::env;
use std::fs;

use crate::errors::interpreter_error::InterpreterError;
use crate::interpreting::interpreter::interpret;
use crate::interpreting::value::Value;
use crate::parsing::ast::Expr;
use crate::parsing::ast::Stmt;
use crate::parsing::parser::parse_tokens;

pub fn error(e: InterpreterError) {
    println!("{}", e);
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
fn run_file(program_file: &str) -> Result<(), InterpreterError> {
    let bytes = fs::read(program_file);

    match bytes {
        Ok(file_bytes) => {
            let program: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&file_bytes);
            let tokens: Vec<Token> = lex_program(&program)?;
            for token in tokens.iter() {
                println!("{:?}", token);
            }
            let statements: Vec<Stmt> = parse_tokens(&tokens)?;

            for statement in statements.iter() {
                println!("{}", statement)
            }

            interpret(statements)?;
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
    Ok(())
}

//CLI listening
fn run_prompt() -> Result<(), InterpreterError> {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let tokens: Vec<Token> = match lex_program(&input) {
        Ok(tokens) => tokens,
        Err(e) => {
            error(InterpreterError::LexError(e));
            Vec::new()
        }
    };

    for token in tokens.iter() {
        println!("{:?}", token);
    }

    let statements: Vec<Stmt> = parse_tokens(&tokens)?;

    for statement in statements.iter() {
        println!("{}", statement)
    }

    interpret(statements)?;

    Ok(())
}

fn main() {
    let program_file: Option<String> = process_args();
    match program_file {
        Some(p) => match run_file(&p) {
            Ok(_) => (),
            Err(e) => error(e),
        },
        None => match run_prompt() {
            Ok(_) => (),
            Err(e) => error(e),
        },
    }
}

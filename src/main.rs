mod environment;
mod errors;
mod interpreting;
mod lexing;
mod parsing;
mod resolving;

use lexing::lexer::lex_program;
use lexing::token::Token;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::rc::Rc;
use std::time::Instant;

use crate::environment::environment::Environment;
use crate::errors::interpreter_error::InterpreterError;
use crate::interpreting::interpreter::Interpreter;
use crate::interpreting::value::Value;
use crate::parsing::ast::Expr;
use crate::parsing::ast::Stmt;
use crate::parsing::parser::parse_tokens;
use crate::resolving::resolver::Resolver;

//defining type to be used throughout the program
type WrappedEnv = Rc<RefCell<Environment>>;

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

            let start_time = Instant::now();

            let tokens: Vec<Token> = lex_program(&program)?;
            let statements: Vec<Stmt> = parse_tokens(&tokens)?;

            let mut interpreter = Interpreter::new();

            let mut resolver = Resolver::new(&interpreter);

            resolver.resolve_stmts(&statements)?;

            interpreter.interpret(&statements)?;

            let duration = start_time.elapsed();
            println!("Execution time: {:.2?}", duration);
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
    Ok(())
}

// CLI listening
fn run_prompt() -> Result<(), InterpreterError> {
    let mut interpreter = Interpreter::new();

    loop {
        print!("> ");
        use std::io::Write;
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes_read = std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if bytes_read == 0 {
            break;
        }

        let tokens = match lex_program(&input) {
            Ok(tokens) => tokens,
            Err(e) => {
                error(InterpreterError::LexError(e));
                continue;
            }
        };

        let statements = match parse_tokens(&tokens) {
            Ok(stmts) => stmts,
            Err(e) => {
                println!("Parse Error: {}", e);
                continue;
            }
        };

        let mut resolver = Resolver::new(&interpreter);
        if let Err(e) = resolver.resolve_stmts(&statements) {
            println!("Resolve Error: {}", e);
            continue;
        }

        if let Err(e) = interpreter.interpret(&statements) {
            println!("Runtime Error: {}", e);
        }
    }

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

mod lexing;

use lexing::lexer::lex_program;
use lexing::token::Token;

const PROGRAM: &str = "
int x = 5;
int y = 6;
int z = x + y;

if (z > 10) {
    print(\"Hello, world!\");
} else {
    print(\"Goodbye, world!\");
}
";

fn process_args(): Option<String> {
    let args: Vec<String> = env::args().collect();
    let mut optional_program: Option<String> = None;
    if args.len == 2 {
        program = args[1];
    }
    return program;
}

//file input (make sure to actually extract the text from the file)
fn run_file(program: String) {
    let tokens: Vec<Token> = lex_program(program);
    
    for token in tokens.iter() {
        println!("{:?}", token);
    }
}

//CLI listening
fn run_prompt() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let tokens: Vec<Token> = lex_program(input);
    
    for token in tokens.iter() {
        println!("{:?}", token);
    }
}

fn main() {
    let program: String = process_args();
    match program {
        case Some(p) => run_file(p);
        case None => run_prompt();
    }
}

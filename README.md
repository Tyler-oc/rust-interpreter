# Rust Interpreter

A tree-walking interpreter for a custom language, implemented in Rust.

## Features

- **Lexer**: Tokenizes source code into meaningful symbols.  
- **Parser**: Recursive descent parser building an Abstract Syntax Tree (AST).  
- **Interpreter**: Tree-walking evaluation of expressions with runtime values.  
- **Error Handling**: Custom reporting for syntax and runtime errors.  

## Tech Stack

- Just Rust with cargo for build dependencies. Nothing extra needed

## Example Usage

cargo build
cargo run -- <Path_to_your_text_file>

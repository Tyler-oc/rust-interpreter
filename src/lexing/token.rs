use crate::errors::lex_error::LexError;
#[derive(Clone, Debug)]
pub enum Token {
    //keywords
    Print,
    If,
    Else,
    For,
    While,
    Null,
    And,
    Or,
    Fun,
    Class,
    Super,
    Return,
    This,

    //literal
    Number(f64),
    StringLiteral(String),
    Boolean(bool),

    //identifiers
    Identifier(String),
    Var,

    //operators
    Plus,
    Minus,
    Dot, //property calls
    Slash,
    Star,
    Equal,
    Comma,

    //punctuation
    Semicolon,
    LeftParen,
    RightParen,
    RightBrace,
    LeftBrace,

    //logical operator
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Bang,
    BangEqual,
    EqualEqual,

    EOF,
}

impl Token {}

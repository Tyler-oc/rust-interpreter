use crate::{errors::lex_error::LexError, parsing::ast::Literal};
#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
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
    Number,
    StringLiteral,
    True,
    False,

    //identifiers
    Identifier,
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

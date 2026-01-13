use crate::{
    errors::lex_error::LexError,
    lexing::token::{Token, TokenKind},
    parsing::ast::Literal,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEYWORDS: Lazy<HashMap<&'static str, TokenKind>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TokenKind::And);
    m.insert("class", TokenKind::Class);
    m.insert("else", TokenKind::Else);
    m.insert("false", TokenKind::False);
    m.insert("for", TokenKind::For);
    m.insert("fun", TokenKind::Fun);
    m.insert("if", TokenKind::If);
    m.insert("null", TokenKind::Null);
    m.insert("or", TokenKind::Or);
    m.insert("print", TokenKind::Print);
    m.insert("return", TokenKind::Return);
    m.insert("super", TokenKind::Super);
    m.insert("this", TokenKind::This);
    m.insert("true", TokenKind::True);
    m.insert("var", TokenKind::Var);
    m.insert("while", TokenKind::While);
    m
});

struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        c
    }

    fn peek(&mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char;
        c
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.as_bytes()[self.current + 1] as char
    }

    fn match_char(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() == c {
            return true;
        }
        false
    }

    fn add_token(&mut self, kind: TokenKind, literal: Option<Literal>) {
        let lexeme = self.source[self.start..self.current].to_string();

        self.tokens.push(Token {
            kind,
            lexeme,
            literal,
            line: self.line,
        })
    }

    fn increment_line(&mut self) {
        self.line += 1
    }

    fn string(&mut self) -> Result<(), LexError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.increment_line()
            };
            self.advance();
        }

        if self.is_at_end() {
            return Err(LexError::UnterminatedString { line: self.line });
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1]; //note this only works if all values are ASCII, which is assumed. Panics otherwise

        self.add_token(
            TokenKind::StringLiteral,
            Some(Literal::StringLiteral(value.to_string())),
        );
        Ok(())
    }

    fn number(&mut self) -> Result<(), LexError> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let num_str = &self.source[self.start..self.current];
        let parsed_num: Result<f64, LexError> = match num_str.parse() {
            Ok(num) => Ok(num),
            Err(_) => Err(LexError::NumberParsingError {
                num_str: num_str.to_string(),
                line: self.line,
            }),
        };
        match parsed_num {
            Ok(val) => self.add_token(TokenKind::Number, Some(Literal::Number(val))),
            Err(e) => return Err(e),
        };
        Ok(())
    }

    fn identifier(&mut self) -> () {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let kind = match KEYWORDS.get(text) {
            Some(kind) => kind.clone(),
            None => TokenKind::Identifier,
        };

        self.add_token(kind, None);
    }

    fn scan_token(&mut self) -> Result<(), LexError> {
        let c = self.advance();

        match c {
            '\n' => self.increment_line(),
            '(' => self.add_token(TokenKind::LeftParen, None),
            ')' => self.add_token(TokenKind::RightParen, None),
            '{' => self.add_token(TokenKind::LeftBrace, None),
            '}' => self.add_token(TokenKind::RightBrace, None),
            '+' => self.add_token(TokenKind::Plus, None),
            '-' => self.add_token(TokenKind::Minus, None),
            '*' => self.add_token(TokenKind::Star, None),
            '.' => self.add_token(TokenKind::Dot, None),
            ';' => self.add_token(TokenKind::Semicolon, None),
            ',' => self.add_token(TokenKind::Comma, None),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::BangEqual, None);
                    self.advance();
                } else {
                    self.add_token(TokenKind::Bang, None);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::EqualEqual, None);
                    self.advance();
                } else {
                    self.add_token(TokenKind::Equal, None);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::LessEqual, None);
                    self.advance();
                } else {
                    self.add_token(TokenKind::LessThan, None);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::GreaterEqual, None);
                    self.advance();
                } else {
                    self.add_token(TokenKind::GreaterThan, None);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenKind::Slash, None);
                }
            }
            '"' => {
                let result = self.string();
                match result {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
            '\r' => (),
            '\t' => (),
            ' ' => (),
            _ => {
                if is_digit(c) {
                    let result = self.number();
                    match result {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    return Err(LexError::UnexpectedCharacter {
                        char: c,
                        line: self.line,
                    });
                }
            }
        };
        Ok(())
    }
}

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn is_alpha(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_');
}

fn is_alphanumeric(c: char) -> bool {
    return is_alpha(c) || is_digit(c);
}

pub fn lex_program(source: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(source);

    while !lexer.is_at_end() {
        lexer.start = lexer.current;
        lexer.scan_token()?;
    }

    lexer.tokens.push(Token {
        kind: TokenKind::EOF,
        lexeme: "".to_string(),
        literal: None,
        line: lexer.line,
    });

    Ok(lexer.tokens)
}

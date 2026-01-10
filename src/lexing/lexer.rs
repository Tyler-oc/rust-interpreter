use crate::{errors::lex_error::LexError, lexing::token::Token};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEYWORDS: Lazy<HashMap<&'static str, Token>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and", Token::And);
    m.insert("class", Token::Class);
    m.insert("else", Token::Else);
    m.insert("false", Token::False);
    m.insert("for", Token::For);
    m.insert("fun", Token::Fun);
    m.insert("if", Token::If);
    m.insert("null", Token::Null);
    m.insert("or", Token::Or);
    m.insert("print", Token::Print);
    m.insert("return", Token::Return);
    m.insert("super", Token::Super);
    m.insert("this", Token::This);
    m.insert("true", Token::True);
    m.insert("var", Token::Var);
    m.insert("while", Token::While);
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

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
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

        self.add_token(Token::StringLiteral(value.to_string()));
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
            Ok(val) => self.add_token(Token::Number(val)),
            Err(e) => return Err(e),
        };
        Ok(())
    }

    fn identifier(&mut self) -> () {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = match KEYWORDS.get(text) {
            Some(t) => t,
            None => &Token::Identifier(text.to_string()),
        };

        self.add_token(token_type.clone());
    }

    fn scan_token(&mut self) -> Result<(), LexError> {
        let c = self.advance();

        match c {
            '\n' => self.increment_line(),
            '(' => self.add_token(Token::LeftParen),
            ')' => self.add_token(Token::RightParen),
            '{' => self.add_token(Token::LeftBrace),
            '}' => self.add_token(Token::RightBrace),
            '+' => self.add_token(Token::Plus),
            '-' => self.add_token(Token::Minus),
            '*' => self.add_token(Token::Star),
            '.' => self.add_token(Token::Dot),
            ';' => self.add_token(Token::Semicolon),
            ',' => self.add_token(Token::Comma),
            '!' => {
                if self.match_char('=') {
                    self.add_token(Token::BangEqual);
                    self.advance();
                } else {
                    self.add_token(Token::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(Token::EqualEqual);
                    self.advance();
                } else {
                    self.add_token(Token::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(Token::LessEqual);
                    self.advance();
                } else {
                    self.add_token(Token::LessThan);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(Token::GreaterEqual);
                    self.advance();
                } else {
                    self.add_token(Token::GreaterThan);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Token::Slash);
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

    lexer.add_token(Token::EOF);

    Ok(lexer.tokens)
}

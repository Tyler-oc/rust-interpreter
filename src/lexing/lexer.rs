use crate::{errors::lexerror::LexError, lexing::token::Token};
use regex::Regex;

// //TODO: can't be global matching has to be anchored to index
// pub fn lex_program(program: &str) -> Vec<Token> {
//     let current_input = program;

//     //highest to lowest priority
//     //ADD NEW TOKENS
//     let tokens = [
//         "Print",
//         "If",
//         "Else",
//         "While",
//         "For",
//         "Fun",
//         "Class",
//         "Super",
//         "Return",
//         "This",
//         "And",
//         "Or",
//         "Null",
//         "Plus",
//         "Minus",
//         "Star",
//         "Slash",
//         "Dot",
//         "Assign",
//         "Semicolon",
//         "LeftParen",
//         "RightParen",
//         "LeftBrace",
//         "RightBrace",
//         "Bang",
//         "BangEqual",
//         "GreaterEqual",
//         "LessEqual",
//         "EqualEqual",
//         "GreaterThan",
//         "LessThan",
//         "IntegerLiteral",
//         "StringLiteral",
//         "True",
//         "False",
//         "Identifier",
//     ];

//     let mut match_vec: Vec<(&str, usize, usize)> = Vec::new();

//     for token in tokens.iter() {
//         let token_regex = Token::get_token_regex(token);
//         match token_regex {
//             Ok(t) => {
//                 let re = Regex::new(t.as_str()).unwrap();
//                 let matched = re.find_iter(current_input);

//                 let all_matches = matched.collect::<Vec<_>>();

//                 if all_matches.len() == 0 {
//                     continue;
//                 }

//                 for m in all_matches.iter() {
//                     match_vec.push((token, m.start(), m.end()));
//                 }
//             }
//             Err(e) => eprintln!("{}", e),
//         };
//     }

//     match_vec.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| (b.2 - b.1).cmp(&(a.2 - a.1))));

//     let mut token_vec: Vec<Token> = Vec::new();
//     for m in match_vec.iter() {
//         let token = Token::get_token(m.0, Some(&current_input[m.1..m.2]));
//         match token {
//             Ok(t) => token_vec.push(t),
//             Err(e) => eprintln!("{}", e),
//         }
//         //token_vec.push(Token::get_token(m.0, Some(&current_input[m.1..m.2])));
//     }

//     return token_vec;
// }

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
                return Err(LexError::UnexpectedCharacter {
                    char: c,
                    line: self.line,
                });
            }
        };
        Ok(())
    }
}

pub fn lex_program(source: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(source);

    while !lexer.is_at_end() {
        lexer.start = lexer.current;
        lexer.scan_token()?;
    }

    Ok(lexer.tokens)
}

use crate::{
    errors::parse_error::ParseError,
    lexing::token::Token,
    parsing::ast::{BinaryOp, Expr, Literal, UnaryOp},
};

struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn is_at_end(&mut self) -> bool {
        match self.peek() {
            Token::EOF => true,
            _ => false,
        }
    }

    fn previous(&mut self) -> Result<&Token, ParseError> {
        match self.tokens.get(self.current - 1) {
            Some(t) => Ok(t),
            None => Err(ParseError::IndexOutOfBounds),
        }
    }

    fn advance(&mut self) -> Result<&Token, ParseError> {
        if !self.is_at_end() {
            self.current += 1;
        }
        match self.previous() {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        }
    }

    fn check(&mut self, token_type: Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek() == token_type;
    }

    fn match_token(&mut self, token_types: Vec<Token>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while match_token(Token::EqualEqual..Token::BangEqual) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary {
                left: expr,
                op: operator,
                right: right,
            }
        }

        expr
    }

    pub fn parse(&mut self) -> Expr {
        self.equality()
    }
}

pub fn parse_tokens(tokens: &Vec<Token>) -> Expr {
    let mut parser: Parser = Parser::new(tokens);

    let expr: Expr = parser.parse();

    expr
}

pub fn parse_binary_op(token: &Token) -> Result<BinaryOp, ParseError> {
    match token {
        Token::And => Ok(BinaryOp::And),
        Token::Or => Ok(BinaryOp::Or),
        Token::Plus => Ok(BinaryOp::Plus),
        Token::Minus => Ok(BinaryOp::Minus),
        Token::Star => Ok(BinaryOp::Star),
        Token::Slash => Ok(BinaryOp::Slash),
        Token::GreaterEqual => Ok(BinaryOp::GreaterEqual),
        Token::GreaterThan => Ok(BinaryOp::GreaterThan),
        Token::EqualEqual => Ok(BinaryOp::EqualEqual),
        Token::BangEqual => Ok(BinaryOp::BangEqual),
        Token::LessEqual => Ok(BinaryOp::LessEqual),
        Token::LessThan => Ok(BinaryOp::LessThan),
        Token::Equal => Ok(BinaryOp::Equal),
        _ => Err(ParseError::InvalidConversion(
            "could not convert to binary operator".to_string(),
        )), //figure out formatting later to enter to display tokens
    }
}

pub fn parse_unary_op(token: &Token) -> Result<UnaryOp, ParseError> {
    match token {
        Token::Bang => Ok(UnaryOp::Bang),
        _ => Err(ParseError::InvalidConversion(
            "could not convert to unary operator".to_string(),
        )),
    }
}

pub fn parse_literal(token: &Token) -> Result<Literal, ParseError> {
    match token {
        Token::Number(i) => Ok(Literal::Number(*i)),
        Token::StringLiteral(s) => Ok(Literal::StringLiteral(s.clone())),
        Token::Boolean(b) => Ok(Literal::Boolean(*b)),
        Token::Null => Ok(Literal::Null),
        _ => Err(ParseError::InvalidConversion(
            "could not convert literal".to_string(),
        )),
    }
}

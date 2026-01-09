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

    fn peek(&mut self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&mut self) -> bool {
        match self.peek() {
            Token::EOF => true,
            _ => false,
        }
    }

    fn previous(&mut self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&mut self, token_type: Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().clone() == token_type;
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

    fn primary(&mut self) => Result<Expr, ParseError> {
        if self.match_token(vec![Token::Boolean]) {
            return Ok(Literal::Boolean(false))
        }
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(vec![Token::Bang, Token::Minus]) {
            let operator: UnaryOp = match parse_unary_op(self.previous()) {
                Ok(b) => b,
                Err(e) => return Err(e),
            };
            let right: Expr = match self.unary() {
                Ok(e) => e,
                Err(err) => return Err(e),
            };
            return Ok(Expr::Unary {
                exp: Box::new(right),
                op: operator,
            });
        }
        self.primary()
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = match self.unary() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };

        while self.match_token(vec![Token::Slash, Token::Star]) {
            let operator: BinaryOp = match parse_binary_op(self.previous()) {
                Ok(b) => b,
                Err(e) => return Err(e),
            };
            let right: Expr = match self.unary() {
                Ok(e) => e,
                Err(err) => return Err(err),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = match self.factor() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };

        while self.match_token(vec![Token::Minus, Token::Plus]) {
            let operator: BinaryOp = match parse_binary_op(self.previous()) {
                Ok(b) => b,
                Err(e) => return Err(e),
            };
            let right: Expr = match self.factor() {
                Ok(e) => e,
                Err(err) => return Err(err),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = match self.term() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };

        while self.match_token(vec![
            Token::GreaterEqual,
            Token::GreaterThan,
            Token::LessEqual,
            Token::LessThan,
        ]) {
            let operator: BinaryOp = match parse_binary_op(self.previous()) {
                Ok(b) => b,
                Err(e) => return Err(e),
            };
            let right: Expr = match self.term() {
                Ok(e) => e,
                Err(err) => return Err(err),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = match self.comparison() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };

        while self.match_token(vec![Token::EqualEqual, Token::BangEqual]) {
            let operator: BinaryOp = match parse_binary_op(self.previous()) {
                Ok(b) => b,
                Err(e) => return Err(e),
            };
            let right: Expr = match self.comparison() {
                Ok(e) => e,
                Err(err) => return Err(err),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        match self.equality() {
            Ok(e) => Ok(e),
            Err(err) => return Err(err),
        }
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

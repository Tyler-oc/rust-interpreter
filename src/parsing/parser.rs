use crate::{
    errors::parse_error::ParseError,
    lexing::token::{Token, TokenKind},
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
        match self.peek().kind {
            TokenKind::EOF => true,
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

    fn check(&mut self, token_type: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().kind == token_type;
    }

    fn match_token(&mut self, token_types: Vec<TokenKind>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn synchronize(&mut self) -> () {
        self.advance();

        while !self.is_at_end() {
            match self.previous().kind {
                TokenKind::Semicolon => return (),
                TokenKind::Class => return (),
                TokenKind::Fun => return (),
                TokenKind::Var => return (),
                TokenKind::For => return (),
                TokenKind::If => return (),
                TokenKind::While => return (),
                TokenKind::Print => return (),
                TokenKind::Return => return (),
                _ => (),
            }
            self.advance();
        }
    }

    fn consume(&mut self, token_type: TokenKind, message: String) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(ParseError::InvalidGrouping(message))
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        println!("primary");
        if self.match_token(vec![TokenKind::LeftParen]) {
            let expr: Expr = match self.expression() {
                Ok(e) => e,
                Err(err) => return Err(err),
            };
            match self.consume(
                TokenKind::RightParen,
                "Expect ')' after expression".to_string(),
            ) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
            return Ok(Expr::Grouping {
                exp: Box::new(expr),
            });
        }

        match parse_literal(self.peek()) {
            Ok(l) => Ok(Expr::Literal(l)),
            Err(e) => Err(e),
        }
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        println!("unary");
        if self.match_token(vec![TokenKind::Bang, TokenKind::Minus]) {
            let operator: UnaryOp = match parse_unary_op(self.previous()) {
                Ok(b) => b,
                Err(e) => return Err(e),
            };
            let right: Expr = match self.unary() {
                Ok(e) => e,
                Err(err) => return Err(err),
            };
            return Ok(Expr::Unary {
                exp: Box::new(right),
                op: operator,
            });
        }
        self.primary()
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        println!("factor");
        let mut expr: Expr = match self.unary() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };

        while self.match_token(vec![TokenKind::Slash, TokenKind::Star]) {
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
        println!("term");
        let mut expr: Expr = match self.factor() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };

        while self.match_token(vec![TokenKind::Minus, TokenKind::Plus]) {
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
        println!("comparison");
        let mut expr: Expr = match self.term() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };

        while self.match_token(vec![
            TokenKind::GreaterEqual,
            TokenKind::GreaterThan,
            TokenKind::LessEqual,
            TokenKind::LessThan,
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
        println!("Equality");
        let mut expr: Expr = match self.comparison() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };

        while self.match_token(vec![TokenKind::EqualEqual, TokenKind::BangEqual]) {
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

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        println!("Expression");
        match self.equality() {
            Ok(e) => Ok(e),
            Err(err) => return Err(err),
        }
    }
}

pub fn parse_tokens(tokens: &Vec<Token>) -> Result<Expr, ParseError> {
    let mut parser: Parser = Parser::new(tokens);

    let expr: Expr = match parser.expression() {
        Ok(e) => e,
        Err(err) => return Err(err),
    };

    Ok(expr)
}

pub fn parse_binary_op(token: &Token) -> Result<BinaryOp, ParseError> {
    match token.kind {
        TokenKind::And => Ok(BinaryOp::And),
        TokenKind::Or => Ok(BinaryOp::Or),
        TokenKind::Plus => Ok(BinaryOp::Plus),
        TokenKind::Minus => Ok(BinaryOp::Minus),
        TokenKind::Star => Ok(BinaryOp::Star),
        TokenKind::Slash => Ok(BinaryOp::Slash),
        TokenKind::GreaterEqual => Ok(BinaryOp::GreaterEqual),
        TokenKind::GreaterThan => Ok(BinaryOp::GreaterThan),
        TokenKind::EqualEqual => Ok(BinaryOp::EqualEqual),
        TokenKind::BangEqual => Ok(BinaryOp::BangEqual),
        TokenKind::LessEqual => Ok(BinaryOp::LessEqual),
        TokenKind::LessThan => Ok(BinaryOp::LessThan),
        TokenKind::Equal => Ok(BinaryOp::Equal),
        _ => Err(ParseError::InvalidConversion(
            "could not convert to binary operator".to_string(),
        )), //figure out formatting later to enter to display tokens
    }
}

pub fn parse_unary_op(token: &Token) -> Result<UnaryOp, ParseError> {
    match token.kind {
        TokenKind::Bang => Ok(UnaryOp::Bang),
        _ => Err(ParseError::InvalidConversion(
            "could not convert to unary operator".to_string(),
        )),
    }
}

pub fn parse_literal(token: &Token) -> Result<Literal, ParseError> {
    match token.kind {
        TokenKind::Number => match &token.literal {
            Some(l) => Ok(l.clone()),
            None => Err(ParseError::MissingValue {
                val: token.lexeme.clone(),
                line: token.line,
            }),
        },
        TokenKind::StringLiteral => match &token.literal {
            Some(l) => Ok(l.clone()),
            None => Err(ParseError::MissingValue {
                val: token.lexeme.clone(),
                line: token.line,
            }),
        },
        TokenKind::False => Ok(Literal::False),
        TokenKind::True => Ok(Literal::True),
        TokenKind::Null => Ok(Literal::Null),
        _ => Err(ParseError::InvalidConversion(
            "could not convert literal".to_string(),
        )),
    }
}

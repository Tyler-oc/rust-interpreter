use crate::{
    errors::parse_error::ParseError,
    lexing::token::{Token, TokenKind},
    parsing::ast::{BinaryOp, Expr, Literal, LogicalOp, Stmt, UnaryOp},
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

    fn finished_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments: Vec<Expr> = Vec::new();
        if !self.check(TokenKind::RightParen) {
            loop {
                arguments.push(self.expression()?);

                if !self.match_token(vec![TokenKind::Comma]) {
                    break;
                }
                if arguments.len() >= 255 {
                    return Err(ParseError::SyntaxError("Too many arguments".to_string()));
                }
            }
        }
        let paren: &Token = self.consume(
            TokenKind::RightParen,
            "Expect ) after function call".to_string(),
        )?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren: paren.clone(),
            arguments: arguments,
        })
    }

    fn consume(&mut self, token_type: TokenKind, message: String) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(ParseError::InvalidGrouping(message))
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
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
        if self.match_token(vec![TokenKind::Identifier]) {
            return Ok(Expr::Variable(self.previous().clone()));
        }
        match parse_literal(self.peek()) {
            Ok(l) => {
                self.advance();
                Ok(Expr::Literal(l))
            }
            Err(e) => Err(e),
        }
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.primary()?;

        loop {
            if self.match_token(vec![TokenKind::LeftParen]) {
                expr = self.finished_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
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
                op: operator,
                right: Box::new(right),
            });
        }
        self.call()
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
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

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = match self.equality() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };

        while self.match_token(vec![TokenKind::And]) {
            let and: LogicalOp = match parse_logical_op(self.previous()) {
                Ok(l) => l,
                Err(err) => return Err(err),
            };
            let right: Expr = match self.equality() {
                Ok(e) => e,
                Err(err) => return Err(err),
            };

            expr = Expr::Logical {
                left: Box::new(expr),
                op: and,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = match self.and() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };

        while self.match_token(vec![TokenKind::Or]) {
            let or: LogicalOp = match parse_logical_op(self.previous()) {
                Ok(t) => t,
                Err(err) => return Err(err),
            };
            let right: Expr = match self.and() {
                Ok(e) => e,
                Err(err) => return Err(err),
            };

            expr = Expr::Logical {
                left: Box::new(expr),
                op: or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr: Expr = match self.or() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };

        if self.match_token(vec![TokenKind::Equal]) {
            let equals: Token = self.previous().clone();
            let exp: Expr = match self.or() {
                Ok(e) => e,
                Err(err) => return Err(err),
            };

            match expr {
                Expr::Variable(t) => {
                    return Ok(Expr::Assignment {
                        name: t,
                        exp: Box::new(exp),
                    });
                }
                _ => return Err(ParseError::AssignmentError(equals)),
            }
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        match self.assignment() {
            Ok(e) => Ok(e),
            Err(err) => return Err(err),
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr: Expr = match self.expression() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };
        match self.consume(TokenKind::Semicolon, "Expect ; after statement".to_string()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(Stmt::Expression(expr))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr: Expr = match self.expression() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };
        match self.consume(TokenKind::Semicolon, "Expect ; after statement".to_string()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(Stmt::Print(expr))
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        match self.consume(
            TokenKind::LeftParen,
            "Expect ( after if statement".to_string(),
        ) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        let condition = match self.expression() {
            Ok(e) => e,
            Err(err) => return Err(err),
        };
        match self.consume(
            TokenKind::RightParen,
            "Expect ( after if statement".to_string(),
        ) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let then_branch = match self.statement() {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        let mut else_branch: Option<Stmt> = None;

        if self.match_token(vec![TokenKind::Else]) {
            else_branch = match self.statement() {
                Ok(s) => Some(s),
                Err(e) => return Err(e),
            };
        }

        Ok(Stmt::If {
            condition: condition,
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        match self.consume(TokenKind::LeftParen, "Expect ( after while".to_string()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let condition = match self.expression() {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        match self.consume(TokenKind::RightParen, "Expect ) after while".to_string()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let body = match self.statement() {
            Ok(b) => b,
            Err(e) => return Err(e),
        };

        Ok(Stmt::While {
            condition: condition,
            body: Box::new(body),
        })
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(
            TokenKind::LeftParen,
            "Expect ( after for declaration".to_string(),
        )?;

        let mut initializer: Option<Stmt>;
        if self.match_token(vec![TokenKind::Semicolon]) {
            initializer = None;
        } else if self.match_token(vec![TokenKind::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition: Option<Expr> = None;
        if !self.check(TokenKind::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(
            TokenKind::Semicolon,
            "Expect ; after loop condition".to_string(),
        )?;

        let mut increment: Option<Expr> = None;
        if !self.check(TokenKind::Semicolon) {
            increment = Some(self.expression()?);
        }
        self.consume(
            TokenKind::RightParen,
            "Expect ) after for loop clauses".to_string(),
        )?;

        let mut body = self.statement()?;

        if !increment.is_none() {
            body = Stmt::Block(vec![body, Stmt::Expression(increment.unwrap())])
        }

        if condition.is_none() {
            condition = Some(Expr::Literal(Literal::True));
        }
        body = Stmt::While {
            condition: condition.unwrap(),
            body: Box::new(body),
        };

        if !initializer.is_none() {
            body = Stmt::Block(vec![initializer.unwrap(), body]);
        }

        Ok(body)
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            match self.declaration() {
                Ok(s) => statements.push(s),
                Err(e) => return Err(e),
            }
        }
        match self.consume(TokenKind::RightBrace, "Expect closing brace".to_string()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        return Ok(statements);
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(vec![TokenKind::If]) {
            return match self.if_statement() {
                Ok(s) => Ok(s),
                Err(e) => Err(e),
            };
        }
        if self.match_token(vec![TokenKind::Print]) {
            return match self.print_statement() {
                Ok(s) => Ok(s),
                Err(e) => Err(e),
            };
        }
        if self.match_token(vec![TokenKind::While]) {
            return match self.while_statement() {
                Ok(s) => Ok(s),
                Err(e) => Err(e),
            };
        }
        if self.match_token(vec![TokenKind::For]) {
            return match self.for_statement() {
                Ok(s) => Ok(s),
                Err(e) => Err(e),
            };
        }
        if self.match_token(vec![TokenKind::LeftBrace]) {
            return match self.block() {
                Ok(statements) => Ok(Stmt::Block(statements)),
                Err(e) => Err(e),
            };
        }
        self.expression_statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name: Token = match self.consume(
            TokenKind::Identifier,
            "Expect identifier after var declaration".to_string(),
        ) {
            Ok(t) => t.clone(),
            Err(e) => return Err(e),
        };

        let mut initializer: Option<Expr> = None;
        if self.match_token(vec![TokenKind::Equal]) {
            initializer = match self.expression() {
                Ok(e) => Some(e),
                Err(err) => return Err(err),
            };
        }

        match self.consume(
            TokenKind::Semicolon,
            "Expect ; after declaration".to_string(),
        ) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(Stmt::Var {
            name: name,
            initializer: initializer,
        })
    }

    fn fun_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name: Token = match self.consume(
            TokenKind::LeftParen,
            "Expect ( after function declaration".to_string(),
        ) {
            Ok(t) => t.clone(),
            Err(e) => return Err(e),
        };

        let mut params: Vec<Token> = Vec::new();
        if !self.check(TokenKind::RightParen) {
            loop {
                if (params.len() >= 255) {
                    return Err(ParseError::InvalidDeclaration(
                        "Too many params".to_string(),
                    ));
                }

                let t = match self
                    .consume(TokenKind::Identifier, "Expect parameter name".to_string())
                {
                    Ok(t) => t.clone(),
                    Err(e) => return Err(e),
                };

                params.push(t);

                if !self.match_token(vec![TokenKind::Comma]) {
                    break;
                }
            }
            self.consume(
                TokenKind::RightParen,
                "Expect ) after function delcaration".to_string(),
            )?;
        }

        self.consume(
            TokenKind::LeftBrace,
            "Expect { when declaring function body".to_string(),
        )?;
        let body: Vec<Stmt> = self.block()?;

        Ok(Stmt::Fun {
            name: name,
            params: params,
            body: body,
        })
    }

    pub fn declaration(&mut self) -> Result<Stmt, ParseError> {
        //huge note here that this stops errors from being thrown and just evaluates what it can.
        //flaw but useful right now for checking how the program works.
        if self.match_token(vec![TokenKind::Var]) {
            match self.var_declaration() {
                Ok(stmt) => return Ok(stmt),
                Err(e) => {
                    self.synchronize();
                    return Err(e);
                }
            }
        }
        if self.match_token(vec![TokenKind::Fun]) {
            match self.fun_declaration() {
                Ok(stmt) => return Ok(stmt),
                Err(e) => {
                    self.synchronize();
                    return Err(e);
                }
            }
        }
        match self.statement() {
            Ok(stmt) => return Ok(stmt),
            Err(e) => {
                self.synchronize();
                return Err(e);
            }
        }
    }
}

pub fn parse_tokens(tokens: &Vec<Token>) -> Result<Vec<Stmt>, ParseError> {
    let mut parser: Parser = Parser::new(tokens);
    let mut statements: Vec<Stmt> = Vec::new();
    let mut errors: Vec<ParseError> = Vec::new();

    loop {
        if parser.is_at_end() {
            break;
        }

        match parser.declaration() {
            Ok(stmt) => statements.push(stmt),
            Err(e) => errors.push(e),
        };
    }

    Ok(statements)
}

pub fn parse_binary_op(token: &Token) -> Result<BinaryOp, ParseError> {
    match token.kind {
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

pub fn parse_logical_op(token: &Token) -> Result<LogicalOp, ParseError> {
    match token.kind {
        TokenKind::And => Ok(LogicalOp::And),
        TokenKind::Or => Ok(LogicalOp::Or),
        _ => Err(ParseError::InvalidConversion(
            "could not convert to logical operator".to_string(),
        )),
    }
}

pub fn parse_literal(token: Token) -> Result<Literal, ParseError> {
    match token.kind {
        TokenKind::Number => match token.literal {
            Some(l) => Ok(l.clone()),
            None => Err(ParseError::MissingValue {
                val: token.lexeme.clone(),
                line: token.line,
            }),
        },
        TokenKind::StringLiteral => match token.literal {
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

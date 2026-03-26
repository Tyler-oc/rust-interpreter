use std::collections::HashMap;

use crate::{
    errors::resolver_error::ResolverError,
    interpreting::interpreter::Interpreter,
    lexing::token::Token,
    parsing::ast::{BinaryOp, Expr, Literal, LogicalOp, Stmt, UnaryOp},
};

struct Resolver<'a> {
    interpreter: &'a Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    pub fn new(&mut self, interpreter: &'a Interpreter) -> Self {
        Resolver {
            interpreter: interpreter,
            scopes: Vec::new(),
        }
    }

    fn declare(&mut self, name: &Token) -> Result<(), ResolverError> {
        if self.scopes.len() == 0 {
            return Ok(());
        }

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), false);
            Ok(())
        } else {
            Err(ResolverError::ScopingError(name.lexeme.clone()))
        }
    }

    fn define(&mut self, name: &Token) -> Result<(), ResolverError> {
        if self.scopes.len() == 0 {
            return Ok(());
        }

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
            Ok(())
        } else {
            Err(ResolverError::ScopingError(name.lexeme.clone()))
        }
    }

    fn resolve_local(&mut self, _expr: &Expr, name: &Token) -> Result<(), ResolverError> {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(name, self.scopes.len() - 1 - i);
                return Ok(());
            }
        }
        Ok(())
    }

    fn visit_variable_expr(&mut self, expr: &Expr, name: &Token) -> Result<(), ResolverError> {
        if let Some(scope) = self.scopes.last() {
            if scope.get(&name.lexeme) == Some(&false) {
                return Err(ResolverError::ScopingError(format!(
                    "Cannot read local var in it's own initializer: {}",
                    expr.clone().to_string()
                )));
            }
        }
        self.resolve_local(expr, name);
        Ok(())
    }

    fn visit_assign_expr(
        &mut self,
        name: &Token,
        value: &Expr,
        original_expr: &Expr,
    ) -> Result<(), ResolverError> {
        self.resolve_expr(value)?;
        self.resolve_local(original_expr, name)?;
        Ok(())
    }

    fn resolve_var_stmt(
        &mut self,
        name: &Token,
        initializer: &Option<Expr>,
    ) -> Result<(), ResolverError> {
        self.declare(name)?;
        match initializer {
            Some(i) => self.resolve_expr(i)?,
            None => (),
        }
        self.define(name)?;
        Ok(())
    }

    fn visit_function_stmt(
        &mut self,
        name: &Token,
        params: &Vec<Token>,
        body: &Vec<Stmt>,
    ) -> Result<(), ResolverError> {
        self.declare(name)?;
        self.define(name)?;

        self.resolve_function_stmt(params, body)?;
        Ok(())
    }

    fn resolve_function_stmt(
        &mut self,
        params: &Vec<Token>,
        body: &Vec<Stmt>,
    ) -> Result<(), ResolverError> {
        self.begin_scope()?;
        for param in params.iter() {
            self.declare(param)?;
            self.define(param)?;
        }
        self.resolve_stmts(body)?;
        self.end_scope()?;
        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Box<Stmt>,
        else_branch: &Box<Option<Stmt>>,
    ) -> Result<(), ResolverError> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(then_branch)?;
        if let Some(else_stmt) = else_branch.as_ref() {
            self.resolve_stmt(else_stmt)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), ResolverError> {
        self.resolve_expr(&expr)?;
        Ok(())
    }

    fn visit_return_stmt(
        &mut self,
        _keyword: &Token,
        value: &Option<Expr>,
    ) -> Result<(), ResolverError> {
        if let Some(v) = value.as_ref() {
            self.resolve_expr(v)?;
        }

        Ok(())
    }

    fn visit_while_stmt(
        &mut self,
        condition: &Expr,
        body: &Box<Stmt>,
    ) -> Result<(), ResolverError> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(body)?;
        Ok(())
    }

    fn visit_binary_expr(
        &mut self,
        left: &Box<Expr>,
        _op: &BinaryOp,
        right: &Box<Expr>,
    ) -> Result<(), ResolverError> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;

        Ok(())
    }

    fn visit_call_expr(
        &mut self,
        callee: &Box<Expr>,
        _paren: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<(), ResolverError> {
        self.resolve_expr(callee)?;
        for argument in arguments.iter() {
            self.resolve_expr(argument)?;
        }

        Ok(())
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<(), ResolverError> {
        self.resolve_expr(expr)?;
        Ok(())
    }

    fn visit_literal_expr(&mut self, _l: &Literal) -> Result<(), ResolverError> {
        Ok(())
    }

    fn visit_logical_expr(
        &mut self,
        left: &Box<Expr>,
        _op: &LogicalOp,
        right: &Box<Expr>,
    ) -> Result<(), ResolverError> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;

        Ok(())
    }

    fn visit_unary_expr(&mut self, _op: &UnaryOp, right: &Box<Expr>) -> Result<(), ResolverError> {
        self.resolve_expr(right)?;

        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), ResolverError> {
        match expr {
            Expr::Assignment { name, exp } => self.visit_assign_expr(name, exp, expr),
            Expr::Binary { left, op, right } => self.visit_binary_expr(&left, &op, &right),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => self.visit_call_expr(&callee, &paren, &arguments),
            Expr::Grouping { exp } => self.visit_grouping_expr(&exp),
            Expr::Literal(l) => self.visit_literal_expr(l),
            Expr::Logical { left, op, right } => self.visit_logical_expr(&left, &op, &right),
            Expr::Unary { op, right } => self.visit_unary_expr(&op, &right),
            Expr::Variable(t) => self.visit_variable_expr(expr, &t),
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), ResolverError> {
        match stmt {
            Stmt::Block(stmts) => self.resolve_stmts(stmts),
            Stmt::Expression(expr) => self.resolve_expr(expr),
            Stmt::Fun { name, params, body } => self.visit_function_stmt(name, params, body),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => self.visit_if_stmt(condition, then_branch, else_branch),
            Stmt::Print(expr) => self.visit_print_stmt(expr),
            Stmt::Return { keyword, value } => self.visit_return_stmt(keyword, value),
            Stmt::Var { name, initializer } => self.resolve_var_stmt(name, initializer),
            Stmt::While { condition, body } => self.visit_while_stmt(condition, body),
        }
    }

    fn resolve_stmts(&mut self, stmts: &Vec<Stmt>) -> Result<(), ResolverError> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn begin_scope(&mut self) -> Result<(), ResolverError> {
        let new_map: HashMap<String, bool> = HashMap::new();
        self.scopes.push(new_map);
        Ok(())
    }

    fn end_scope(&mut self) -> Result<(), ResolverError> {
        self.scopes.pop();
        Ok(())
    }

    pub fn resolve_block(&mut self, block: &Stmt) -> Result<(), ResolverError> {
        let stmt_vec = match block {
            Stmt::Block(vec) => vec,
            _ => {
                return Err(ResolverError::ScopingError(
                    "Block statement not found".to_string(),
                ));
            }
        };
        self.begin_scope()?;
        self.resolve_stmts(stmt_vec)?;
        self.end_scope()?;
        Ok(())
    }
}

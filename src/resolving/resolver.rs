use std::collections::HashMap;

use crate::{
    errors::resolver_error::ResolverError,
    interpreting::interpreter::Interpreter,
    lexing::token::Token,
    parsing::ast::{Expr, Stmt},
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

    fn declare(&mut self, name: Token) -> Result<(), ResolverError> {
        if self.scopes.len() == 0 {
            return Ok(());
        }

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme, false);
            Ok(())
        } else {
            Err(ResolverError::ScopingError(name.lexeme))
        }
    }

    fn define(&mut self, name: Token) -> Result<(), ResolverError> {
        if self.scopes.len() == 0 {
            return Ok(());
        }

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme, true);
            Ok(())
        } else {
            Err(ResolverError::ScopingError(name.lexeme))
        }
    }

    fn resolve_local(&mut self, expr: Expr, name: Token) -> Result<(), ResolverError> {}

    fn visit_variable_expr(&mut self, expr: Expr, name: Token) -> Result<(), ResolverError> {
        if let Some(scope) = self.scopes.last() {
            if scope.get(&name.lexeme) == Some(&false) {
                return Err(ResolverError::ScopingError(expr.to_string()));
            }
        }
        self.resolve_local(expr, name);
        Ok(())
    }

    fn resolve_var(&mut self, name: Token, initializer: Option<Expr>) -> Result<(), ResolverError> {
        self.declare(name);
        match initializer {
            Some(i) => self.resolve_expr(i),
            None => (),
        }
        self.define(name);
        Ok(())
    }

    fn begin_scope(&mut self) -> Result<(), ResolverError> {
        let new_map: HashMap<String, bool> = HashMap::new();
        self.scopes.push(new_map);
        Ok(())
    }

    fn resolve(&mut self, stmts: Vec<Stmt>) -> Result<(), ResolverError> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression(e) => self.resolve_expr(e),
                Stmt::Var { name, initializer } => self.resolve_var(name, initializer),
                _ => (),
            }
        }
        Ok(())
    }

    fn end_scope(&mut self) -> Result<(), ResolverError> {
        self.scopes.pop();
        Ok(())
    }

    pub fn resolve_block(&mut self, block: &Stmt) -> Result<(), ResolverError> {
        self.begin_scope()?;
        self.resolve()?;
        self.end_scope()?;
        Ok(())
    }
}

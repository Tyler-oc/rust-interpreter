use std::{cell::RefCell, collections::HashMap, rc::Rc};

use regex::CaptureNames;

use crate::{
    WrappedEnv,
    environment::environment::Environment,
    errors::{environment_error::EnvironmentError, runtime_error::RunTimeError},
    interpreting::value::Value,
    lexing::token::Token,
    parsing::ast::{BinaryOp, Expr, Literal, LogicalOp, Stmt, UnaryOp},
};

struct Interpreter {
    environment: WrappedEnv,
}

impl Interpreter {
    pub fn new(environment: WrappedEnv) -> Self {
        Interpreter {
            environment: environment,
        }
    }

    fn eval_literal(&mut self, literal: &Literal) -> Result<Value, RunTimeError> {
        match literal {
            Literal::Number(n) => Ok(Value::Number(*n)),
            Literal::StringLiteral(s) => Ok(Value::String(s.clone())),
            Literal::True => Ok(Value::Boolean(true)),
            Literal::False => Ok(Value::Boolean(false)),
            Literal::Null => Ok(Value::Null),
            _ => Err(RunTimeError::CouldNotEval(literal.to_string())), //just in case even though above is exhausting match
        }
    }

    fn eval_grouping(&mut self, expr: &Expr) -> Result<Value, RunTimeError> {
        match expr {
            Expr::Grouping { exp } => self.evaluate(exp),
            _ => return Err(RunTimeError::CouldNotEval(expr.to_string())),
        }
    }

    fn is_equal(&mut self, v1: Value, v2: Value) -> Result<bool, RunTimeError> {
        match (v1, v2) {
            (Value::Number(n1), Value::Number(n2)) => Ok(n1 == n2),
            (Value::String(s1), Value::String(s2)) => Ok(s1 == s2),
            (Value::Boolean(b1), Value::Boolean(b2)) => Ok(b1 == b2),
            (_, _) => Err(RunTimeError::CouldNotEval("==".to_string())),
        }
    }

    fn eval_assignment(&mut self, name: &Token, exp: &Expr) -> Result<Value, RunTimeError> {
        let value: Value = match self.evaluate(exp) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        self.environment
            .borrow_mut()
            .assign(&name.lexeme, value.clone())?;
        Ok(value)
    }

    fn eval_var(&mut self, name: &Token) -> Result<Value, RunTimeError> {
        match self.environment.borrow_mut().get(&name.lexeme) {
            Ok(val) => Ok(val),
            Err(e) => Err(RunTimeError::EnvironmentError(e)),
        }
    }

    fn eval_binary(
        &mut self,
        left: &Expr,
        op: &BinaryOp,
        right: &Expr,
    ) -> Result<Value, RunTimeError> {
        let left = match self.evaluate(left) {
            Ok(left) => left,
            Err(e) => return Err(e),
        };
        let right = match self.evaluate(right) {
            Ok(right) => right,
            Err(e) => return Err(e),
        };
        match op {
            BinaryOp::Minus => match (left, right) {
                (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
                (_, _) => Err(RunTimeError::CouldNotEval("minus".to_string())),
            },
            BinaryOp::Slash => match (left, right) {
                (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 / n2)),
                (_, _) => Err(RunTimeError::CouldNotEval("slash".to_string())),
            },
            BinaryOp::Star => match (left, right) {
                (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
                (_, _) => Err(RunTimeError::CouldNotEval("star".to_string())),
            },
            BinaryOp::Plus => match (left, right) {
                //can add additional conversions and abilities in this later
                (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
                (Value::String(s1), Value::String(s2)) => Ok(Value::String(s1 + &s2)),
                (_, _) => Err(RunTimeError::CouldNotEval("plus".to_string())),
            },
            BinaryOp::GreaterThan => match (left, right) {
                (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 > n2)),
                (_, _) => Err(RunTimeError::CouldNotEval(">".to_string())),
            },
            BinaryOp::GreaterEqual => match (left, right) {
                (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 >= n2)),
                (_, _) => Err(RunTimeError::CouldNotEval(">=".to_string())),
            },
            BinaryOp::LessThan => match (left, right) {
                (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 < n2)),
                (_, _) => Err(RunTimeError::CouldNotEval("<".to_string())),
            },
            BinaryOp::LessEqual => match (left, right) {
                (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 <= n2)),
                (_, _) => Err(RunTimeError::CouldNotEval(">".to_string())),
            },
            BinaryOp::EqualEqual => match self.is_equal(left, right) {
                Ok(b) => Ok(Value::Boolean(b)),
                Err(e) => Err(e),
            },
            BinaryOp::BangEqual => match self.is_equal(left, right) {
                Ok(b) => Ok(Value::Boolean(!b)),
                Err(e) => Err(e),
            },
            _ => Err(RunTimeError::CouldNotEval("Operator not found".to_string())),
        }
    }

    fn eval_unary(&mut self, op: &UnaryOp, right: &Expr) -> Result<Value, RunTimeError> {
        let right = match self.evaluate(right) {
            Ok(right) => right,
            Err(e) => return Err(e),
        };

        match op {
            UnaryOp::Minus => match right {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(RunTimeError::CouldNotEval("- unary".to_string())),
            },
            UnaryOp::Bang => Ok(Value::Boolean(!self.is_truthy(&right))),
        }
    }

    fn eval_logical(
        &mut self,
        left: &Expr,
        op: &LogicalOp,
        right: &Expr,
    ) -> Result<Value, RunTimeError> {
        let left_val = match self.evaluate(left) {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        match op {
            LogicalOp::Or => {
                if self.is_truthy(&left_val) {
                    return Ok(left_val);
                }
            }
            _ => (),
        };
        if !self.is_truthy(&left_val) {
            return Ok(left_val);
        }

        self.evaluate(right)
    }

    fn eval_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Stmt>,
    ) -> Result<(), RunTimeError> {
        let condition = match self.evaluate(condition) {
            Ok(e) => e,
            Err(err) => return Err(err),
        };
        if self.is_truthy(&condition) {
            let _ = match self.execute(&then_branch) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        } else {
            let _ = match else_branch {
                Some(else_branch) => match self.execute(&else_branch) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                },
                None => (),
            };
        }
        Ok(())
    }

    fn eval_while(&mut self, condition: &Expr, body: &Stmt) -> Result<(), RunTimeError> {
        loop {
            let val = self.evaluate(condition)?;

            if !self.is_truthy(&val) {
                break;
            }

            self.execute(body)?;
        }
        Ok(())
    }

    fn eval_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: WrappedEnv,
    ) -> Result<(), RunTimeError> {
        let previous = Rc::clone(&self.environment);

        self.environment = environment;

        let result: Result<(), RunTimeError> = (|| {
            for statement in statements {
                self.execute(statement)?;
            }
            Ok(())
        })();
        self.environment = previous;
        result
    }

    pub fn evaluate(&mut self, exp: &Expr) -> Result<Value, RunTimeError> {
        match exp {
            Expr::Binary { left, op, right } => match self.eval_binary(left, op, right) {
                Ok(val) => Ok(val),
                Err(e) => return Err(e),
            },
            Expr::Unary { op, right } => match self.eval_unary(op, right) {
                Ok(val) => Ok(val),
                Err(e) => return Err(e),
            },
            Expr::Logical { left, op, right } => match self.eval_logical(left, op, right) {
                Ok(val) => Ok(val),
                Err(e) => Err(e),
            },
            Expr::Grouping { exp } => match self.eval_grouping(exp) {
                Ok(val) => Ok(val),
                Err(e) => return Err(e),
            },
            Expr::Literal(literal) => match self.eval_literal(literal) {
                Ok(val) => Ok(val),
                Err(e) => return Err(e),
            },
            Expr::Variable(name) => match self.eval_var(name) {
                Ok(val) => Ok(val),
                Err(e) => return Err(e),
            },
            Expr::Assignment { name, exp } => match self.eval_assignment(name, exp) {
                Ok(val) => Ok(val),
                Err(e) => return Err(e),
            },
            _ => Err(RunTimeError::CouldNotEval(exp.to_string())),
        }
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), RunTimeError> {
        match stmt {
            Stmt::Expression(e) => match self.evaluate(e) {
                Ok(e) => println!("{}", e), //for testing don't acutally print though in practice
                Err(err) => return Err(err),
            },
            Stmt::Print(e) => match self.evaluate(e) {
                Ok(e) => println!("{}", e),
                Err(err) => return Err(err),
            },
            Stmt::Block(statements) => {
                let env_new = Rc::new(RefCell::new(Environment {
                    values: HashMap::new(),
                    outer_environment: Some(Rc::clone(&self.environment)),
                }));
                match self.eval_block(statements, env_new) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => match self.eval_if(condition, then_branch, else_branch) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            Stmt::While { condition, body } => match self.eval_while(condition, body) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            Stmt::For {
                initializer: _initializer,
                condition: _condition,
                increment: _increment,
                body: _body,
            } => {
                return Err(RunTimeError::CouldNotEval(
                    "For loop not restructured correctly".to_string(),
                ));
            }
            Stmt::Var { name, initializer } => {
                let val;
                match initializer {
                    Some(initializer) => {
                        val = match self.evaluate(initializer) {
                            Ok(v) => Some(v),
                            Err(err) => return Err(err),
                        };
                    }
                    None => val = Some(Value::Null),
                }

                match val {
                    Some(v) => self
                        .environment
                        .borrow_mut()
                        .define(name.lexeme.to_string(), v)?,
                    None => {
                        return Err(RunTimeError::EnvironmentError(
                            EnvironmentError::UndefinedVariable(name.lexeme.to_string()),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn is_truthy(&mut self, val: &Value) -> bool {
        match val {
            Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Number(0.0) => false,
            _ => true,
        }
    }
}

pub fn interpret(statements: Vec<Stmt>) -> Result<(), RunTimeError> {
    let environment = Rc::new(RefCell::new(Environment::new(None)));
    let mut interpreter: Interpreter = Interpreter::new(environment);

    for statement in statements.iter() {
        match interpreter.execute(statement) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

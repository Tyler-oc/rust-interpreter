use std::{cell::RefCell, fmt::write, rc::Rc};

use crate::{
    WrappedEnv,
    environment::environment::Environment,
    errors::runtime_error::RunTimeError,
    interpreting::{interpreter::Interpreter, value::Value},
    parsing::ast::Stmt,
};

#[derive(Debug, Clone)]
pub enum Callable {
    Function {
        declaration: Stmt,
        closure: WrappedEnv,
    },
    Native {
        arity: usize,
        body: fn(Vec<Value>) -> Value,
    },
}

impl std::fmt::Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Callable::Function {
                declaration,
                closure: _closure,
            } => write!(f, "<fn {}>", declaration),
            Callable::Native { arity, body: _body } => {
                write!(f, "Native fn with arity {}", arity)
            }
        }
    }
}

impl Callable {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, RunTimeError> {
        return match self {
            Callable::Function {
                declaration,
                closure,
            } => {
                let environment: WrappedEnv =
                    Rc::new(RefCell::new(Environment::new(Some(Rc::clone(closure)))));
                if let Stmt::Fun { name, params, body } = declaration {
                    for (i, param) in params.iter().enumerate() {
                        let arg = args.get(i).ok_or_else(|| {
                            RunTimeError::CallableError("not enough arguments".to_string())
                        })?;

                        environment
                            .borrow_mut()
                            .define(param.lexeme.clone(), arg.clone());
                    }

                    interpreter.eval_block(body, environment)?;
                }

                Ok(Value::Null)
            }
            _ => Ok(Value::Null),
        };
    }

    pub fn arity(&self) -> usize {
        match self {
            Callable::Function {
                declaration,
                closure,
            } => {
                if let Stmt::Fun { name, params, body } = declaration {
                    return params.len();
                }
                return 0;
            }
            Callable::Native { arity, body } => return *arity,
        };
    }
}

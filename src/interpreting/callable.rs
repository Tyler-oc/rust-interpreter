use std::fmt::write;

use crate::{
    WrappedEnv,
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
            } => write!(f, "Declaring function: {}", declaration),
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
        Ok(Value::Null)
    }

    pub fn arity() {}
}

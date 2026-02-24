use std::fmt::write;

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
        return match self {
            Callable::Function {
                declaration,
                closure,
            } => {
                let environment: WrappedEnv = interpreter.globals;
                let params = match declaration {
                    Stmt::Fun { name, params, body } => params,
                    _ => return Err(RunTimeError::CallableError("Params not found".to_string())),
                };
                let declaration_body = match declaration {
                    Stmt::Block(b) => b,
                    _ => {
                        return Err(RunTimeError::CallableError(
                            "Declaration body error".to_string(),
                        ));
                    }
                };

                let params_len = params.len();

                for number in 0..params_len {
                    environment
                        .borrow_mut()
                        .define(params.get(number)?.lexeme, args.get(number)?.clone())?;
                }
                interpreter.eval_block(declaration_body, environment)
                Ok(Value::Null)
            }
            _ => Ok(Value::Null)
        }
    }

    pub fn arity() {}
}

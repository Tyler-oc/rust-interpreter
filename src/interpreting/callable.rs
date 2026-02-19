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
}

impl std::fmt::Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Callable::Function {
                declaration,
                closure: _closure,
            } => write!(f, "Declaring function: {}", declaration),
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

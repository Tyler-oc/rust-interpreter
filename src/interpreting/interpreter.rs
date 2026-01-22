use crate::{errors::interpreter_error::InterpreterError, interpreting::value::Value, parsing::ast::Expr};

struct Interpreter<'a> {
    expr: Expr,
}

impl<'a> Interpreter<'a> {
    fn literal(&mut self) -> Result<Value, InterpreterError> {
        
    }

    fn is_truthy(&mut self, )
}

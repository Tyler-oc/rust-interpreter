use crate::{
    errors::runtime_error::RunTimeError,
    interpreting::value::Value,
    parsing::ast::{BinaryOp, Expr, Literal, UnaryOp},
};

struct Interpreter<'a> {
    expr: Expr,
}

impl<'a> Interpreter<'a> {
    fn literal(&mut self, literal: Literal) -> Result<Value, RunTimeError> {
        match literal {
            Literal::Number(n) => Ok(Value::Number(n)),
            Literal::StringLiteral(s) => Ok(Value::String(s)),
            Literal::True => Ok(Value::Boolean(true)),
            Literal::False => Ok(Value::Boolean(false)),
            Literal::Null => Ok(Value::Null),
            _ => Err(RunTimeError::CouldNotEval(literal.to_string())),
        }
    }

    fn grouping(&mut self, Expr::Grouping { exp }: crate::Expr) -> Result<Value, RunTimeError> {
        self.evaluate(exp)
    }

    fn eval_binary(&mut self, Expr::Binary { left, op, right }: Binary) -> Result<Value, RunTimeError> {
        let left = match self.evaluate(*left) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        let right = match self.evaluate(*right) {
            Ok(val) => val,
            Err(e) => return Err(e)        
        };
        //maybe go other way and match left and right types then define possible operations
        match op {
            BinaryOp::Minus => Ok(right - left),
            BinaryOp::Slash => Ok(right / left),
            BinaryOp::Star => Ok(right * left),
            BinaryOp::Plus => ,

        }
    }

    fn eval_unary(
        &mut self,
        Expr::Unary { exp, op }: crate::Expr,
    ) -> Result<Value, RunTimeError> {
        let right = match self.evaluate(*exp) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        match op {
            UnaryOp::Minus => Ok(Value::Number(-right)),
            UnaryOp::Bang => Ok(Value::Boolean(!self.is_truthy(right))),
        }
    }

    pub fn evaluate(&mut self, exp: Expr) -> Result<Value, RunTimeError> {
        Ok(Value::Number(1.0))
    }

    fn is_truthy(&mut self, val: Value) -> bool {
        match val {
            Value::Null => false,
            Value::Boolean(b) => b,
            Value::Number(0.0) => false,
            _ => true,
        }
    }
}

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

    fn is_equal(&mut self, v1: Value, v2: Value) -> Result<bool, RunTimeError> {
        match (v1, v2) {
            (Value::Number(n1), Value::Number(n2)) => Ok(n1 == n2),
            (Value::String(s1), Value::String(s2)) => Ok(s1 == s2),
            (Value::Boolean(b1), Value::Boolean(b2)) => Ok(b1 == b2),
            (_, _) => Err(RunTimeError::CouldNotEval("==".to_string())),
        }
    }

    fn eval_binary(
        &mut self,
        Expr::Binary { left, op, right }: Binary,
    ) -> Result<Value, RunTimeError> {
        let left = match self.evaluate(*left) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        let right = match self.evaluate(*right) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        //maybe go other way and match left and right types then define possible operations
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

    fn eval_unary(&mut self, Expr::Unary { exp, op }: crate::Expr) -> Result<Value, RunTimeError> {
        let right = match self.evaluate(*exp) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        match op {
            UnaryOp::Minus => match right {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(RunTimeError::CouldNotEval("- unary".to_string())),
            },
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

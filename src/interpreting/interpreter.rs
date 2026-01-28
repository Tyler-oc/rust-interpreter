use crate::{
    errors::runtime_error::RunTimeError,
    interpreting::value::Value,
    parsing::ast::{BinaryOp, Expr, Literal, Stmt, UnaryOp},
};

struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    fn eval_literal(&mut self, literal: Literal) -> Result<Value, RunTimeError> {
        match literal {
            Literal::Number(n) => Ok(Value::Number(n)),
            Literal::StringLiteral(s) => Ok(Value::String(s)),
            Literal::True => Ok(Value::Boolean(true)),
            Literal::False => Ok(Value::Boolean(false)),
            Literal::Null => Ok(Value::Null),
            _ => Err(RunTimeError::CouldNotEval(literal.to_string())), //just in case even though above is exhausting match
        }
    }

    fn eval_grouping(&mut self, expr: Expr) -> Result<Value, RunTimeError> {
        match expr {
            Expr::Grouping { exp } => self.evaluate(*exp),
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

    fn eval_binary(
        &mut self,
        left: Expr,
        op: BinaryOp,
        right: Expr,
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

    fn eval_unary(&mut self, op: UnaryOp, right: Expr) -> Result<Value, RunTimeError> {
        let right = match self.evaluate(right) {
            Ok(right) => right,
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
        match exp {
            Expr::Binary { left, op, right } => match self.eval_binary(*left, op, *right) {
                Ok(val) => Ok(val),
                Err(e) => return Err(e),
            },
            Expr::Unary { op, right } => match self.eval_unary(op, *right) {
                Ok(val) => Ok(val),
                Err(e) => return Err(e),
            },
            Expr::Grouping { exp } => match self.eval_grouping(*exp) {
                Ok(val) => Ok(val),
                Err(e) => return Err(e),
            },
            Expr::Literal(literal) => match self.eval_literal(literal) {
                Ok(val) => Ok(val),
                Err(e) => return Err(e),
            },
            _ => Err(RunTimeError::CouldNotEval(exp.to_string())),
        }
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), RunTimeError> {
        match stmt {
            Stmt::Expression(e) => match self.evaluate(e.clone()) {
                Ok(e) => println!("{}", e), //for testing don't acutally print though in practice
                Err(err) => return Err(err),
            },
            Stmt::Print(e) => match self.evaluate(e.clone()) {
                Ok(e) => println!("{}", e),
                Err(err) => return Err(err),
            },
        }
        Ok(())
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

pub fn interpret(statements: Vec<Stmt>) -> Result<(), RunTimeError> {
    let mut interpreter: Interpreter = Interpreter::new();

    for statement in statements.iter() {
        match interpreter.execute(statement) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

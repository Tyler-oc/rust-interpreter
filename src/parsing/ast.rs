use crate::{WrappedEnv, lexing::token::Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    For {
        initializer: Box<Stmt>,
        condition: Box<Stmt>,
        increment: Box<Stmt>,
        body: Box<Stmt>,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Expr {
    //remember to change Box to Rc (shared ownership) or Arc (multi thread)
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Logical {
        left: Box<Expr>,
        op: LogicalOp,
        right: Box<Expr>,
    },
    Grouping {
        exp: Box<Expr>,
    },
    Assignment {
        name: Token,
        exp: Box<Expr>,
    },
    Literal(Literal),
    Variable(Token),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    GreaterEqual,
    GreaterThan,
    EqualEqual,
    BangEqual,
    LessEqual,
    LessThan,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Bang,
    Minus,
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    StringLiteral(String),
    Null,
    True,
    False,
}

//display implementations for enums

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expression(e) => write!(f, "{}", e), //normally don't display anything but nice for testing
            Stmt::Print(e) => write!(f, "{}", e),
            Stmt::Block(s) => {
                let mut output = String::new();
                for statement in s.iter() {
                    output.push_str(&statement.to_string());
                }
                write!(f, "{}", output)
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => match &**else_branch {
                Some(else_branch) => write!(
                    f,
                    "if {} then {} else {}",
                    condition, then_branch, else_branch
                ),
                None => write!(f, "if {} then {}", condition, then_branch),
            },
            Stmt::While { condition, body } => {
                write!(f, "While {} do {}", condition, body)
            }
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                write!(
                    f,
                    "For {} {} {} do {}",
                    initializer, condition, increment, body
                )
            }
            Stmt::Var { name, initializer } => match initializer {
                Some(initializer) => {
                    write!(f, "variable {} with value {}", name.lexeme, initializer)
                }
                None => write!(f, "variable {} with no assigned value", name.lexeme),
            },
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary { left, op, right } => {
                write!(f, "({}{}{})", left, op, right)
            }
            Expr::Unary { op, right } => {
                write!(f, "({}{})", op, right)
            }
            Expr::Call {
                callee,
                paren: _paren,
                arguments,
            } => {
                write!(
                    f,
                    "calle {} with arguments {}",
                    callee,
                    arguments
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Expr::Logical { left, op, right } => {
                write!(f, "{}{}{}", left, op, right)
            }
            Expr::Grouping { exp } => {
                write!(f, "(group {})", exp)
            }
            Expr::Assignment { name, exp } => {
                write!(f, "({} -> {})", name.lexeme, exp)
            }
            Expr::Literal(val) => {
                write!(f, "{}", val)
            }
            Expr::Variable(t) => {
                write!(f, "({})", t.lexeme)
            }
        }
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BinaryOp::Plus => "+",
            BinaryOp::Minus => "-",
            BinaryOp::Star => "*",
            BinaryOp::Slash => "/",
            BinaryOp::Equal => "=",
            BinaryOp::GreaterEqual => ">=",
            BinaryOp::GreaterThan => ">",
            BinaryOp::EqualEqual => "==",
            BinaryOp::BangEqual => "!=",
            BinaryOp::LessEqual => "<=",
            BinaryOp::LessThan => "<",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UnaryOp::Bang => "!",
            UnaryOp::Minus => "-",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for LogicalOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LogicalOp::And => "&&",
            LogicalOp::Or => "||",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Literal::Number(val) => val.to_string(),
            Literal::StringLiteral(val) => val.to_string(),
            Literal::Null => "NULL".to_string(),
            Literal::True => "true".to_string(),
            Literal::False => "false".to_string(),
        };
        write!(f, "{}", s)
    }
}

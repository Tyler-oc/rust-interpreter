#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
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
    Grouping {
        exp: Box<Expr>,
    },
    Literal(Literal),
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
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Bang,
    Minus,
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
            Stmt::Expression(e) => write!(f, "{}", e),
            Stmt::Print(e) => write!(f, "{}", e),
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
            Expr::Grouping { exp } => {
                write!(f, "(group {})", exp)
            }
            Expr::Literal(val) => {
                write!(f, "{}", val)
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
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
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

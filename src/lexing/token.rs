use crate::errors::lex_error::LexError;
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    //keywords
    Print,
    If,
    Else,
    For,
    While,
    Null,
    And,
    Or,
    Fun,
    Class,
    Super,
    Return,
    This,

    //literal
    Number(f64),
    StringLiteral(String),
    True,
    False,

    //identifiers
    Identifier(String),
    Var,

    //operators
    Plus,
    Minus,
    Dot, //property calls
    Slash,
    Star,
    Equal,
    Comma,

    //punctuation
    Semicolon,
    LeftParen,
    RightParen,
    RightBrace,
    LeftBrace,

    //logical operator
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Bang,
    BangEqual,
    EqualEqual,

    EOF,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // keywords
            Token::Print => write!(f, "print"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::For => write!(f, "for"),
            Token::While => write!(f, "while"),
            Token::Null => write!(f, "null"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::Fun => write!(f, "fun"),
            Token::Class => write!(f, "class"),
            Token::Super => write!(f, "super"),
            Token::Return => write!(f, "return"),
            Token::This => write!(f, "this"),
            Token::Var => write!(f, "var"),

            // literals
            Token::Number(n) => write!(f, "{}", n),
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),

            // identifiers
            Token::Identifier(name) => write!(f, "{}", name),

            // operators
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Dot => write!(f, "."),
            Token::Slash => write!(f, "/"),
            Token::Star => write!(f, "*"),
            Token::Equal => write!(f, "="),
            Token::Comma => write!(f, ","),

            // punctuation
            Token::Semicolon => write!(f, ";"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),

            // logical / comparison operators
            Token::GreaterThan => write!(f, ">"),
            Token::LessThan => write!(f, "<"),
            Token::GreaterEqual => write!(f, ">="),
            Token::LessEqual => write!(f, "<="),
            Token::Bang => write!(f, "!"),
            Token::BangEqual => write!(f, "!="),
            Token::EqualEqual => write!(f, "=="),

            Token::EOF => write!(f, "EOF"),
        }
    }
}

impl Token {}

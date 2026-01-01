use crate::errors::lexerror::LexError;
#[derive(Debug)]
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
    IntegerLiteral(i32),
    StringLiteral(String),
    Boolean(bool),

    //identifiers
    Identifier(String),

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
}

impl Token {
    // pub fn get_token(token_type: &str, value: Option<&str>) -> Result<Token, LexError> {
    //     //note that once unnecessary string typing is removed, this will change.
    //     match token_type {
    //         "Print" => Ok(Token::Print("print".to_string())),
    //         "If" => Ok(Token::If("if".to_string())),
    //         "Else" => Ok(Token::Else("else".to_string())),
    //         "For" => Ok(Token::For("for".to_string())),
    //         "While" => Ok(Token::While("while".to_string())),
    //         "Null" => Ok(Token::Null("null".to_string())),
    //         "And" => Ok(Token::And("and".to_string())),
    //         "Or" => Ok(Token::Or("or".to_string())),
    //         "True" => Ok(Token::Boolean(true)),
    //         "False" => Ok(Token::Boolean(false)),
    //         "Fun" => Ok(Token::Fun("fun".to_string())),
    //         "Class" => Ok(Token::Class("class".to_string())),
    //         "Super" => Ok(Token::Super("super".to_string())),
    //         "Return" => Ok(Token::Return("return".to_string())),
    //         "This" => Ok(Token::This("this".to_string())),
    //         "IntegerLiteral" => match value {
    //             Some(v) => Ok(Token::IntegerLiteral(v.parse::<i32>().unwrap())),
    //             None => Err(LexError::ValueError(token_type.to_string())),
    //         },
    //         "StringLiteral" => match value {
    //             Some(v) => Ok(Token::StringLiteral(v.to_string())),
    //             None => Err(LexError::ValueError(token_type.to_string())),
    //         },
    //         "Identifier" => match value {
    //             Some(v) => Ok(Token::Identifier(v.to_string())),
    //             None => Err(LexError::ValueError(token_type.to_string())),
    //         },
    //         "Plus" => Ok(Token::Plus("+".to_string())),
    //         "Minus" => Ok(Token::Minus("-".to_string())),
    //         "Dot" => Ok(Token::Dot(".".to_string())),
    //         "Slash" => Ok(Token::Slash("/".to_string())),
    //         "Star" => Ok(Token::Star("*".to_string())),
    //         "Assign" => Ok(Token::Assign("=".to_string())),
    //         "Semicolon" => Ok(Token::Semicolon(";".to_string())),
    //         "LeftParen" => Ok(Token::LeftParen("(".to_string())),
    //         "RightParen" => Ok(Token::RightParen(")".to_string())),
    //         "LeftBrace" => Ok(Token::RightBrace("{".to_string())),
    //         "RightBrace" => Ok(Token::LeftBrace("}".to_string())),
    //         "GreaterThan" => Ok(Token::GreaterThan(">".to_string())),
    //         "LessThan" => Ok(Token::LessThan("<".to_string())),
    //         "GreaterEqual" => Ok(Token::GreaterEqual(">=".to_string())),
    //         "LessEqual" => Ok(Token::LessEqual("<=".to_string())),
    //         "Bang" => Ok(Token::Bang("!".to_string())),
    //         "BangEqual" => Ok(Token::BangEqual("!=".to_string())),
    //         "EqualEqual" => Ok(Token::EqualEqual("==".to_string())),
    //         _ => Err(LexError::NotFound(token_type.to_string())),
    //     }
    // }

    // pub fn get_token_regex(token_type: &str) -> Result<String, LexError> {
    //     match token_type {
    //         "Print" => Ok(r"\bprint\b".to_string()),
    //         "If" => Ok(r"\bif\b".to_string()),
    //         "Else" => Ok(r"\belse\b".to_string()),
    //         "For" => Ok(r"\bfor\b".to_string()),
    //         "While" => Ok(r"\bwhile\b".to_string()),
    //         "Null" => Ok(r"\bnull\b".to_string()),
    //         "And" => Ok(r"\band\b".to_string()),
    //         "Or" => Ok(r"\bor\b".to_string()),
    //         "Boolean" => Ok(r"\b(true|false)\b".to_string()),
    //         "Fun" => Ok(r"\bfun\b".to_string()),
    //         "Class" => Ok(r"\bclass\b".to_string()),
    //         "Super" => Ok(r"\bsuper\b".to_string()),
    //         "Return" => Ok(r"\breturn\b".to_string()),
    //         "This" => Ok(r"\bthis\b".to_string()),
    //         "IntegerLiteral" => Ok(r"\d+".to_string()),
    //         "StringLiteral" => Ok(r#""([^"\\]|\\.)*""#.to_string()),
    //         "Identifier" => Ok(r"[a-zA-Z_][a-zA-Z0-9_]*".to_string()),
    //         "Plus" => Ok(r"\+".to_string()),
    //         "Minus" => Ok(r"\-".to_string()),
    //         "Dot" => Ok(r"\.".to_string()),
    //         "Slash" => Ok(r"\/".to_string()),
    //         "Star" => Ok(r"\*".to_string()),
    //         "Assign" => Ok(r"=".to_string()),
    //         "Semicolon" => Ok(r";".to_string()),
    //         "LeftParen" => Ok(r"\(".to_string()),
    //         "RightParen" => Ok(r"\)".to_string()),
    //         "LeftBrace" => Ok(r"\{".to_string()),
    //         "RightBrace" => Ok(r"\}".to_string()),
    //         "GreaterThan" => Ok(r">".to_string()),
    //         "LessThan" => Ok(r"<".to_string()),
    //         "GreaterEqual" => Ok(r">=".to_string()),
    //         "LessEqual" => Ok(r"<=".to_string()),
    //         "Bang" => Ok(r"\!".to_string()),
    //         "BangEqual" => Ok(r"\!=".to_string()),
    //         "EqualEqual" => Ok(r"==".to_string()),
    //         _ => Err(LexError::NotFound(token_type.to_string())),
    //     }
    // }
}

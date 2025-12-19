use crate::errors::lexerror::LexError;

#[derive(Debug)]
pub enum Token {
    //keywords
    Print(String),
    If(String),
    Else(String),

    //literal
    IntegerLiteral(i32),
    StringLiteral(String),

    //identifiers
    Identifier(String),

    //operators
    Plus(String),
    Minus(String),
    Dot(String),
    Slash(String),
    Star(String),
    Assign(String),

    //punctuation
    Semicolon(String),
    LeftParen(String),
    RightParen(String),
    RightBrace(String),
    LeftBrace(String),

    //logical operator
    GreaterThan(String),
    LessThan(String),
    GreaterEqual(String),
    LessEqual(String),
    Bang(String),
    BangEqual(String),
    EqualEqual(String),
}

impl Token {
    pub fn get_token(token_type: &str, value: Option<&str>) -> Token {
        //note that once unnecessary string typing is removed, this will change.
        match token_type {
            "Print" => Token::Print("print".to_string()),
            "If" => Token::If("if".to_string()),
            "Else" => Token::Else("else".to_string()),
            "IntegerLiteral" => Token::IntegerLiteral(match value {
                Some(v) => v.parse::<i32>().unwrap(),
                None => panic!("IntegerLiteral missing value"),
            }),
            "StringLiteral" => Token::StringLiteral(match value {
                Some(v) => v.to_string(),
                None => panic!("StringLiteral missing value"),
            }),
            "Identifier" => Token::Identifier(match value {
                Some(v) => v.to_string(),
                None => panic!("Identifier missing value"),
            }),
            "Plus" => Token::Plus("+".to_string()),
            "Minus" => Token::Minus("-".to_string()),
            "Dot" => Token::Dot(".".to_string()),
            "Slash" => Token::Slash("/".to_string()),
            "Star" => Token::Star("*".to_string()),
            "Assign" => Token::Assign("=".to_string()),
            "Semicolon" => Token::Semicolon(";".to_string()),
            "LeftParen" => Token::LeftParen("(".to_string()),
            "RightParen" => Token::RightParen(")".to_string()),
            "LeftBrace" => Token::RightBrace("{".to_string()),
            "RightBrace" => Token::LeftBrace("}".to_string()),
            "GreaterThan" => Token::GreaterThan(">".to_string()),
            "LessThan" => Token::LessThan("<".to_string()),
            "GreaterEqual" => Token::GreaterEqual(">=".to_string()),
            "LessEqual" => Token::LessEqual("<=".to_string()),
            "Bang" => Token::Bang("!".to_string()),
            "BangEqual" => Token::BangEqual("!=".to_string()),
            "EqualEqual" => Token::EqualEqual("==".to_string()),
            _ => LexError(token_type),
        }
    }

    pub fn get_token_regex(token_type: &str) -> String {
        match token_type {
            "Print" => r"print",
            "If" => r"if",
            "Else" => r"else",
            "IntegerLiteral" => r"\d+",
            "StringLiteral" => r#"\".*\""#,
            "Identifier" => r"[a-zA-Z_][a-zA-Z0-9_]* =",
            "Plus" => r"\+",
            "Minus" => r"\-",
            "Dot" => r"\.",
            "Slash" => r"\/",
            "Star" => r"\*",
            "Assign" => r"=",
            "Semicolon" => r";",
            "LeftParen" => r"\(",
            "RightParen" => r"\)",
            "LeftBrace" => r"\{",
            "RightBrace" => r"\}",
            "GreaterThan" => r">",
            "LessThan" => r"<",
            "GreaterEqual" => r"\<=",
            "LessEqual" => r"\<=",
            "Bang" => r"\!",
            "BangEqual" => r"\!=",
            "EqualEqual" => r"==",
            _ => LexError(token_type),
        }
        .to_string()
    }
}

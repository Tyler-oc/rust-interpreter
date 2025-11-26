//come back later, add more types, remove unnecessary string typing

#[derive(Debug)]
pub enum Token {
    //keywords
    Print(String),
    Int(String),
    If(String),
    Else(String),

    //literal
    IntegerLiteral(i32),
    StringLiteral(String),

    //identifiers
    Identifier(String),

    //operators
    Plus(String),
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
}

impl Token {
    pub fn get_token(token_type: &str, value: Option<&str>) -> Token {
        //note that once unnecessary string typing is removed, this will change.
        match token_type {
            "Print" => Token::Print("print".to_string()),
            "Int" => Token::Int("int".to_string()),
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
            "Assign" => Token::Assign("=".to_string()),
            "Semicolon" => Token::Semicolon(";".to_string()),
            "LeftParen" => Token::LeftParen("(".to_string()),
            "RightParen" => Token::RightParen(")".to_string()),
            "LeftBrace" => Token::RightBrace("{".to_string()),
            "RightBrace" => Token::LeftBrace("}".to_string()),
            "GreaterThan" => Token::GreaterThan(">".to_string()),
            "LessThan" => Token::LessThan("<".to_string()),
            _ => panic!("Token does not match a defined type: Token {}", token_type),
        }
    }

    pub fn get_token_regex(token_type: &str) -> String {
        match token_type {
            "Print" => r"print",
            "Int" => r"int",
            "If" => r"if",
            "Else" => r"else",
            "IntegerLiteral" => r"\d+",
            "StringLiteral" => r#"\".*\""#,
            "Identifier" => r"[a-zA-Z_][a-zA-Z0-9_]* =",
            "Plus" => r"+",
            "Assign" => r"=",
            "Semicolon" => r";",
            "LeftParen" => r"(",
            "RightParen" => r")",
            "LeftBrace" => r"{",
            "RightBrace" => r"}",
            "GreaterThan" => r">",
            "LessThan" => r"<",
            _ => panic!("Token does not match defined type: Token {}", token_type),
        }
        .to_string()
    }
}

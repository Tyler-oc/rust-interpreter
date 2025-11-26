pub enum token {
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

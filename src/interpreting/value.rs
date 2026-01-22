#[derive(Debug)]
pub enum Value {
    Int(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl Value {}

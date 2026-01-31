use std::collections::HashMap;

use crate::{errors::environment_error::EnvironmentError, interpreting::value::Value};

pub(crate) struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    //current decision is to error when redefining a variable with var keyword
    pub fn define(&mut self, name: String, val: Value) -> Result<(), EnvironmentError> {
        match self.values.get(&name) {
            None => {
                self.values.insert(name, val);
                return Ok(());
            }
            Some(_) => Err(EnvironmentError::MultipleAssignmentVariable(name)),
        }
    }

    pub fn get(&mut self, name: String) -> Result<Value, EnvironmentError> {
        match self.values.get(&name) {
            Some(v) => Ok(v.clone()),
            None => Err(EnvironmentError::UndefinedVariable(name)),
        }
    }
}

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{WrappedEnv, errors::environment_error::EnvironmentError, interpreting::value::Value};
#[derive(Clone, Debug)]
pub(crate) struct Environment {
    pub values: HashMap<String, Value>,
    pub outer_environment: Option<WrappedEnv>,
}

impl Environment {
    pub fn new(outer_environment: Option<WrappedEnv>) -> Self {
        Environment {
            values: HashMap::new(),
            outer_environment: outer_environment,
        }
    }

    pub fn new_all_initialized(
        values: HashMap<String, Value>,
        outer_environment: Option<WrappedEnv>,
    ) -> Self {
        Environment {
            values: values,
            outer_environment: outer_environment,
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

    pub fn get(&mut self, name: &str) -> Result<Value, EnvironmentError> {
        if let Some(stored_val) = self.values.get(name) {
            return Ok(stored_val.clone());
        }
        if let Some(ref env) = self.outer_environment {
            return env.borrow_mut().get(name);
        }
        Err(EnvironmentError::UndefinedVariable(name.to_string()))
    }

    pub fn assign(&mut self, name: &str, val: Value) -> Result<(), EnvironmentError> {
        if let Some(stored_val) = self.values.get_mut(name) {
            *stored_val = val;
            return Ok(());
        }
        if let Some(ref env) = self.outer_environment {
            return env.borrow_mut().assign(name, val);
        }
        Err(EnvironmentError::UndefinedVariable(name.to_string()))
    }
}

use crate::{
    errors::resolver_error::ResolverError, interpreting::interpreter::Interpreter,
    parsing::ast::Stmt,
};

struct Resolver {
    interpreter: Interpreter,
}

impl Resolver {
    pub fn new(&mut self, interpreter: Interpreter) -> Self {
        Resolver {
            interpreter: interpreter,
        }
    }

    fn begin_scope(&mut self) -> Result<(), ResolverError> {
        Ok(())
    }

    fn resolve(&mut self) -> Result<(), ResolverError> {
        Ok(())
    }

    fn end_scope(&mut self) -> Result<(), ResolverError> {
        Ok(())
    }

    pub fn resolve_block(&mut self, block: &Stmt) -> Result<(), ResolverError> {
        self.begin_scope()?;
        self.resolve()?;
        self.end_scope()?;
        Ok(())
    }
}

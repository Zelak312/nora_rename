use std::fmt::Debug;

use crate::{
    errors::{BasicError, Error},
    interpreter::Interpreter,
};

#[derive(PartialEq, Clone)]
pub enum ExecutableNodeReturn {
    String(String),
    Number(f64),
}

impl ExecutableNodeReturn {
    pub fn string(self) -> Result<String, Box<dyn Error>> {
        if let ExecutableNodeReturn::String(s) = self {
            return Ok(s);
        }

        Err(BasicError::new("ssheesh".to_owned()))
    }

    // pub fn number(self) -> Result<f64, Box<dyn Error>> {
    //     if let ExecutableNodeReturn::Number(s) = self {
    //         return Ok(s);
    //     }

    //     Err(BasicError::new("ssheesh".to_owned()))
    // }

    pub fn number_or_string(self) -> Result<f64, Box<dyn Error>> {
        match self {
            ExecutableNodeReturn::Number(n) => Ok(n),
            ExecutableNodeReturn::String(n) => match n.parse::<f64>() {
                Ok(s) => Ok(s),
                _ => Err(BasicError::new("dwjdi".to_owned())),
            },
        }
    }
}

pub trait ExecutableNode: Debug {
    fn execute(&self, _: &mut Interpreter) -> Result<ExecutableNodeReturn, Box<dyn Error>>;
}

use std::fmt::Debug;

use crate::{
    errors::{BasicError, Error},
    interpreter::Interpreter,
};

#[derive(PartialEq, Clone)]
pub enum ExecutableNodeReturn {
    String(String),
    Number(f64),
    Bool(bool),
}

impl ExecutableNodeReturn {
    pub fn string(&self) -> Result<String, Box<dyn Error>> {
        if let ExecutableNodeReturn::String(s) = self {
            return Ok(s.clone());
        }

        Err(BasicError::new("ssheesh".to_owned()))
    }

    // pub fn number(&self) -> Result<f64, Box<dyn Error>> {
    //     if let ExecutableNodeReturn::Number(s) = self {
    //         return Ok(s.clone());
    //     }

    //     Err(BasicError::new("ssheesh".to_owned()))
    // }

    pub fn bool(&self) -> Result<bool, Box<dyn Error>> {
        if let ExecutableNodeReturn::Bool(s) = self {
            return Ok(s.clone());
        }

        Err(BasicError::new("ssheesh".to_owned()))
    }

    pub fn to_number(&self) -> Result<f64, Box<dyn Error>> {
        match self {
            ExecutableNodeReturn::Number(n) => Ok(n.clone()),
            ExecutableNodeReturn::String(n) => match n.parse::<f64>() {
                Ok(s) => Ok(s),
                _ => Err(BasicError::new("dwjdi".to_owned())),
            },
            ExecutableNodeReturn::Bool(n) => match n {
                true => Ok(1.0),
                false => Ok(0.0),
            },
        }
    }

    pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
        match self {
            ExecutableNodeReturn::Number(n) => Ok(n.to_string()),
            ExecutableNodeReturn::String(n) => Ok(n.clone()),
            ExecutableNodeReturn::Bool(n) => match n {
                true => Ok(String::from("true")),
                false => Ok(String::from("false")),
            },
        }
    }

    pub fn eqq(&self, r: ExecutableNodeReturn) -> Result<ExecutableNodeReturn, Box<dyn Error>> {
        Ok(ExecutableNodeReturn::Bool(match self {
            ExecutableNodeReturn::Number(n) => self.execute_condition(n, &r.to_number()?),
            ExecutableNodeReturn::String(n) => self.execute_condition(n, &r.to_string()?),
            ExecutableNodeReturn::Bool(n) => self.execute_condition(n, &r.bool()?),
        }))
    }

    pub fn execute_condition<T>(&self, l: T, r: T) -> bool
    where
        T: PartialEq,
    {
        l.eq(&r)
    }
}

pub trait ExecutableNode: Debug {
    fn execute(&self, _: &mut Interpreter) -> Result<ExecutableNodeReturn, Box<dyn Error>>;
}

use std::{collections::HashMap, rc::Rc};

use regex::{CaptureNames, Captures};

use crate::errors::BasicError;
use crate::lib::types::boolean::NBoolean;
use crate::lib::types::number::NNumber;
use crate::lib::types::string::NString;
use crate::utils::equal_utils;
use crate::{errors::Error, lib::object_type::ObjectType, tokenizer::token::TokenType};

use super::nodes;

pub struct Interpreter {
    scope: HashMap<String, String>,
    count: i32,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            scope: HashMap::new(),
            count: 0,
        }
    }

    fn insert_special_vars(&mut self) {
        self.scope
            .insert(String::from("#count"), self.count.to_string());
    }

    fn insert_captures(&mut self, captures: &Captures, names: CaptureNames) {
        for name in names {
            if let Some(m) = name {
                let cap = captures.name(m);
                if cap.is_none() {
                    continue;
                }

                self.scope
                    .insert(m.to_owned(), cap.unwrap().as_str().to_owned());
            }
        }

        for i in 0..captures.len() {
            if let Some(cap) = captures.get(i) {
                self.scope
                    .insert(String::from("#") + &i.to_string(), cap.as_str().to_string());
            }
        }
    }

    pub fn execute(
        &mut self,
        captures: &Captures,
        names: CaptureNames,
        node: Rc<dyn nodes::ExecutableNode>,
    ) -> Result<ObjectType, Box<dyn Error>> {
        self.insert_special_vars();
        self.insert_captures(captures, names);
        let res = node.execute(self);
        self.count += 1;
        res
    }
}

impl nodes::ExecutableNode for nodes::NodeBinaryOperator {
    fn execute(&self, interpreter: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        let left_o = self.left.execute(interpreter)?;
        match left_o {
            ObjectType::NNumber(n) => {
                let rigth = self.right.execute(interpreter)?.into_number()?;
                let inner_value = match self.operator {
                    TokenType::Addition => n.inner_value + rigth.inner_value,
                    TokenType::Subtraction => n.inner_value - rigth.inner_value,
                    TokenType::Multiplication => n.inner_value * rigth.inner_value,
                    TokenType::Division => n.inner_value / rigth.inner_value,
                    TokenType::Power => n.inner_value.powf(rigth.inner_value),
                    TokenType::Log => n.inner_value.log(rigth.inner_value),
                    _ => panic!("Operation not found for number"),
                };

                return Ok(ObjectType::NNumber(NNumber { inner_value }));
            }
            ObjectType::NString(n) => {
                let rigth = self.right.execute(interpreter)?.into_string()?;
                let inner_value = match self.operator {
                    TokenType::Addition => n.inner_value + &rigth.inner_value,
                    _ => panic!("Operation not found for string"),
                };

                return Ok(ObjectType::NString(NString { inner_value }));
            }
            _ => panic!("Cannot do binary operation on this type"),
        }
    }
}

impl nodes::ExecutableNode for nodes::NodeBlock {
    fn execute(&self, interpreter: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        let mut inner_value = self
            .content
            .execute(interpreter)?
            .into_string()?
            .inner_value;
        if let Some(node) = &self.next {
            inner_value += &node.execute(interpreter)?.into_string()?.inner_value;
        }

        Ok(ObjectType::NString(NString { inner_value }))
    }
}

impl nodes::ExecutableNode for nodes::NodeContent {
    fn execute(&self, interpreter: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        let mut inner_value = String::from(&self.content);
        if let Some(node) = &self.next {
            inner_value += &node.execute(interpreter)?.into_string()?.inner_value;
        }

        Ok(ObjectType::NString(NString { inner_value }))
    }
}

impl nodes::ExecutableNode for nodes::NodeIdentifer {
    fn execute(&self, i: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        // TODO: should make this a linePointingError
        // Need to implement nodes start and length for this to happen
        let capture = i.scope.get(&self.content).ok_or(BasicError::new(format!(
            "Couldn't find variable: {}",
            &self.content
        )))?;
        Ok(ObjectType::NString(NString {
            inner_value: capture.to_owned(),
        }))
    }
}

impl nodes::ExecutableNode for nodes::NodeString {
    fn execute(&self, _: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        Ok(ObjectType::NString(NString {
            inner_value: self.content.clone(),
        }))
    }
}

impl nodes::ExecutableNode for nodes::NodeNumber {
    fn execute(&self, _: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        Ok(ObjectType::NNumber(NNumber {
            inner_value: self.content.clone(),
        }))
    }
}

impl nodes::ExecutableNode for nodes::NodeCondition {
    fn execute(&self, i: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        let inner_value = match self.left.execute(i)? {
            ObjectType::NBoolean(n) => {
                equal_utils::partial_eq(&self.operator, n, self.right.execute(i)?.into_boolean()?)?
            }
            ObjectType::NString(n) => {
                equal_utils::partial_eq(&self.operator, n, self.right.execute(i)?.into_string()?)?
            }
            ObjectType::NNumber(n) => {
                equal_utils::partial_ord(&self.operator, n, self.right.execute(i)?.into_number()?)?
            }
        };

        Ok(ObjectType::NBoolean(NBoolean { inner_value }))
    }
}
impl nodes::ExecutableNode for nodes::NodeTernary {
    fn execute(&self, i: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        let cond = self.condition.execute(i)?.into_boolean()?;
        if cond.inner_value {
            return self.left.execute(i);
        }

        return self.right.execute(i);
    }
}

impl nodes::ExecutableNode for nodes::NodeKeyword {
    fn execute(&self, i: &mut Interpreter) -> Result<ObjectType, Box<dyn Error>> {
        Ok(match self.keyword {
            TokenType::KeyNumber => ObjectType::NNumber(self.content.execute(i)?.into_number()?),
            TokenType::KeyString => ObjectType::NString(self.content.execute(i)?.into_string()?),
            _ => panic!("djijdiw"),
        })
    }
}
